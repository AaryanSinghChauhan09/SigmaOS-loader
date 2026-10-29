//! SigmaOS Loader
//! System loader and boot management

pub mod loader;
pub mod launch_ready;

pub use loader::*;
pub use launch_ready::*;

#[derive(Debug, Default, Clone)]
pub struct Loader;
