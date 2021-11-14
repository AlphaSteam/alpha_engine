#![allow(unused_imports)]
extern crate alpha_engine;
use std::borrow::Borrow;

use alpha_engine::audio::audio_engine::algebra::{UnitQuaternion, Vector3};
use alpha_engine::audio::audio_engine::buffer::{DataSource, SoundBufferResource};
use alpha_engine::audio::audio_engine::context::SoundContext;
use alpha_engine::audio::audio_engine::engine::SoundEngine;
use alpha_engine::audio::audio_engine::futures::executor::block_on;
use alpha_engine::audio::audio_engine::hrtf::Vec3;
use alpha_engine::audio::audio_engine::math::Matrix4Ext;
use alpha_engine::audio::audio_engine::source::generic::GenericSourceBuilder;
use alpha_engine::audio::audio_engine::source::spatial::SpatialSourceBuilder;
use alpha_engine::audio::audio_engine::source::{SoundSource, Status};
use alpha_engine::event::event_manager::EventManager;
use alpha_engine::event::{self, DeviceEvent, DeviceId, KeyboardInput, VirtualKeyCode};
use alpha_engine::{engine, game, shaders::Shader, sys, text};
use engine::Engine;
use game::Game;
use sys::{
    axes::Axis, cam::projection_ortho::ProjectionOrtho,
    cam::projection_perspective::ProjectionPerspective, fullscreen::Fullscreen,
    game_object::GameObject, system::System,
};
use text::font::Font;
fn start(system: &mut System, event_manager: &mut EventManager) {
    system.set_window_fullscreen(Fullscreen::False);
    system.set_window_resolution([600, 800]);
    system.set_window_maximized(true);
    system.set_current_shader(Shader::Basic);

    /*    system.add_font("Arial", "src/fonts/ArialCE.ttf");
    system.render_text(
        "Test".to_string(),
        "Arial".to_string(),
        [200.0, 200.0],
        [10.0, 10.0],
        0.0,
        [1.0, 1.0, 1.0],
    ); */
    //let projection = ProjectionPerspective::new(0.6, 120.0, 0.0, 800.0);
    //system.camera_mut().set_projection(projection);
    //let window_resolution = system.get_window_resolution();
    /*   let projection = ProjectionOrtho::new(
        0.0,
        window_resolution[0],
        0.0,
        window_resolution[1],
        -500.0,
        500.0,
    ); */
    //system.camera_mut().set_projection(projection);
    /*  system
    .camera_mut()
    .transform_mut()
    .rotate(Axis::YAxis, -90.0); */

    let mut sprite = GameObject::game_object_from_sprite(
        [500.0, 200.0, 0.0],
        "src/sprites/placeholder.png".to_string(),
    );
    sprite.transform_mut().set_local_scale([1.0, 1.5, 1.0]);
    sprite.transform_mut().rotate(Axis::ZAxis, -90.0);
    //sprite.transform_mut().rotate(Axis::XAxis, 90.0);
    //sprite.transform_mut().rotate(Axis::YAxis, 90.0);

    system.add_game_object("Sprite 1".to_string(), sprite);

    let sprite2 = GameObject::game_object_from_sprite(
        [500.0, 200.0, 0.0],
        "src/sprites/placeholder.png".to_string(),
    );
    system.add_game_object("Sprite 2".to_string(), sprite2);
    event_manager.set_key_callback(process_inputs);
    event_manager.set_device_added_callback(device_added);
    event_manager.set_device_removed_callback(device_removed);
    event_manager.set_motion_callback(motion);
    event_manager.set_mouse_motion_callback(mouse_motion);

    let sprite3 =
        GameObject::game_object_from_sprite([1300.0, 200.0, 0.0], "NOT EXISTENT.png".to_string());
    system.add_game_object("Sprite 3".to_string(), sprite3);
    event_manager.set_key_callback(process_inputs);

    setup_sound(system);
}
fn update(system: &mut System, _event_manager: &mut EventManager) {
    let window_res = system.get_window_resolution().clone();
    let mut object_transform = system
        .get_game_object_mut("Sprite 2".to_string())
        .unwrap()
        .transform_mut()
        .clone();
    object_transform.rotate(Axis::ZAxis, 10.0);
    if object_transform.local_position()[0] < window_res[0] {
        object_transform.translate([1000.0, 0.0, 0.0]);
        object_transform.scale([2.0, 0.5, 1.0]);
    } else {
        object_transform.set_local_position([-1.0, 1.0, 1.0]);
        object_transform.set_local_scale([1.0, 1.0, 1.0]);
    }

    let sound_context = system
        .get_sound_context("Basic context".to_string())
        .unwrap();
    let handle = system.get_sound_source("Moving".to_string()).unwrap();
    let old_position = sound_context
        .state()
        .source_mut(*handle)
        .spatial_mut()
        .position();
    if old_position[0] < 3.0 {
        sound_context
            .state()
            .source_mut(*handle)
            .spatial_mut()
            .set_position(Vector3::new(
                old_position[0] + 1.0,
                old_position[1],
                old_position[2],
            ));
    } else {
        sound_context
            .state()
            .source_mut(*handle)
            .spatial_mut()
            .set_position(Vector3::new(-1.0, 0.0, 0.0));
    }
}
fn stop(_system: &mut System, _event_manager: &mut EventManager) {}

