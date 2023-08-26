pub(crate) mod agent;
pub(crate) mod player;
pub mod system;

pub mod coordinates;
pub mod personality;
pub mod velocity;

pub use agent::*;
pub use common::*;
pub use player::*;

pub(crate) fn entity_plugin(app: &mut App) {
    app.spawn
}
