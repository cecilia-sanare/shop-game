#![windows_subsystem = "windows"]
use std::time::Duration;

use bevy::asset::ChangeWatcher;
use bevy::input::common_conditions::input_toggle_active;
use bevy::prelude::*;
use bevy::render::camera::ScalingMode;
use bevy::render::render_resource::AddressMode;
use bevy::render::render_resource::SamplerDescriptor;
use bevy::render::texture::ImageSampler;
use bevy_embedded_assets::EmbeddedAssetPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use ui::GameUIPlugin;
use world::WorldPlugin;

mod ui;
mod world;

// Monitor Size
const WIDTH: i16 = 2560;
const HEIGHT: i16 = 1440;

const WINDOW_PERCENTAGE: i16 = 2;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum AppState {
    MainMenu,
    #[default]
    InGame,
    Paused,
}

fn main() {
    let mut app = App::new();

    app.insert_resource(ClearColor(Color::hex("5fcde4").unwrap()))
        .insert_resource(Msaa::Off)
        .add_state::<AppState>()
        .add_plugins(
            DefaultPlugins
                .set(AssetPlugin {
                    watch_for_changes: ChangeWatcher::with_delay(Duration::from_secs(1)),
                    ..default()
                })
                .set(ImagePlugin {
                    default_sampler: SamplerDescriptor {
                        address_mode_u: AddressMode::Repeat,
                        address_mode_v: AddressMode::Repeat,
                        address_mode_w: AddressMode::Repeat,
                        ..ImageSampler::nearest_descriptor()
                    },
                    ..default()
                })
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Poly".into(),
                        resolution: (WIDTH / WINDOW_PERCENTAGE, HEIGHT / WINDOW_PERCENTAGE).into(),
                        resizable: false,
                        position: WindowPosition::At(IVec2::new(
                            (WIDTH / WINDOW_PERCENTAGE / 2) as i32,
                            (HEIGHT / 10) as i32,
                        )),
                        focused: true,
                        ..default()
                    }),
                    ..default()
                })
                .build()
                .add_before::<AssetPlugin, _>(EmbeddedAssetPlugin),
        )
        .add_plugins((
            WorldInspectorPlugin::default().run_if(input_toggle_active(false, KeyCode::Grave)),
            GameUIPlugin,
            WorldPlugin,
        ))
        .add_systems(Startup, spawn_camera);

    // this code is compiled only if debug assertions are enabled (debug mode)
    #[cfg(debug_assertions)]
    use bevy::window::close_on_esc;
    #[cfg(debug_assertions)]
    app.add_systems(Update, close_on_esc);

    app.run();
}

fn spawn_camera(mut commands: Commands) {
    let mut camera = Camera2dBundle::default();

    camera.projection.scaling_mode = ScalingMode::AutoMin {
        min_width: 160.0,
        min_height: 90.0,
    };

    commands.spawn((camera, Name::new("Camera")));
}
