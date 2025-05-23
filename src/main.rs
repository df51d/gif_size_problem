use bevy::prelude::*;

use vleue_kinetoscope::{AnimatedImageController, AnimatedImagePlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(AnimatedImagePlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, print)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);
    commands.spawn(AnimatedImageController::play(asset_server.load("foo.gif")));
}

fn print(
    playing_images: Query<(Entity, &mut AnimatedImageController, &mut Sprite)>,
    images: Res<Assets<Image>>,
    time: Res<Time>,
) {
    for (_entity, _animated_image_controller, sprite) in &playing_images {
        let img = images.get(&sprite.image).unwrap();
        dbg!(img.size(), time.elapsed());
    }
}
