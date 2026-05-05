# RoboScope - VEX V5 Simulation

RoboScope is an enhanced desktop SDK for vexide with features like display simulation and the ability to connect to a physics engine. The end-goal for this project is to provide a way for users to design their robot code and train their drivers even when access to a physical robot is limited. This project is developed using only publicly-available or measurable information about the behavior of VEX products.

![Display simulator](./assets/display.gif)

## Getting started

Add vex-sdk-simulator as a dependency and change your program entrypoint to call `vex_sdk_simulator::run_simulator`.

Then `cargo run`. If you would like to see display output:

```sh
cargo install --git https://github.com/lewisfm/vex-simulation roboscope-viewer
```

Then run `roboscope-viewer` while your program is running.

## Projects

- [Brain Simulator] aka `vex-sdk-desktop`: Drop-in replacement SDK library which provides desktop
    implementations of functions normally only available via VEXos V5. This lets you run unmodified
    robot code like an auton selector or drive program without a V5 brain. Simulation events and
    status updates are published via shared memory for other processes to use.
- [Display Viewer]: A fairly simple app that connects to an active brain simulator and shows the
    current image on the brain's display.
- [IPC library] aka `roboscope-ipc`: Packet definition library for publishing and subscribing to
    simulator data. This could be used to implement a custom simulator visualizer or physics engine.

There are also some example projects under `examples/` which you can try simulating (the GIF above
is the `display` example).

[Brain Simulator]: ./simulator
[Display Viewer]: ./viewer
[IPC Library]: ./ipc

## Planned

I'm working on wiring up the device APIs to the IPC library so you can send simulated sensor
readings to your brain simulator and control simulated motors.

If this project pans out I'm also planning to get this integrated into vexide to make it more plug
and play.

## Usage

Run an example V5 program:

```sh
cargo run --example display
```

At the same time, you can run one of these programs to dynamically add features to the simulation:

View the display:

```sh
cargo run -p roboscope-viewer -r
```

Start a physics server for a V5 program to use (this minimal example will connect a distance sensor
on port 1 and oscillate it back and forth):

```sh
cargo run -p roboscope-ipc --example oscillator
```

### Builtin Display mode

As a convenience, you can completely disable IPC support and instead open the display window directly from your V5 simulator process.

```sh
cargo run --example display -F vex-sdk-desktop/windowed
```

## Troubleshooting

If you get a stack overflow, make sure that your display drawing code is not allocating any large arrays on the stack. The VEX V5 has a very large stack compared to most operating systems.

