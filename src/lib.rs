#![feature(reflect_marker)]
#![deny(missing_docs)]

//! A high level library for playing music

extern crate current;
extern crate sdl2_mixer;
extern crate sdl2;

use sdl2_mixer as mix;
use current::{ Current, CurrentGuard };
use std::collections::HashMap;
use std::hash::Hash;
use std::marker::Reflect;
use std::path::Path;

fn init_audio() {
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

unsafe fn current_music_tracks<T: 'static + Reflect>() -> Current<HashMap<T, mix::Music>> {
    Current::new()
}

/// Initializes audio and sets up current objects.
pub fn start<T: Eq + Hash + 'static + Reflect, F: FnOnce()>(f: F) {
    let sdl = sdl2::init().unwrap();
    let audio = sdl.audio().unwrap();
    let timer = sdl.timer().unwrap();
    
    init_audio();
    let mut music_tracks: HashMap<T, mix::Music> = HashMap::new();

    let music_tracks_guard = CurrentGuard::new(&mut music_tracks);

    f();

    drop(music_tracks_guard);
    drop(timer);
    drop(audio);
    drop(sdl);
}

/// Binds a file to value.
pub fn bind_file<T, P>(val: T, file: P)
    where T: 'static + Eq + Hash + Reflect,
          P: AsRef<Path>
{
    let track = mix::Music::from_file(&file.as_ref()).unwrap();
    unsafe { current_music_tracks() }.insert(val, track);
}

/// Tells how many times to repeat.
#[derive(Copy, Clone)]
pub enum Repeat {
    /// Repeats forever.
    Forever,
    /// Repeats amount of times.
    Times(u16),
}

impl Repeat {
    fn to_sdl2_repeats(&self) -> isize {
        match self {
            &Repeat::Forever => -1,
            &Repeat::Times(val) => {
                val as isize
            }
        }
    }
}

/// Plays a music track.
pub fn play<T: Eq + Hash + 'static + Reflect>(val: &T, repeat: Repeat) {
    let _ = unsafe { current_music_tracks::<T>() }.get(val)
        .expect("music: Attempted to play value that is not bound to asset")
        .play(repeat.to_sdl2_repeats());
}

