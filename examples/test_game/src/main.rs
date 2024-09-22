use bevy::prelude::*;


#[derive(Component, Reflect)]
pub struct PlayerSprite; 

fn spawn_player_sprire(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    q_players: Query<Entity, Added<PlayerSprite>>) {

    for entity in q_players.iter() {
        commands.spawn(SpriteBundle {
            texture: asset_server.load("icon.png"),
            ..default()
        });
    }

}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .register_type::<PlayerSprite>()
        .add_systems(Update, spawn_player_sprire)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn(PlayerSprite);
}    
