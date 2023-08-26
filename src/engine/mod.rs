use bevy::prelude::*;

pub(crate) mod bevy_config;
pub(crate) mod types;

pub mod fs;
pub mod ui;
pub mod world;

/// This is a trait that allows adding functions as plugins to an App
pub trait FnPluginExt {
    /// This is the extension function
    fn add_fn_plugin(&mut self, f: impl FnOnce(&mut Self)) -> &mut Self;
}

/// Implemented for `App` to allow using Rust functions in place of Bevy plugins without sacrificing the builder pattern
impl FnPluginExt for App {
    fn add_fn_plugin(&mut self, f: impl FnOnce(&mut Self)) -> &mut Self {
        let mut app = std::mem::take(self);
        f(&mut app);
        *self = app;
        self
    }
}
