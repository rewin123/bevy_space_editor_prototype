use std::{ffi::c_void, sync::{Arc, Mutex}};

use libloading::{Library, Symbol};
use bevy::{prelude::*, tasks::{block_on, AsyncComputeTaskPool, Task}};
use std::process::Command;

#[derive(Resource)]
pub struct ProjectInfo {
    pub name: String,
    pub path: String,
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
enum GameInstanceState {
    #[default]
    Empty,
    Loading,
    Running,
    Error
}

#[derive(Resource)]
pub struct GameInstance {
    pub library: Library,
}

impl GameInstance {
    pub fn new(library: Library) -> Self {
        Self {
            library
        }
    }

    pub fn create_game_app(&self) {
        unsafe {
            let create_game_app = self.library.get::<unsafe extern "C" fn()>(b"create_game_app\0").unwrap();
            create_game_app();
        }
    }

    pub fn init_plugins(&self) {
        unsafe {
            let init_plugins = self.library.get::<unsafe extern "C" fn()>(b"init_plugins\0").unwrap();
            init_plugins();
        }
    }

    pub fn update_app(&self) {
        unsafe {
            let update_app = self.library.get::<unsafe extern "C" fn()>(b"update_app\0").unwrap();
            update_app();
        }
    }

    pub fn destroy_app(&self) {
        unsafe {
            let destroy_app = self.library.get::<unsafe extern "C" fn()>(b"destroy_app\0").unwrap();
            destroy_app();
        }
    }

    pub fn get_app<'a>(&'a mut self) -> &'a mut App {
        unsafe {
            let get_app = self.library.get::<unsafe extern "C" fn() -> *mut c_void>(b"get_app\0").unwrap();
            let app_ptr = get_app();
            let app = &mut *(app_ptr as *mut App);
            app
        }
    }

    pub fn get_world<'a>(&'a mut self) -> &'a mut World {
        unsafe {
            let get_world = self.library.get::<unsafe extern "C" fn() -> *mut c_void>(b"get_world\0").unwrap();
            let world_ptr = get_world();
            let world = &mut *(world_ptr as *mut World);
            world
        }
    }
}

impl Drop for GameInstance {
    fn drop(&mut self) {
        self.destroy_app();
    }
}

pub struct ProjectPlugin;

impl Plugin for ProjectPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameInstanceState>();
        app.init_resource::<GameInstanceLoadTask>();

        app.add_event::<LoadGameInstance>();

        app.add_systems(Update, load_game_instance.run_if(resource_exists::<ProjectInfo>));
        app.add_systems(Update, check_loading);

        app.add_systems(Startup, test_startup);

        app.add_systems(OnEnter(GameInstanceState::Running), init_game_app);
    }
}

fn test_startup(
    mut commands: Commands,
    mut events: EventWriter<LoadGameInstance>,
) {
    commands.insert_resource(ProjectInfo {
        name: "test_game".to_string(),
        path: "examples/test_game".to_string(),
    });

    events.send(LoadGameInstance);
}

fn init_game_app(
    mut game_instance: ResMut<GameInstance>,
) {
    game_instance.create_game_app();
    info!("Game Instance Runned");
}

#[derive(Event)]
pub struct LoadGameInstance;

#[derive(Resource, Default)]
pub struct GameInstanceLoadTask (Option<Task<Result<Library, String>>>);

fn load_game_instance(
    mut commands: Commands,
    mut events: EventReader<LoadGameInstance>,
    mut current_project: Res<ProjectInfo>,
    mut load_state: ResMut<NextState<GameInstanceState>>
) {
    if events.is_empty() {
        return;
    }

    events.clear();

    let proj_path = current_project.path.clone();
    let project_name = current_project.name.clone();
    let task = AsyncComputeTaskPool::get()
    .spawn(async move {

        //compile game in "library"
        let mut command = Command::new("cargo");
        command.args(["build", "--lib", "--features", "editor", "--target-dir", "target"]);

        if let Err(e) = command.current_dir(proj_path.clone()).spawn() {
            error!("Failed to compile game: {}", e);
            return Err("Failed to compile game: ".to_string() + e.to_string().as_str());
        }

        let lib_path = proj_path + "/target/debug/" + project_name.as_str() + ".dll";
        let lib = unsafe { Library::new(&lib_path).unwrap() };
        Ok(lib)
    });

    commands.insert_resource(GameInstanceLoadTask(Some(task)));
    load_state.set(GameInstanceState::Loading);
}

fn check_loading(
    mut commands: Commands,
    mut loading_task: ResMut<GameInstanceLoadTask>,
    mut load_state : ResMut<NextState<GameInstanceState>>,
) {
    if loading_task.0.is_some() {
        if loading_task.0.as_mut().map(|t| t.is_finished()).unwrap_or_default() {
            let task = loading_task.0.take().unwrap();
            let library = block_on(
                async {
                    task.cancel().await
            });

            let Some(library) = library else {
                error!("Can not load dll library of game. Undefined error");
                load_state.set(GameInstanceState::Error);
                return;
            };

            match library {
                Ok(library) => {
                    commands.insert_resource(GameInstance::new(library));
                    info!("Game Instance Loaded");
                    load_state.set(GameInstanceState::Running);
                },
                Err(err) => {
                    error!("Can not load dll library of game: {}", err);
                    load_state.set(GameInstanceState::Error);

                },
            }
        }
    }
}