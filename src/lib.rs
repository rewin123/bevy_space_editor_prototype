pub mod project;

use bevy::prelude::*;

pub struct EditorPlugin;

impl Plugin for EditorPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(project::ProjectPlugin);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_add_plugin() {
        App::new().add_plugins((MinimalPlugins, EditorPlugin));
    }
}
