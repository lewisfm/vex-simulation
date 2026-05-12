//! Display renderer implementation which writes data to a GUI window.

use std::{
    num::NonZeroU32,
    rc::Rc,
    time::{Duration, Instant},
};

use anyhow::{Context as _, Result, anyhow};
use fast_image_resize::{
    ResizeAlg, ResizeOptions, Resizer,
    images::{TypedImage, TypedImageRef},
    pixels::U8x4,
};
use roboscope_ipc::{
    Config, Publisher, Sample, SimServices, Subscriber,
    display::{DISPLAY_HEIGHT, DISPLAY_WIDTH, DisplayFrame, DisplayInput, DisplayInputKind},
};
use softbuffer::{Context, Surface};
use tracing::{debug, error, trace};
use winit::{
    application::ApplicationHandler,
    dpi::LogicalSize,
    event::{MouseButton, StartCause, WindowEvent},
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop, OwnedDisplayHandle},
    window::{Theme, Window, WindowId},
};

#[cfg(target_os = "macos")]
mod macos;

const WINDOW_SIZE: LogicalSize<f64> = LogicalSize::new(480.0, 272.0);

type DisplayCtx = Context<OwnedDisplayHandle>;

enum ViewerEvent {
    Shutdown,
}

fn main() -> Result<()> {
    ViewerApp::start()
}

pub struct ViewerApp {
    sim_display: Option<SimDisplayWindow>,
    context: DisplayCtx,
    last_frame_time: Option<Instant>,
    subscriber: Option<Subscriber<DisplayFrame>>,
    publisher: Option<Publisher<DisplayInput>>,
}

impl ViewerApp {
    pub fn start() -> Result<()> {
        let ipc = SimServices::join(Some("viewer"), &Config::default())?;
        let subscriber = ipc.display_frames()?.subscriber_builder().create()?;
        let publisher = ipc.display_input()?.publisher_builder().create()?;

        let event_loop = EventLoop::<ViewerEvent>::with_user_event().build().unwrap();
        let proxy = event_loop.create_proxy();
        ctrlc::set_handler(move || {
            let _ = proxy.send_event(ViewerEvent::Shutdown);
        })
        .context("Failed to register Ctrl-C handler")?;

        let display = event_loop.owned_display_handle();
        let mut simulator = ViewerApp::new(display, subscriber, publisher)?;
        event_loop.run_app(&mut simulator)?;

        Ok(())
    }

    fn new(
        display: OwnedDisplayHandle,
        subscriber: Subscriber<DisplayFrame>,
        publisher: Publisher<DisplayInput>,
    ) -> Result<Self> {
        let context = DisplayCtx::new(display)
            .map_err(|e| anyhow!(e.to_string()))
            .context("Failed to create display rendering context")?;

        Ok(Self {
            sim_display: None,
            context,
            last_frame_time: None,
            subscriber: Some(subscriber),
            publisher: Some(publisher),
        })
    }

    fn schedule_render(&mut self, event_loop: &ActiveEventLoop, last_render: Instant) {
        let frame_period = Duration::from_secs(1) / 60;
        let now = Instant::now();

        let mut next_render = last_render + frame_period;
        if next_render < now {
            next_render = now + frame_period;
        }

        event_loop.set_control_flow(ControlFlow::WaitUntil(next_render));
    }
}

impl ApplicationHandler<ViewerEvent> for ViewerApp {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.sim_display.is_none() {
            match SimDisplayWindow::open(
                event_loop,
                &self.context,
                self.subscriber.take().unwrap(),
                self.publisher.take().unwrap(),
            ) {
                Ok(sim_display) => self.sim_display = Some(sim_display),
                Err(error) => error!(%error, "Failed to open VEX V5 Display window"),
            }
        }
    }

    fn user_event(&mut self, event_loop: &ActiveEventLoop, event: ViewerEvent) {
        if matches!(event, ViewerEvent::Shutdown) {
            event_loop.exit();
        }
    }

    fn new_events(&mut self, event_loop: &ActiveEventLoop, cause: StartCause) {
        match cause {
            StartCause::Init => {
                // Start a timer for rendering the display at 60 fps.
                self.schedule_render(event_loop, Instant::now());
            }
            StartCause::ResumeTimeReached {
                requested_resume, ..
            } => {
                // 60Hz render timer has triggered, so render a frame.
                self.schedule_render(event_loop, requested_resume);

                let now = Instant::now();
                if let Some(last) = self.last_frame_time.replace(now) {
                    trace!(measured_period = ?now - last, "Frame time");
                }

                if let Some(d) = &mut self.sim_display {
                    d.queue_redraw();
                }
            }
            _ => {}
        }
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: WindowId,
        event: WindowEvent,
    ) {
        if let Some(sim_display) = &mut self.sim_display
            && window_id == sim_display.window_id()
        {
            sim_display.handle_event(event_loop, event);
        }
    }
}

/// A simulated VEX V5 display.
pub struct SimDisplayWindow {
    window: Rc<Window>,
    surface: Surface<OwnedDisplayHandle, Rc<Window>>,
    subscriber: Subscriber<DisplayFrame>,
    publisher: Publisher<DisplayInput>,
    last_frame: Option<Sample<DisplayFrame>>,

    scale_factor: f64,
    num_clicks: u32,
    is_mouse_down: bool,
    mouse_coords: [i16; 2],

    // A frame has been explicitly requested by the app; the next redraw should autorender the
    // canvas, update the program header, notify vexDisplayRender callers, etc. instead of just
    // scaling the previous rendered frame.
    has_scheduled_frame: bool,
}

