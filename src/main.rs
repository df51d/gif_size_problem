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
    commands.spawn(
        (
            //xd
            AnimatedImageController::play(asset_server.load("foo.gif"))
        ),
    );
}

fn print(
    mut playing_images: Query<(Entity, &mut AnimatedImageController, &mut Sprite)>,
    // animated_images: Res<Assets<AnimatedImage>>,
    images: Res<Assets<Image>>,
    time: Res<Time>,
) {
    for (e, a, s) in &playing_images {
        let img = images.get(&s.image).unwrap();
        dbg!(img.size(), time.elapsed());
    }
}
