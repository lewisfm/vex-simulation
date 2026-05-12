#![feature(c_variadic)]
#![deny(unsafe_op_in_unsafe_fn)]

use std::path::Path;

mod canvas;
mod config;
mod device;
mod display;
pub mod error;
mod ipc;
pub mod sdk;

/// Initialize the simulator.
///
/// This should be called before accessing SDK functions, or else they will effectively be no-ops.
pub fn init() -> anyhow::Result<()> {
    let mut args = std::env::args();
    let path = args.next().unwrap_or_else(|| "Simulator".to_string());

    let exe_name = Path::new(&path)
        .file_name()
        .and_then(|str| str.to_str())
        .unwrap_or(&path);

    ipc::start(exe_name)?;

    Ok(())
}
