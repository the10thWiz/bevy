//! Create and play an animation defined by code that operates on the `Transform` component.

use std::f32::consts::PI;

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 1.0,
        })
        .add_startup_system(setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut animations: ResMut<Assets<AnimationClip>>,
) {
    // Camera
    commands.spawn_bundle(Camera3dBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    // Spawn satellite
    let satellite = commands
        .spawn_bundle(PbrBundle {
            transform: Transform::from_xyz(1.5, 0.0, 0.0),
            mesh: meshes.add(Mesh::from(shape::Cube { size: 0.5 })),
            material: materials.add(Color::rgb(0.3, 0.9, 0.3).into()),
            ..default()
        })
        .insert(Name::new("satellite"))
        // Save entity ID
        .id();
    // An orbit controller with no visible parts, purely for animation
    let orbit_controller = commands
        .spawn_bundle(SpatialBundle::VISIBLE_IDENTITY)
        .insert(Name::new("orbit_controller"))
        // parent the satellite to orbit_controller
        .insert_children(0, &[satellite])
        // Save entity ID
        .id();
    // Spawn central planet
    let planet = commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Icosphere::default())),
            material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
            ..default()
        })
        .insert(Name::new("planet"))
        // parent the orbit_controller to planets
        .insert_children(0, &[orbit_controller])
        .id();

    // Creating the animation
    let mut animation = AnimationClip::default();
    // A curve can modify a single part of a transform, here the translation
    animation.add_curve_to_path(
        planet,
        VariableCurve {
            keyframe_timestamps: vec![0.0, 1.0, 2.0, 3.0, 4.0],
            keyframes: Keyframes::Translation(vec![
                Vec3::new(1.0, 0.0, 1.0),
                Vec3::new(-1.0, 0.0, 1.0),
                Vec3::new(-1.0, 0.0, -1.0),
                Vec3::new(1.0, 0.0, -1.0),
                // in case seamless looping is wanted, the last keyframe should
                // be the same as the first one
                Vec3::new(1.0, 0.0, 1.0),
            ]),
        },
    );
    // Or it can modify the rotation of the transform.
    // To find the entity to modify, the hierarchy  will be traversed looking for
    // an entity with the right name at each level
    animation.add_curve_to_path(
        orbit_controller,
        VariableCurve {
            keyframe_timestamps: vec![0.0, 1.0, 2.0, 3.0, 4.0],
            keyframes: Keyframes::Rotation(vec![
                Quat::IDENTITY,
                Quat::from_axis_angle(Vec3::Y, PI / 2.),
                Quat::from_axis_angle(Vec3::Y, PI / 2. * 2.),
                Quat::from_axis_angle(Vec3::Y, PI / 2. * 3.),
                Quat::IDENTITY,
            ]),
        },
    );
    // If a curve in an animation is shorter than the other, it will not repeat
    // until all other curves are finished. In that case, another animation should
    // be created for each part that would have a different duration / period
    animation.add_curve_to_path(
        satellite,
        VariableCurve {
            keyframe_timestamps: vec![0.0, 0.5, 1.0, 1.5, 2.0, 2.5, 3.0, 3.5, 4.0],
            keyframes: Keyframes::Scale(vec![
                Vec3::splat(0.8),
                Vec3::splat(1.2),
                Vec3::splat(0.8),
                Vec3::splat(1.2),
                Vec3::splat(0.8),
                Vec3::splat(1.2),
                Vec3::splat(0.8),
                Vec3::splat(1.2),
                Vec3::splat(0.8),
            ]),
        },
    );
    // There can be more than one curve targeting the same entity path
    animation.add_curve_to_path(
        satellite,
        VariableCurve {
            keyframe_timestamps: vec![0.0, 1.0, 2.0, 3.0, 4.0],
            keyframes: Keyframes::Rotation(vec![
                Quat::IDENTITY,
                Quat::from_axis_angle(Vec3::Y, PI / 2.),
                Quat::from_axis_angle(Vec3::Y, PI / 2. * 2.),
                Quat::from_axis_angle(Vec3::Y, PI / 2. * 3.),
                Quat::IDENTITY,
            ]),
        },
    );

    // Create the animation player, and set it to repeat
    let mut player = AnimationPlayer::default();
    player.play(animations.add(animation)).repeat();

    // The animation player needs to be added to an entity (although it doesn't matter which)
    commands.spawn().insert(player);
}
