//! The title screen that appears when the game starts.

use bevy::{
    prelude::*,
    render::texture::{ImageLoaderSettings, ImageSampler},
};

use super::Screen;
use crate::{
    game::{assets::SoundtrackKey, audio::soundtrack::PlaySoundtrack},
    ui::prelude::*,
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Title), enter_title);
    app.add_systems(OnExit(Screen::Title), exit_title);

    app.register_type::<TitleAction>();
    app.add_systems(Update, handle_title_action.run_if(in_state(Screen::Title)));
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Reflect)]
#[reflect(Component)]
enum TitleAction {
    Play,
    Credits,
    /// Exit doesn't work well with embedded applications.
    #[cfg(not(target_family = "wasm"))]
    Exit,
}

const TITLE_BACKGROUND_COLOR: Color = Color::srgb(0.0, 0.0, 239.0);

fn enter_title(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .ui_root()
        .insert((
            Name::new("Splash screen"),
            BackgroundColor(TITLE_BACKGROUND_COLOR),
            StateScoped(Screen::Splash),
        ))
        .with_children(|children| {
            children.spawn((
                Name::new("Splash image"),
                ImageBundle {
                    style: Style {
                        margin: UiRect::all(Val::Auto),
                        height: Val::Percent(100.0),
                        ..default()
                    },
                    image: UiImage::new(asset_server.load_with_settings(
                        // This should be an embedded asset for instant loading, but that is
                        // currently [broken on Windows Wasm builds](https://github.com/bevyengine/bevy/issues/14246).
                        "images/laboratory.png",
                        |settings: &mut ImageLoaderSettings| {
                            // Make an exception for the splash image in case
                            // `ImagePlugin::default_nearest()` is used for pixel art.
                            settings.sampler = ImageSampler::nearest();
                        },
                    )),
                    ..default()
                },
            ));
        });

    commands
        .spawn((
            Name::new("UI Root"),
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::FlexEnd,
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(10.0),
                    position_type: PositionType::Absolute,
                    ..default()
                },
                ..default()
            },
        ))
        .insert(StateScoped(Screen::Title))
        .with_children(|children| {
            children.button("Play").insert(TitleAction::Play);
            children.button("Credits").insert(TitleAction::Credits);

            #[cfg(not(target_family = "wasm"))]
            children.button("Exit").insert(TitleAction::Exit);
        });

    commands.trigger(PlaySoundtrack::Key(SoundtrackKey::Title));
}

fn handle_title_action(
    mut next_screen: ResMut<NextState<Screen>>,
    mut button_query: InteractionQuery<&TitleAction>,
    #[cfg(not(target_family = "wasm"))] mut app_exit: EventWriter<AppExit>,
) {
    for (interaction, action) in &mut button_query {
        if matches!(interaction, Interaction::Pressed) {
            match action {
                TitleAction::Play => next_screen.set(Screen::Playing),
                TitleAction::Credits => next_screen.set(Screen::Credits),

                #[cfg(not(target_family = "wasm"))]
                TitleAction::Exit => {
                    app_exit.send(AppExit::Success);
                }
            }
        }
    }
}

fn exit_title(mut commands: Commands) {
    // We could use [`StateScoped`] on the sound playing entites instead.
    commands.trigger(PlaySoundtrack::Disable);
}