impl SimDisplayWindow {
    pub fn open(
        event_loop: &ActiveEventLoop,
        context: &DisplayCtx,
        subscriber: Subscriber<DisplayFrame>,
        publisher: Publisher<DisplayInput>,
    ) -> Result<Self> {
        debug!("Opening V5 display window");

        #[cfg(target_os = "macos")]
        self::macos::init_app();

        let attrs = Window::default_attributes()
            .with_resizable(false)
            .with_min_inner_size(WINDOW_SIZE)
            .with_inner_size(WINDOW_SIZE)
            .with_theme(Some(Theme::Dark))
            .with_title("VEX V5 Simulator");

        let window = Rc::new(event_loop.create_window(attrs)?);

        #[cfg(target_os = "macos")]
        {
            window.set_resizable(true);
            self::macos::notify_aspect_ratio(&window);
        }

        let surface = Surface::new(context, window.clone())
            .map_err(|e| anyhow!(e.to_string()))
            .context("Failed to create V5 display rendering surface")?;

        Ok(Self {
            surface,
            window,
            subscriber,
            publisher,
            last_frame: None,
            scale_factor: 1.0,
            is_mouse_down: false,
            mouse_coords: [0, 0],
            num_clicks: 0,
            has_scheduled_frame: true,
        })
    }

    /// Handle an event sent to this window.
    pub fn handle_event(&mut self, event_loop: &ActiveEventLoop, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                self.redraw().unwrap();
            }
            WindowEvent::MouseInput { state, button, .. } => {
                if button != MouseButton::Left {
                    return;
                }

                let release_count = self.num_clicks;
                self.is_mouse_down = state.is_pressed();
                if self.is_mouse_down {
                    self.num_clicks = self.num_clicks.wrapping_add(1);
                }

                _ = self.publisher.send_copy(DisplayInput {
                    kind: if self.is_mouse_down {
                        DisplayInputKind::Press
                    } else {
                        DisplayInputKind::Release
                    },
                    press_count: self.num_clicks,
                    release_count,
                    x: self.mouse_coords[0],
                    y: self.mouse_coords[1],
                });
            }
            WindowEvent::CursorMoved { position, .. } => {
                let x = position.x * self.scale_factor;
                let y = position.y * self.scale_factor;
                self.mouse_coords = [x as i16, y as i16];

                if self.is_mouse_down {
                    _ = self.publisher.send_copy(DisplayInput {
                        kind: if self.is_mouse_down {
                            DisplayInputKind::Hold
                        } else {
                            DisplayInputKind::Release
                        },
                        press_count: self.num_clicks,
                        release_count: self.num_clicks - 1,
                        x: self.mouse_coords[0],
                        y: self.mouse_coords[1],
                    });
                }
            }
            WindowEvent::Resized(_) => {
                // Tell the window manager that we have a certain aspect ratio set if possible.
                // This makes dragging the left side of the window resize properly instead of
                // just shifting the window to the left.
                #[cfg(target_os = "macos")]
                self::macos::notify_aspect_ratio(&self.window);

                // Maintain the proper aspect ratio.
                let dims = self.window.inner_size();
                let mut fb_dims = dims;

                let current_aspect_ratio = dims.width as f64 / dims.height as f64;
                let desired_aspect_ratio = WINDOW_SIZE.width / WINDOW_SIZE.height;

                if current_aspect_ratio > desired_aspect_ratio {
                    fb_dims.width = (desired_aspect_ratio * dims.height as f64) as u32;
                } else {
                    fb_dims.height = (1.0 / desired_aspect_ratio * dims.width as f64) as u32;
                }

                if dims != fb_dims && !self.window.is_maximized() {
                    _ = self.window.request_inner_size(fb_dims);
                }

                self.scale_factor = WINDOW_SIZE.width / fb_dims.width as f64;

                // Scale the framebuffer to the window.
                self.surface
                    .resize(
                        NonZeroU32::new(fb_dims.width).unwrap(),
                        NonZeroU32::new(fb_dims.height).unwrap(),
                    )
                    .unwrap();
            }
            _ => {}
        }
    }

    pub fn queue_redraw(&mut self) {
        self.has_scheduled_frame = true;
        self.window.request_redraw();
    }

    pub fn window_id(&self) -> WindowId {
        self.window.id()
    }

    /// Scale the display's contents to the size of the window, then write them to the framebuffer.
    pub fn redraw(&mut self) -> Result<()> {
        let next_frame = self.subscriber.receive()?;
        let Some(frame) = next_frame.as_ref().or(self.last_frame.as_ref()) else {
            return Ok(());
        };

        let mut window_buffer = self.surface.buffer_mut().unwrap();
        let width = window_buffer.width().get();
        let height = window_buffer.height().get();

        // Scale the contents to the window size so the entire thing is filled.
        // The destination of the scaled image is the framebuffer itself.

        let buffer_pixels: &[U8x4] = bytemuck::must_cast_slice(&frame.buffer);
        let window_pixels: &mut [U8x4] = bytemuck::must_cast_slice_mut(&mut window_buffer);

        let frame_image = TypedImageRef::new(DISPLAY_WIDTH, DISPLAY_HEIGHT, buffer_pixels).unwrap();
        let mut window_image = TypedImage::from_pixels_slice(width, height, window_pixels).unwrap();

        let mut resizer = Resizer::new();
        resizer
            .resize_typed::<U8x4>(
                &frame_image,
                &mut window_image,
                &ResizeOptions::new()
                    .resize_alg(ResizeAlg::Nearest)
                    .use_alpha(false),
            )
            .unwrap();

        // Swap buffers.
        self.window.pre_present_notify();
        window_buffer.present().unwrap();

        if next_frame.is_some() {
            self.last_frame = next_frame;
        }

        Ok(())
    }
}
