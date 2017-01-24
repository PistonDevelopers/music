#![deny(missing_docs)]

//! A high level library for playing music

extern crate current;
extern crate sdl2;

use sdl2::mixer as mix;
use current::{Current, CurrentGuard};
use std::collections::HashMap;
use std::hash::Hash;
use std::any::Any;
use std::path::Path;

/// Minimum value for playback volume parameter.
pub const MIN_VOLUME: f64 = 0.0;

/// Maximum value for playback volume parameter.
pub const MAX_VOLUME: f64 = 1.0;

fn init_audio() {
    // Load dynamic libraries.
    // Ignore formats that are not built in.
    let _ = mix::init(mix::INIT_MP3 | mix::INIT_FLAC | mix::INIT_MOD | mix::INIT_FLUIDSYNTH |
                      mix::INIT_MODPLUG | mix::INIT_OGG);
    mix::open_audio(mix::DEFAULT_FREQUENCY,
                    mix::DEFAULT_FORMAT,
                    mix::DEFAULT_CHANNELS,
                    1024)
        .unwrap();
    mix::allocate_channels(mix::DEFAULT_CHANNELS);
}

unsafe fn current_music_tracks<T: 'static + Any>() -> Current<HashMap<T, mix::Music>> {
    Current::new()
}

/// Initializes audio and sets up current objects.
pub fn start<T: Eq + Hash + 'static + Any, F: FnOnce()>(f: F) {
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
    where T: 'static + Eq + Hash + Any,
          P: AsRef<Path>
{
    let track = mix::Music::from_file(file.as_ref()).unwrap();
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
    fn to_sdl2_repeats(&self) -> i32 {
        match *self {
            Repeat::Forever => -1,
            Repeat::Times(val) => val as i32,
        }
    }
}

/// Sets the volume of the music mixer.
///
/// The volume is set on a scale of 0.0 to 1.0, which means 0-100%.
/// Values greater than 1.0 will use 1.0.
/// Values less than 0.0 will use 0.0.
pub fn set_volume(volume: f64) {
    // Map 0.0 - 1.0 to 0 - 128 (sdl2_mixer::MAX_VOLUME).
    mix::Music::set_volume((volume.max(MIN_VOLUME).min(MAX_VOLUME) * mix::MAX_VOLUME as f64) as
                           i32);
}

/// Plays a music track.
pub fn play<T: Eq + Hash + 'static + Any>(val: &T, repeat: Repeat) {
    let _ = unsafe { current_music_tracks::<T>() }
        .get(val)
        .expect("music: Attempted to play value that is not bound to asset")
        .play(repeat.to_sdl2_repeats());
}
