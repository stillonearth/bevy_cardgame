//! Game mechanics and content.

use bevy::prelude::*;

// mod animation;
pub mod assets;
pub mod audio;
pub mod cards;
pub mod spawn;
pub mod ui;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        // animation::plugin,
        audio::plugin,
        assets::plugin,
        spawn::plugin,
        cards::plugin,
        ui::plugin,
    ));
}
