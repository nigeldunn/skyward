use skyward::SkywardPlugin;
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(SkywardPlugin)
        .run();
}