fn process_inputs(system: &mut System, key: KeyboardInput, _device_id: DeviceId) {
    let key_code = key.virtual_keycode;
    match key_code {
        None => println!("Key not recognized"),
        Some(virtual_key) => match virtual_key {
            VirtualKeyCode::D => match key.state {
                alpha_engine::event::ElementState::Pressed => {
                    /* system
                    .camera_mut()
                    .transform_mut()
                    .translate([100.0, 0.0, 0.0]); */
                    system
                        .get_game_object_mut("Sprite 2".to_string())
                        .unwrap()
                        .transform_mut()
                        .translate([1000.0, 0.0, 0.0]);
                }
                alpha_engine::event::ElementState::Released => (),
            },
            VirtualKeyCode::A => match key.state {
                alpha_engine::event::ElementState::Pressed => {
                    system
                        .get_game_object_mut("Sprite 2".to_string())
                        .unwrap()
                        .transform_mut()
                        .translate([-1000.0, 0.0, 0.0]);
                }
                alpha_engine::event::ElementState::Released => (),
            },

            VirtualKeyCode::S => match key.state {
                alpha_engine::event::ElementState::Pressed => {
                    system
                        .get_game_object_mut("Sprite 2".to_string())
                        .unwrap()
                        .transform_mut()
                        .translate([0.0, -1000.0, 0.0]);
                    system.set_framerate_target(system.framerate_target() - 1.0)
                }
                alpha_engine::event::ElementState::Released => (),
            },
            VirtualKeyCode::W => match key.state {
                alpha_engine::event::ElementState::Pressed => {
                    system
                        .get_game_object_mut("Sprite 2".to_string())
                        .unwrap()
                        .transform_mut()
                        .translate([0.0, 1000.0, 0.0]);
                    system.set_framerate_target(system.framerate_target() + 1.0)
                }
                alpha_engine::event::ElementState::Released => (),
            },
            VirtualKeyCode::P => match key.state {
                alpha_engine::event::ElementState::Pressed => system.set_framerate_target(10.0),
                alpha_engine::event::ElementState::Released => system.set_framerate_target(60.0),
            },
            VirtualKeyCode::Z => match key.state {
                alpha_engine::event::ElementState::Pressed => {
                    let sound_context = system
                        .get_sound_context("Basic context".to_string())
                        .unwrap();
                    let handle = system.get_sound_source("Punch".to_string()).unwrap();
                    sound_context.state().source_mut(*handle).play();
                }
                _ => (),
            },
            VirtualKeyCode::X => match key.state {
                alpha_engine::event::ElementState::Pressed => {
                    let sound_context = system
                        .get_sound_context("Basic context".to_string())
                        .unwrap();
                    let moving = system.get_sound_source("Moving".to_string()).unwrap();
                    let left = system.get_sound_source("Left".to_string()).unwrap();
                    let right = system.get_sound_source("Right".to_string()).unwrap();

                    if sound_context.state().source_mut(*moving).status() == Status::Paused {
                        sound_context.state().source_mut(*moving).play();
                        sound_context.state().source_mut(*left).play();
                        sound_context.state().source_mut(*right).play();
                    } else {
                        sound_context.state().source_mut(*moving).pause();
                        sound_context.state().source_mut(*left).pause();
                        sound_context.state().source_mut(*right).pause();
                    }
                }
                _ => (),
            },
            _ => (),
        },
    };
}
fn device_added(_device_id: DeviceId) {
    //println!("Device {:?} added.", device_id);
}
fn device_removed(_device_id: DeviceId) {
    //println!("Device {:?} removed.", device_id);
}
fn motion(_axis: u32, _value: f64, _device_id: DeviceId) {
    //println!("AxisMotion, {:?}, {:?}, {:?}", axis, value, device_id)
}
fn mouse_motion(_delta: (f64, f64), _device_id: DeviceId) {
    //println!("MouseMotion, {:?}, {:?}", delta, device_id)
}
fn setup_sound(system: &mut System) {
    let sound_context = system.create_sound_context();

    system.add_sound_context("Basic context".to_string(), sound_context);
    let sound_buffer = system.add_sound_buffer_from_file(
        "Punch".to_string(),
        "src/audio/punch.wav".to_string(),
        false,
    );

    let generic_source = GenericSourceBuilder::new()
        .with_buffer(sound_buffer)
        .with_gain(0.25)
        .with_status(Status::Paused);
    let source = system.create_sound_source_from_generic(generic_source, false);
    let handle = system.add_source_to_context("Basic context".to_string(), source);
    system.add_sound_source("Punch".to_string(), handle);

    let sine_source = system.add_sound_buffer_from_file(
        "440".to_string(),
        "src/audio/440.wav".to_string(),
        false,
    );

    // Left spatial source
    let generic_left_source = GenericSourceBuilder::new()
        .with_buffer(sine_source.clone())
        .with_status(Status::Paused)
        .with_gain(0.5)
        .with_looping(true)
        .with_pitch(1.0);
    let mut left_source = system.create_sound_source_from_generic(generic_left_source, true);
    let left_source_spatial = left_source.spatial_mut();
    left_source_spatial.set_position(Vector3::new(-100.0, 0.0, 0.0));

    let handle = system.add_source_to_context("Basic context".to_string(), left_source);

    system.add_sound_source("Left".to_string(), handle);

    // Right spatial source
    let generic_right_source = GenericSourceBuilder::new()
        .with_buffer(sine_source.clone())
        .with_status(Status::Paused)
        .with_gain(0.5)
        .with_looping(true)
        .with_pitch(1.25);

    let mut right_source = system.create_sound_source_from_generic(generic_right_source, true);
    let right_source_spatial = right_source.spatial_mut();
    right_source_spatial.set_position(Vector3::new(100.0, 0.0, 0.0));

    let handle = system.add_source_to_context("Basic context".to_string(), right_source);

    system.add_sound_source("Right".to_string(), handle);
    // Moving spatial source
    let generic_moving = GenericSourceBuilder::new()
        .with_buffer(sine_source)
        .with_status(Status::Paused)
        .with_gain(0.10)
        .with_looping(true)
        .with_pitch(2.0);

    let moving_source = system.create_sound_source_from_generic(generic_moving, true);
    let handle = system.add_source_to_context("Basic context".to_string(), moving_source);
    system.add_sound_source("Moving".to_string(), handle);
}
fn main() {
    let game = Game::new(start, update, stop);
    let engine = Engine::new(game, "Basic game".to_string());
    engine.start_main_loop();
}
