use bevy::prelude::*;
use std::{ffi::c_void, sync::Mutex};

pub struct EditorGamePlugin;

impl Plugin for EditorGamePlugin {
    fn build(&self, app: &mut App) {
        // For some future systems
    }
}

static mut REGISTER_PLUGINS: Mutex<Option<Box<dyn Fn(&mut App) + Send + Sync>>> = Mutex::new(None);
static mut APP: Mutex<Option<App>> = Mutex::new(None);

pub fn register_plugins(f: impl Fn(&mut App) + Send + Sync + 'static) {
    unsafe {
        *REGISTER_PLUGINS.lock().unwrap() = Some(Box::new(f));
    }
}



#[no_mangle]
pub extern "C" fn create_game_app() {
    unsafe {
        *APP.lock().unwrap() = Some(App::new());
    }
}

#[no_mangle]
pub extern "C" fn init_plugins() {
    unsafe {
        let Some(app) = &mut *APP.lock().unwrap() else {
            return;
        };
        let register_plugins = &mut *REGISTER_PLUGINS.lock().unwrap();
        if let Some(register_plugins) = register_plugins {
            register_plugins(app);
        }
    }
}

#[no_mangle]
pub extern "C" fn update_app() {
    unsafe {
        if let Some(app) = &mut *APP.lock().unwrap() {
            app.update();
        }
    }
}

#[no_mangle]
pub extern "C" fn destroy_app() {
    unsafe {
        *APP.lock().unwrap() = None;
    }
}

#[no_mangle]
pub extern "C" fn get_app() -> *mut c_void {
    unsafe {
        if let Some(app) = &mut *APP.lock().unwrap() {
            app as *mut _ as *mut c_void
        } else {
            std::ptr::null_mut()
        }
    }
}

#[no_mangle]
pub extern "C" fn get_world() -> *mut c_void {
    unsafe {
        if let Some(app) = &mut *APP.lock().unwrap() {
            app.world_mut() as *mut _ as *mut c_void
        } else {
            std::ptr::null_mut()
        }
    }
}