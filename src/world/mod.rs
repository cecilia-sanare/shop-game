use bevy::prelude::*;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn)
            .add_systems(Update, (move_clouds, spawn_clouds));
    }
}

pub enum Layers {
    WORLD,
    BACKGROUND,
    DECOR,
}

impl Into<f32> for Layers {
    fn into(self) -> f32 {
        f32::from(self as u8)
    }
}

fn spawn(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(SpriteBundle {
        texture: asset_server.load("door.png"),
        transform: Transform {
            translation: Vec3::new(0.0, 0.0, Layers::DECOR.into()),
            ..default()
        },
        ..default()
    });

    commands.spawn(SpriteBundle {
        texture: asset_server.load("shop.png"),
        transform: Transform {
            translation: Vec3::new(0.0, 0.0, Layers::BACKGROUND.into()),
            ..default()
        },
        sprite: Sprite {
            custom_size: Some(Vec2::new(160.0, 90.0)),
            ..default()
        },
        ..default()
    });

    commands.spawn(SpriteBundle {
        texture: asset_server.load("grass.png"),
        transform: Transform {
            translation: Vec3::new(0.0, 0.0, Layers::WORLD.into()),
            ..default()
        },
        sprite: Sprite {
            custom_size: Some(Vec2::new(160.0, 90.0)),
            ..default()
        },
        ..default()
    });
}

fn move_clouds(mut clouds: Query<(&mut Transform, &Cloud)>, time: Res<Time>) {
    for (mut transform, cloud) in clouds.iter_mut() {
        transform.translation.x += cloud.speed * time.delta_seconds();
    }
}

fn spawn_clouds(
    mut commands: Commands,
    clouds: Query<(Entity, &Transform, &Handle<Image>), With<Cloud>>,
    asset_server: Res<AssetServer>,
) {
    if clouds.iter().len() == 0 {
        for i in 0i8..3 {
            let file: &str = match i {
                x if x % 3 == 0 => "cloud-l.png",
                x if x % 2 == 0 => "cloud-m.png",
                _ => "cloud-s.png".into(),
            };

            commands.spawn((
                SpriteBundle {
                    texture: asset_server.load(file),
                    transform: Transform {
                        translation: Vec3::new(60.0 * f32::from(i), 0.0, Layers::WORLD.into()),
                        ..default()
                    },
                    sprite: Sprite {
                        custom_size: Some(Vec2::new(160.0, 90.0)),
                        ..default()
                    },
                    ..default()
                },
                Cloud::default(),
            ));
        }
    }

    for (entity, transform, image) in clouds.iter() {
        if transform.translation.x < 160.0 {
            continue;
        }

        commands.entity(entity).despawn();

        commands.spawn((
            SpriteBundle {
                texture: image.clone(),
                transform: Transform {
                    translation: Vec3::new(-160.0, 0.0, Layers::WORLD.into()),
                    ..default()
                },
                sprite: Sprite {
                    custom_size: Some(Vec2::new(160.0, 90.0)),
                    ..default()
                },
                ..default()
            },
            Cloud::default(),
        ));
    }
}

#[derive(Component)]
pub struct Cloud {
    pub speed: f32,
}

impl Default for Cloud {
    fn default() -> Self {
        Self { speed: 10.0 }
    }
}
