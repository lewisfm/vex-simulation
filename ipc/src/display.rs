use std::{sync::LazyLock, time::Duration};

use iceoryx2::prelude::ZeroCopySend;

pub static DISPLAY_UPDATE_PERIOD: LazyLock<Duration> =
    LazyLock::new(|| Duration::from_secs_f64(1.0 / 60.0));
pub const DISPLAY_WIDTH: u32 = 480;
pub const DISPLAY_HEIGHT: u32 = 272;
pub const DISPLAY_BUF_SIZE: usize = DISPLAY_WIDTH as usize * DISPLAY_HEIGHT as usize;


#[derive(derive_more::Debug, ZeroCopySend, PartialEq, Eq, Clone)]
#[repr(C)]
pub struct DisplayFrame {
    #[debug("..")]
    pub buffer: [u32; DISPLAY_BUF_SIZE],
}


/// A touch event, with coordinates.
///
/// Note that it is considered valid for touch coordinates to be out of range of the display's
/// bounds.
#[derive(Debug, ZeroCopySend, PartialEq, Eq, Clone, Copy)]
#[repr(C)]
pub struct DisplayInput {
    pub kind: DisplayInputKind,
    pub x: i16,
    pub y: i16,
    pub press_count: u32,
    pub release_count: u32,
}


#[derive(Debug, ZeroCopySend, PartialEq, Eq, Clone, Copy)]
#[repr(C)]
pub enum DisplayInputKind {
    Press,
    Hold,
    Release,
}
