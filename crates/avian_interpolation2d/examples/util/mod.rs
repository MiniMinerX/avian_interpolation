use avian_interpolation2d::prelude::*;
use bevy::{input::common_conditions::input_just_pressed, prelude::*};

pub fn plugin(example: Example) -> impl Plugin {
    move |app: &mut App| {
        app.add_systems(Startup, spawn_text(example))
            // Purely aesthetic systems go in `Update`.
            .add_systems(
                Update,
                (
                    toggle_interpolation.run_if(input_just_pressed(KeyCode::Space)),
                    update_text,
                )
                    .chain(),
            );
    }
}

/// Used to tell `spawn_text` which instructions to spawn.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Example {
    /// The minimal set of instructions.
    #[allow(dead_code)]
    Generic,
    /// Includes instructions for moving a box.
    #[allow(dead_code)]
    Moving,
}

fn toggle_interpolation(mut query: Query<&mut InterpolationMode>) {
    for mut interpolation_mode in &mut query {
        *interpolation_mode = match *interpolation_mode {
            InterpolationMode::Linear => InterpolationMode::None,
            InterpolationMode::None => InterpolationMode::Linear,
        };
    }
}

#[derive(Component)]
struct InstructionText;

/// Spawn instructions for the user, depending on the example.
fn spawn_text(example: Example) -> impl Fn(Commands) {
    move |mut commands: Commands| {
        commands
            .spawn(NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    bottom: Val::Px(12.0),
                    left: Val::Px(12.0),
                    ..default()
                },
                ..default()
            })
            .with_children(|parent| {
                let text = |val: &str| {
                    TextSection::new(
                        val,
                        TextStyle {
                            font_size: 25.0,
                            ..default()
                        },
                    )
                };
                let sections = match example {
                    Example::Generic => vec![
                        "Press Space to toggle interpolation modes.\n",
                        "Current interpolation mode: ",
                    ],
                    Example::Moving => vec![
                        "Use WASD to move the box.\n",
                        "Press Space to toggle interpolation modes.\n",
                        "Current interpolation mode: ",
                    ],
                };
                parent.spawn((
                    TextBundle::from_sections(sections.into_iter().map(text)),
                    InstructionText,
                ));
            });
    }
}

fn update_text(
    mut texts: Query<&mut Text, With<InstructionText>>,
    interpolation_modes: Query<&InterpolationMode>,
) {
    let Some(interpolation_mode) = interpolation_modes.iter().next() else {
        return;
    };
    let interpolated = match interpolation_mode {
        InterpolationMode::Linear => "Linear",
        InterpolationMode::None => "None",
    };
    for mut text in &mut texts {
        text.sections.last_mut().unwrap().value =
            format!("Current interpolation mode: {interpolated}");
    }
}
