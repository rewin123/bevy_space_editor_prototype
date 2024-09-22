use bevy::prelude::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        
    }
}

#[cfg(feature = "editor")]
mod editor {
    use editor_game_plugin::*;
    use super::*;
    use bevy::prelude::*;

    struct EditorPlugin;

    impl Plugin for EditorPlugin {
        fn build(&self, app: &mut App) {
            app.add_plugins(EditorGamePlugin);
            app.add_plugins(GamePlugin);
        }
    }

    #[no_mangle]
    pub extern "C" fn set_game_plugin() {
        register_plugins(|app : &mut App| {
            app.add_plugins(EditorPlugin);
        });
    }
}