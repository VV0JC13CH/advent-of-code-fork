//! A shader and a material that uses it.
use std::f32::consts::PI;

use bevy::{
    pbr::CascadeShadowConfigBuilder,
    prelude::*,
    reflect::TypePath,
    render::render_resource::{AsBindGroup, ShaderRef},
};
use bevy_basic_camera::{
    CameraController, CameraControllerPlugin,
};
use day_22::part1::parse_bricks;
use rand::Rng;

fn main() {
    App::new()
        .insert_resource(ClearColor(
            Color::hex("1e1e2e").unwrap(),
        ))
        .add_plugins((
            DefaultPlugins,
            MaterialPlugin::<CustomMaterial>::default(),
        ))
        .add_plugins(CameraControllerPlugin)
        .add_systems(Startup, setup)
        .run();
}

const INPUT: &str = include_str!("../../input1.txt");

const TEST_INPUT: &str = "1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9";
/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<CustomMaterial>>,
    mut materials_std: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let (_, bricks) =
        parse_bricks(INPUT).expect("should parse");
    // directional 'sun' light
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            shadows_enabled: true,
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(0.0, 2.0, 0.0),
            rotation: Quat::from_rotation_x(-PI / 4.),
            ..default()
        },
        // The default cascade config is designed to handle large scenes.
        // As this example has a much smaller world, we can tighten the shadow
        // bounds for better visual quality.
        cascade_shadow_config: CascadeShadowConfigBuilder {
            first_cascade_far_bound: 4.0,
            maximum_distance: 10.0,
            ..default()
        }
        .into(),
        ..default()
    });
    commands.spawn(PbrBundle {
        mesh: meshes
            .add(shape::Plane::from_size(100.0).into()),
        material: materials_std.add(StandardMaterial {
            base_color: Color::hex("313244").unwrap(),
            perceptual_roughness: 1.0,
            ..default()
        }),
        ..default()
    });

    let mut rng = rand::thread_rng();

    for brick in bricks.iter() {
        let hue: i32 = rng.gen_range(0..360);

        let color = Color::Lcha {
            lightness: 0.8,
            chroma: 1.0,
            hue: hue as f32,
            alpha: 1.0,
        };
        for cube in brick.cubes.iter() {
            // cube
            commands.spawn(MaterialMeshBundle {
                mesh: meshes.add(Mesh::from(shape::Cube {
                    size: 1.0,
                })),
                transform: Transform::from_translation(
                    cube.as_vec3().xzy(),
                ),
                // material: materials.add(CustomMaterial {
                //     color,
                //     // color_texture: None,
                //     alpha_mode: AlphaMode::Blend,
                // }),
                material: materials_std.add(
                    StandardMaterial {
                        base_color: color,
                        ..default()
                    },
                ),
                ..default()
            });
        }
    }

    let max_y = bricks
        .iter()
        .flat_map(|brick| brick.cubes.iter())
        .max_by_key(|c| c.z)
        .unwrap();
    let halfway = max_y.z / 2;
    // camera
    dbg!(halfway);
    commands
        .spawn(Camera3dBundle {
            transform: Transform::from_xyz(
                -10.0,
                halfway as f32,
                10.0,
            )
            .looking_at(
                Vec3::new(0.0, halfway as f32, 0.0),
                Vec3::Y,
            ),
            ..default()
        })
        .insert(CameraController::default());
}

// This struct defines the data that will be passed to your shader
#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct CustomMaterial {
    #[uniform(0)]
    color: Color,
    // #[texture(1)]
    // #[sampler(2)]
    // color_texture: Option<Handle<Image>>,
    alpha_mode: AlphaMode,
}

/// The Material trait is very configurable, but comes with sensible defaults for all methods.
/// You only need to implement functions for features that need non-default behavior. See the Material api docs for details!
impl Material for CustomMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/custom_material.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        self.alpha_mode
    }
}
