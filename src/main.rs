


use std::default;
use bevy::render::render_resource::PrimitiveTopology;
use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
#[feature(default_fn)]
use bevy::window::PrimaryWindow;

use bevy::app::App;
use bevy::DefaultPlugins;
use bevy_prototype_debug_lines::*;
mod angle_solver;
fn main() 
{
    let mut testarm = angle_solver::Arm::new(Vec::from([200.0,200.0,200.0,200.0]));
    App::new()
        .insert_resource(Msaa::default())
        .add_plugins(DefaultPlugins)
        .add_plugins(DebugLinesPlugin::default())
        .add_systems(Startup, camera_setup)
        .add_systems(Update, cursor_position)        
        .run();
    
}
fn camera_setup(mut commands: Commands)
{
    commands.spawn(Camera2dBundle::default());
}
fn cursor_position(
    windows: Query<&Window, With<PrimaryWindow>>, 
    mut lines: ResMut<DebugLines>,
) 
{
    let mut testarm = angle_solver::Arm::new(Vec::from([400.0,200.0,200.0,200.0]));
    let start = Vec3::splat(0.);
    let mut end = Vec3::splat(0.);
    if let Some(position) = windows.single().cursor_position() 
    {
        end = Vec3::new(position.x-(windows.single().width())/2.,(windows.single().height()/2.)-position.y,0.);
    } 
    else 
    {
        //println!("Cursor is not in the game window.");
    }
    
    //println!("{} is the angle at distance of {}",angle_solver::Arm::solver(&testarm,(end.x.powf(2.0)+end.y.powf(2.0)).sqrt() as f64),(end.x.powf(2.0)+end.y.powf(2.0)).sqrt() as f64);
    let mut angle=angle_solver::Arm::solver(&testarm,(end.x.powf(2.0)+end.y.powf(2.0)).sqrt().into());
    let mut previous_point = Vec3::splat(0.0);

    //println!("the final position is {}",angle_solver::Arm::final_position(&testarm,angle).0);
    let (final_x,final_y)=angle_solver::Arm::final_position(&testarm,angle);
    
    let cursor_angle = (end.x as f64/end.y as f64).atan();
    let mut kinematic_angle = (final_x/final_y).atan();
    if kinematic_angle.is_sign_positive()
    {
        kinematic_angle=3.1415926535897932384626433+kinematic_angle;
        //angle =6.28318530718-angle;
    }

    let mut starting_angle =cursor_angle-kinematic_angle;
   
    if end.y.is_sign_positive()
    {
        starting_angle=3.1415926535897932384626433+starting_angle;
        //angle =6.28318530718-angle;
    }
    //println!("the distance is {}",(end.x.powf(2.0)+end.y.powf(2.0)).sqrt());
    for i in 1..5
    {
         lines.line(previous_point, 
                    Vec3
                    {
                        x:previous_point.x+(testarm.bone_lengths[i-1] as f32*(starting_angle + (angle*(i as f64))).sin() as f32), 
                        y:previous_point.y+(testarm.bone_lengths[i-1] as f32*(starting_angle + (angle*(i as f64))).cos() as f32), 
                        z:0.0
                    }, 
                0.0);

         previous_point=Vec3
                    {
                        x:previous_point.x+(testarm.bone_lengths[i-1] as f32*(starting_angle + (angle*(i as f64))).sin() as f32), 
                        y:previous_point.y+(testarm.bone_lengths[i-1] as f32*(starting_angle + (angle*(i as f64))).cos() as f32), 
                        z:0.0
                    }; 
    }
   
}
