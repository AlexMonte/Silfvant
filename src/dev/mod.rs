use bevy::prelude::*;
use bevy_gizmo::GizmoPlugin;

pub(crate) struct DevPlugin {}

impl Plugin for DevPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_plugin(GizmoPlugin);
    }
}
