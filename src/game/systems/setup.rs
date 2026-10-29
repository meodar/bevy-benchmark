use crate::{
    game::{
        bundles::{side_enemy_bundle::SideEnemyBundle, top_enemy_bundle::TopEnemyBundle},
        components::{
            health::Health, main_camera::MainCamera, player::Player,
            radial_hit_detector::RadialHitDetector, shooting_point::ShootingPoint, speed::Speed,
            terrain::Terrain,
        },
        resources::{
            benchmark_timer::BenchmarkData, bullet_asset::BulletAssets,
            enemies_manager::EnemiesManager, enemy_prototypes::EnemyPrototypes,
            enemy_spawn_timer::EnemySpawnTimer, skybox_image::SkyboxImage,
        },
    },
    random_table,
};
use bevy::prelude::*;
use bevy::{camera::visibility::RenderLayers, light::Skybox};
use std::f32::consts::PI;

pub fn setup_game(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Заранее, чтобы заполнение таблиц не пришлось на первый кадр замера
    random_table::warm_up();

    // Спавн модели истребителя
    commands.spawn((
        WorldAssetRoot(
            asset_server.load(GltfAssetLabel::Scene(0).from_asset("models\\spaceship\\scene.gltf")),
        ),
        Transform::from_scale(Vec3::new(20.0, 20.0, 20.0)).with_rotation(Quat::from_rotation_y(PI)),
        Player,
        // Сфера 3.7602 в масштабе 20 плюс пуля врага (1.0028 в масштабе врага 10)
        RadialHitDetector::new(3.7602 * 20.0 + 1.0028 * 10.0),
        ShootingPoint(Vec3::new(0.0, 0.0, 6.07018)),
        Speed(600.0),
        Health::new(1000),
    ));

    let skybox_handle = asset_server
        .load("skybox/SBS - Cloudy Skyboxes - Cubemap/Cubemap/Cubemap_Sky_03-512x3072.png");
    commands.insert_resource(SkyboxImage::new(skybox_handle.clone(), false));

    let camera_tf = Transform {
        translation: Vec3::new(0.0, 1000.0, 0.0), // Камера находится сверху
        rotation: Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2), // Смотрит вниз
        ..Default::default()
    };

    // Спавн ортографической камеры (вид сверху)
    commands.spawn((
        Camera3d::default(),
        Projection::Orthographic(OrthographicProjection {
            near: 0.0,
            far: 30000.0,
            ..OrthographicProjection::default_3d()
        }),
        camera_tf.clone(),
        GlobalTransform::default(),
        RenderLayers::layer(0),
        MainCamera,
        Camera {
            order: 0,
            ..default()
        },
    ));

    commands.spawn((
        Camera3d::default(),
        Skybox {
            image: Some(skybox_handle.clone()),
            brightness: 1000.0,
            ..default()
        },
        camera_tf,
        RenderLayers::layer(1),
        Camera {
            order: -1,
            ..default()
        },
    ));

    commands.spawn((
        DirectionalLight {
            illuminance: 4000.0,
            ..default()
        },
        Transform::from_xyz(0.0, 2.0, 0.0).with_rotation(Quat::from_rotation_x(-PI / 4.)),
    ));

    let top_enemy_scene = asset_server.load("models\\enemy\\top_enemy\\scene.gltf#Scene0");
    let side_enemy_scene = asset_server.load("models\\enemy\\side_enemy\\scene.gltf#Scene0");
    commands.insert_resource(EnemySpawnTimer::new(Timer::from_seconds(
        2.0,
        TimerMode::Repeating,
    )));

    commands.insert_resource(BenchmarkData::new(
        Timer::from_seconds(10.0, TimerMode::Repeating),
        10,
    ));

    commands.spawn((
        WorldAssetRoot(
            asset_server.load(GltfAssetLabel::Scene(0).from_asset("models\\terrain\\terrain.gltf")),
        ),
        Transform::from_xyz(0.0, -800.0, 0.0)
            .with_rotation(Quat::from_rotation_y(-std::f32::consts::FRAC_PI_2))
            .with_scale(Vec3::splat(4000.0)),
        Terrain,
    ));
    commands.spawn((
        WorldAssetRoot(
            asset_server.load(GltfAssetLabel::Scene(0).from_asset("models\\terrain\\terrain.gltf")),
        ),
        Transform::from_xyz(0.0, -800.0, -32000.0)
            .with_rotation(Quat::from_rotation_y(-std::f32::consts::FRAC_PI_2))
            .with_scale(Vec3::splat(4000.0)),
        Terrain,
    ));

    let bullet_scene = asset_server.load("models\\bullet\\bullet.gltf#Scene0");

    commands.insert_resource(BulletAssets::new(bullet_scene));
    commands.insert_resource(EnemiesManager::default());
    commands.insert_resource(EnemyPrototypes {
        top: TopEnemyBundle::prototype(top_enemy_scene),
        side: SideEnemyBundle::prototype(side_enemy_scene),
    });
}
