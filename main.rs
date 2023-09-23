


use std::default;
use bevy::render::render_resource::PrimitiveTopology;
use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
#[feature(default_fn)]
use bevy::window::PrimaryWindow;

use bevy::app::App;
use bevy::DefaultPlugins;
use bevy_prototype_debug_lines::*;
fn main() 
{
    App::new()

        .add_plugins(DefaultPlugins)
        .add_plugins(DebugLinesPlugin::default())
        //.add_systems(Startup, setup)
        .add_systems(Startup, camera_setup)
        .add_systems(Update, cursor_position)
        
        .run();
}
fn camera_setup(mut commands: Commands)
{
    commands.spawn(Camera2dBundle::default());
}
fn cursor_position(
    q_windows: Query<&Window, With<PrimaryWindow>>, 
    mut lines: ResMut<DebugLines>,
) 
{
    let start = Vec3::splat(0.);
    let mut end = Vec3::splat(0.);
    if let Some(position) = q_windows.single().cursor_position() {
        end = Vec3::new(position.x,position.y,0.);
        println!("Cursor is inside the primary window, at {:?}", position);
    } else {
        //println!("Cursor is not in the game window.");
    }
    lines.line(start, end, 0.);
}
