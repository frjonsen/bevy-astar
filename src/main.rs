use bevy::{prelude::*, sprite::MaterialMesh2dBundle, window::PresentMode};
use bevy_inspector_egui::WorldInspectorPlugin;

mod position;

use position::Position;
#[derive(Bundle)]
struct Topography {
    spatial: SpatialBundle,
}

#[derive(Component)]
struct Altitude(u8);

#[derive(Resource)]
struct InputMap(Vec<Vec<u8>>);

#[derive(Bundle)]
struct Point {
    altitude: Altitude,
    position: Position,
    mesh: MaterialMesh2dBundle<ColorMaterial>,
}

const WORLD_SIZE: f32 = 1200.;

#[derive(Resource)]
struct TileSize(f32);

impl FromWorld for TileSize {
    fn from_world(world: &mut World) -> Self {
        let input_map = world.get_resource::<InputMap>().unwrap();
        Self(WORLD_SIZE / input_map.0.len().max(input_map.0[0].len()) as f32)
    }
}

fn parse_input() -> Vec<Vec<u8>> {
    let input = include_str!("../input.txt");
    let mut topography: Vec<Vec<u8>> = Vec::new();
    for line in input.trim().lines() {
        let mut p_line: Vec<u8> = Vec::new();

        for c in line.trim().chars().map(|c| match c {
            'S' => 254,
            'E' => 255,
            _ => c as u8 - 97,
        }) {
            p_line.push(c);
        }
        topography.push(p_line);
    }
    topography
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                title: "A* Visualizer".to_string(),
                width: WORLD_SIZE,
                height: WORLD_SIZE / 2.,
                present_mode: PresentMode::AutoVsync,
                ..default()
            },
            ..default()
        }))
        .insert_resource(InputMap(parse_input()))
        .init_resource::<TileSize>()
        .add_startup_system(setup)
        .add_system(bevy::window::close_on_esc)
        // .add_plugin(WorldInspectorPlugin::new())
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    map: Res<InputMap>,
    tile_size: Res<TileSize>,
) {
    commands.spawn(Camera2dBundle::new_with_far(WORLD_SIZE));
    let topography = &map.0;

    let mut rows: Vec<Vec<Point>> = Vec::new();
    for (y, line) in topography.iter().enumerate() {
        let mut parsed_line: Vec<Point> = Vec::new();
        for (x, a) in line.iter().enumerate() {
            let color = match a {
                254 => Color::rgb_u8(255, 0, 0),
                255 => Color::rgb_u8(0, 255, 0),
                _ => Color::rgb_u8(0, 0, 255 / 27 * a),
            };
            let position = Position(x, y);

            let mesh = MaterialMesh2dBundle {
                mesh: meshes
                    .add(shape::Quad::new(Vec2::new(tile_size.0, tile_size.0)).into())
                    .into(),
                material: materials.add(ColorMaterial::from(color)),
                transform: Transform::from_translation(Vec3::new(
                    position.0 as f32 * tile_size.0,
                    position.1 as f32 * tile_size.0,
                    0.,
                )),
                ..default()
            };
            let p = Point {
                altitude: Altitude(*a),
                mesh,
                position,
            };
            parsed_line.push(p);
        }
        rows.push(parsed_line);
    }

    let topography = Topography {
        spatial: SpatialBundle::from_transform(Transform::from_translation(Vec3::new(
            -WORLD_SIZE / 2. + tile_size.0 / 2.,
            -WORLD_SIZE / 4. + tile_size.0 / 2.,
            0.,
        ))),
    };

    let mut t = commands.spawn(topography);

    t.with_children(|p| {
        for row in rows {
            for f in row {
                p.spawn(f);
            }
        }
    });
}
