use crate::resources::SoundsConfig;
use amethyst::{
    assets::{AssetStorage, Handle, Loader},
    audio::{output::Output, OggFormat, Source, SourceHandle},
    ecs::{World, WorldExt},
};

use std::collections::HashMap;

pub struct Sounds {
    pub sound_effects: HashMap<String, SourceHandle>,
}

fn load_audio_track(loader: &Loader, world: &World, file: &str) -> SourceHandle {
    loader.load(file, OggFormat, (), &world.read_resource())
}

pub fn initialize_audio(world: &mut World) {
    let sound_effects = {
        let loader = world.read_resource::<Loader>();
        let sound_data = world.read_resource::<SoundsConfig>();

        let mut sound_effects = HashMap::new();

        for (sound_name, sound_file) in sound_data.iter() {
            sound_effects.insert(
                sound_name.to_owned(),
                load_audio_track(&loader, world, &*("audio/".to_string() + sound_file)),
            );
        }

        Sounds { sound_effects }
    };

    world.insert(sound_effects);
}

pub fn play_sfx(sound: &Handle<Source>, storage: &AssetStorage<Source>, output: Option<&Output>) {
    if let Some(output) = output.as_ref() {
        if let Some(sound) = storage.get(sound) {
            output.play_once(sound, 1.0);
        }
    }
}
