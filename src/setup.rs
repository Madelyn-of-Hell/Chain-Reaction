use bevy::asset::Assets;
use bevy::color::Color;
use bevy::prelude::*;
use crate::BOARD_SIZE;
use crate::types::{Board, Coordinates};

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut windows: Query<&mut Window>,
    mut board: ResMut<Board>,
) {
    let resolution = (windows.single_mut().resolution.width(), windows.single_mut().resolution.height());
    commands.spawn((
        Camera2d,
        Transform::from_xyz(resolution.0/2.0, resolution.1/2.0, 0.0),
    ));
    board.init();
    for x in 0..BOARD_SIZE[0] {
        for y in 0..BOARD_SIZE[1] {
            commands.spawn((Node { justify_content: JustifyContent::Center, ..default() },)).with_children(|parent| {
                parent.spawn((
                    Button,
                    Node {
                        width: Val::Px(25.0),
                        height: Val::Px(25.0),
                        border: UiRect::all(Val::Px(25.0)),
                        top: Val::Px(y as f32 * (resolution.1 / 10.0)),
                        left: Val::Px(x as f32 * (resolution.1 / 10.0)),
                        ..default()
                    },
                    BorderColor(Color::BLACK),
                    BackgroundColor(Color::srgb(1.0, 1.0, 1.0)),
                    Coordinates::new(x,y).expect(format!("Board generation left bounds. This should not have happened. Please message Maddie if you see this.\nCoordinates: X: {} | Y: {}", x, y).as_str())
                ))
                .with_child(
                    (
                        Node {
                            width: Val::Px(20.0),
                            height: Val::Px(20.0),
                            justify_content: JustifyContent::Center,
                            position_type: PositionType::Absolute,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        Text::new(""),

                    )
                );
            }
            );
        }
    }
}
