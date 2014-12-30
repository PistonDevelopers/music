
#![deny(missing_docs)]

//! A high level library for playing music

extern crate current;
extern crate sdl2_mixer;
extern crate sdl2;

use sdl2_mixer as mix;
use current::{ Current, CurrentGuard };
use std::collections::HashMap;
use std::hash::Hash;

fn init_audio() {
    sdl2::init(sdl2::INIT_AUDIO | sdl2::INIT_TIMER);
    // Load dynamic libraries.
    mix::init(
          mix::INIT_MP3
        | mix::INIT_FLAC
        | mix::INIT_MOD
        | mix::INIT_FLUIDSYNTH
        | mix::INIT_MODPLUG
        | mix::INIT_OGG
    );
    mix::open_audio(
        mix::DEFAULT_FREQUENCY,
        mix::DEFAULT_FORMAT,
        mix::DEFAULT_CHANNELS,
        1024
    ).unwrap();
    mix::allocate_channels(mix::DEFAULT_CHANNELS);
}

unsafe fn current_music_tracks<T: 'static>() -> Current<HashMap<T, mix::Music>> {
    Current::new()
}

/// Initializes audio and sets up current objects.
pub fn start<T: Eq + Hash + 'static>(f: ||) {
    init_audio();
    let mut music_tracks: HashMap<T, mix::Music> = HashMap::new();

    let music_tracks_guard = CurrentGuard::new(&mut music_tracks);

    f();

    drop(music_tracks_guard);
}

