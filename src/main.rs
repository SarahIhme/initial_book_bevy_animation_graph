use avian3d::prelude::*;
use bevy_ahoy::prelude::*;
use bevy_enhanced_input::prelude::*;
use std::f32::consts::PI;

use bevy::{
    input::common_conditions::input_just_pressed,
    light::CascadeShadowConfigBuilder,
    prelude::*,
    window::{CursorGrabMode, CursorOptions},
};
use bevy_animation_graph::{
    AnimationGraphPlugin,
    core::{
        animated_scene::{AnimatedSceneHandle, AnimatedSceneInstance},
        animation_graph_player::AnimationGraphPlayer,
        edge_data::{
            DataValue,
            events::{AnimationEvent, EventQueue},
        },
    },
};
use bevy_animation_graph_book::locomotion_blend_parameters_node::LocomotionBlendParametersNode;

#[derive(Component)]
struct CharacterControllerScene;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(AssetPlugin {
                file_path: "assets".to_string(),
                ..default()
            }),
            EnhancedInputPlugin,
            PhysicsPlugins::default(),
            PhysicsDebugPlugin::default(),
            AhoyPlugins::default(),
        ))
        .add_input_context::<PlayerInput>()
        .add_plugins(AnimationGraphPlugin::default())
        .register_type::<LocomotionBlendParametersNode>()
        .insert_resource(GlobalAmbientLight {
            color: Color::WHITE,
            brightness: 0.1,
            ..default()
        })
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                pass_speed_to_animgraph,
                capture_cursor.run_if(input_just_pressed(MouseButton::Left)),
                release_cursor.run_if(input_just_pressed(KeyCode::Escape)),
            ),
        )
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Plane
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::new(Vec3::Y, Vec2::new(5., 5.)))),
        MeshMaterial3d(materials.add(Color::from(LinearRgba::rgb(0.3, 0.5, 0.3)))),
        Collider::half_space(Vec3::Y),
        RigidBody::Static,
    ));

    // Light
    commands.spawn((
        Transform::from_rotation(Quat::from_euler(EulerRot::ZYX, 0.0, 1.0, -PI / 4.)),
        DirectionalLight {
            shadows_enabled: true,
            ..default()
        },
        CascadeShadowConfigBuilder {
            first_cascade_far_bound: 10.0,
            num_cascades: 3,
            minimum_distance: 0.3,
            maximum_distance: 100.0,
            ..default()
        }
        .build(),
    ));

    let player = commands
        .spawn((
            // Add the character controller configuration. We'll use the default settings for now.
            CharacterController {
                acceleration_hz: 3.,
                speed: 8.,
                max_speed: 15.,
                ..Default::default()
            },
            // The KCC currently behaves best when using a cylinder
            Collider::cylinder(0.7, 1.8),
            Transform::from_xyz(0.0, 20.0, 0.0),
            children![(
                Transform::from_xyz(0., -0.9, 0.),
                AnimatedSceneHandle::new(
                    asset_server.load("animated_scenes/human_character_controller.animscn.ron")
                ),
                CharacterControllerScene
            )],
            // Configure inputs. The actions `Movement`, `Jump`, etc. are provided by Ahoy, you just need to bind them.
            PlayerInput,
            actions!(PlayerInput[
                (
                    Action::<Movement>::new(),
                    // Normalize the input vector
                    DeadZone::default(),
                    Bindings::spawn((
                        Cardinal::wasd_keys(),
                        Axial::left_stick()
                    ))
                ),
                (
                    Action::<Jump>::new(),
                    bindings![KeyCode::Space,  GamepadButton::South],
                ),
                (
                    Action::<Crouch>::new(),
                    bindings![KeyCode::ControlLeft, GamepadButton::LeftTrigger2],
                ),
                (
                    Action::<RotateCamera>::new(),
                    Bindings::spawn((
                        // tweak mouse and right stick sensitivity
                        // in Scale::splat values
                        Spawn((Binding::mouse_motion(), Scale::splat(0.07))),
                        Axial::right_stick().with((Scale::splat(4.0), DeadZone::default())),
                    ))
                ),
            ]),
        ))
        .id();

    // Spawn the camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(3., 3., 3.).looking_at(Vec3::new(0.0, 0.875, 0.0), Vec3::Y),
    ));
}

fn pass_speed_to_animgraph(
    velocity: Single<&LinearVelocity, With<CharacterController>>,
    human_character: Query<&AnimatedSceneInstance, With<CharacterControllerScene>>,
    mut animation_players: Query<&mut AnimationGraphPlayer>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    let Ok(player_entity) = human_character.single().map(|i| i.player_entity()) else {
        return;
    };

    let Ok(mut player) = animation_players.get_mut(player_entity) else {
        return;
    };
    let velocity_vec = velocity.zx();
    let velocity_val = velocity_vec.length();

    let mut eq = EventQueue::default();
    if keys.just_pressed(KeyCode::KeyP) {
        println!("Fired punch");
        eq.add_instant_event(AnimationEvent::TransitionToStateLabel("Punch".into()));
    }

    player.set_input_data("speedVec", DataValue::from(velocity_vec));
    player.set_input_data("speed", DataValue::from(velocity_val));
    player.set_input_data("driver_events", eq.into());
}

#[derive(Component, Default)]
pub(crate) struct PlayerInput;

fn capture_cursor(mut cursor: Single<&mut CursorOptions>) {
    cursor.grab_mode = CursorGrabMode::Locked;
    cursor.visible = false;
}

fn release_cursor(mut cursor: Single<&mut CursorOptions>) {
    cursor.visible = true;
    cursor.grab_mode = CursorGrabMode::None;
}
