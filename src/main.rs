use bevy::{prelude::*, sprite::MaterialMesh2dBundle, window::PresentMode};
use bevy_inspector_egui::WorldInspectorPlugin;

#[derive(Bundle)]
struct Topography {
    spatial: SpatialBundle,
}

#[derive(Component)]
struct Altitude(u8);

#[derive(Bundle)]
struct Point {
    altitude: Altitude,
    mesh: MaterialMesh2dBundle<ColorMaterial>,
}

const WORLD_SIZE: f32 = 1200.;

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
                title: "A*".to_string(),
                width: WORLD_SIZE / 2.,
                height: WORLD_SIZE,
                present_mode: PresentMode::AutoVsync,
                ..default()
            },
            ..default()
        }))
        .add_startup_system(setup)
        // .add_plugin(WorldInspectorPlugin::new())
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle::new_with_far(WORLD_SIZE));
    let topography = parse_input();
    let tile_size = WORLD_SIZE / topography.len().max(topography[0].len()) as f32;

    let mut rows: Vec<Vec<Point>> = Vec::new();
    for (y, line) in topography.into_iter().enumerate() {
        let mut parsed_line: Vec<Point> = Vec::new();
        for (x, a) in line.into_iter().enumerate() {
            let color = match a {
                254 => Color::rgb_u8(255, 0, 0),
                255 => Color::rgb_u8(0, 255, 0),
                _ => Color::rgb_u8(0, 0, 255 / 27 * a),
            };
            let mesh = MaterialMesh2dBundle {
                mesh: meshes
                    .add(shape::Quad::new(Vec2::new(tile_size, tile_size)).into())
                    .into(),
                material: materials.add(ColorMaterial::from(color)),
                transform: Transform::from_translation(Vec3::new(
                    y as f32 * tile_size,
                    x as f32 * tile_size,
                    0.,
                )),
                ..default()
            };
            let p = Point {
                altitude: Altitude(a),
                mesh,
            };
            parsed_line.push(p);
        }
        rows.push(parsed_line);
    }

    let topography = Topography {
        spatial: SpatialBundle::from_transform(Transform::from_translation(Vec3::new(
            -WORLD_SIZE / 4.,
            -WORLD_SIZE / 2.,
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
