#![deny(missing_docs)]

//! A high level library for playing music

extern crate current;
extern crate sdl2;

use sdl2::mixer;
use current::{Current, CurrentGuard};
use std::collections::HashMap;
use std::hash::Hash;
use std::any::Any;
use std::path::Path;

/// Minimum value for playback volume parameter.
pub const MIN_VOLUME: f64 = 0.0;

/// Maximum value for playback volume parameter.
pub const MAX_VOLUME: f64 = 1.0;

/// Initializes the audio mixer and allocates the number of concurrent sound channels.
fn init_audio(num_sound_channels: i32) {
    // Load dynamic libraries.
    // Ignore formats that are not built in.
    let _ = mixer::init(mixer::InitFlag::MP3 | mixer::InitFlag::FLAC | mixer::InitFlag::MOD |
                        mixer::InitFlag::OGG);
    mixer::open_audio(mixer::DEFAULT_FREQUENCY,
                      mixer::DEFAULT_FORMAT,
                      mixer::DEFAULT_CHANNELS,
                      1024)
            .unwrap();
    // Sets the number of simultaneous sound effects channels
    // that are available.
    mixer::allocate_channels(num_sound_channels);
}

unsafe fn current_music_tracks<T: 'static + Any>() -> Current<HashMap<T, mixer::Music<'static>>> {
    Current::new()
}

unsafe fn current_sound_tracks<T: 'static + Any>() -> Current<HashMap<T, mixer::Chunk>> {
    Current::new()
}

/// Creates SDL context and starts the audio context
///
/// * `num_sound_channels`: The number of concurrent sound channels to allocate. This limits
///  the number of sounds that can be played simultaneously.
pub fn start<M: Eq + Hash + 'static + Any, S: Eq + Hash + 'static + Any, F: FnOnce()>
    (num_sound_channels: i32, f: F) {
    let sdl = sdl2::init().unwrap();
    start_context::<M, S, _>(&sdl, num_sound_channels, f);
    drop(sdl);
}

/// Initializes audio and sets up current objects
///
/// * `num_sound_channels`: The number of concurrent sound channels to allocate. This limits
///  the number of sounds that can be played simultaneously.
pub fn start_context<M: Eq + Hash + 'static + Any, S: Eq + Hash + 'static + Any, F: FnOnce()>
    (sdl: &sdl2::Sdl, num_sound_channels: i32, f: F){

    let audio = sdl.audio().unwrap();
    let timer = sdl.timer().unwrap();

    init_audio(num_sound_channels);
    let mut music_tracks: HashMap<M, mixer::Music> = HashMap::new();
    let music_tracks_guard = CurrentGuard::new(&mut music_tracks);

    let mut sound_tracks: HashMap<S, mixer::Chunk> = HashMap::new();
    let sound_tracks_guard = CurrentGuard::new(&mut sound_tracks);

    f();

    drop(sound_tracks_guard);
    drop(music_tracks_guard);
    drop(timer);
    drop(audio);
}

/// Binds a music file to value.
pub fn bind_music_file<T, P>(val: T, file: P)
    where T: 'static + Eq + Hash + Any,
          P: AsRef<Path>
{
    let track = mixer::Music::from_file(file.as_ref()).unwrap();
    unsafe { current_music_tracks() }.insert(val, track);
}

/// Binds a sound file to value.
pub fn bind_sound_file<T, P>(val: T, file: P)
    where T: 'static + Eq + Hash + Any,
          P: AsRef<Path>
{
    let track = mixer::Chunk::from_file(file.as_ref()).unwrap();
    unsafe { current_sound_tracks() }.insert(val, track);
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
    return mixer::Music::set_volume(to_sdl2_volume(volume));
}

/// Converts from piston_music volume representation to SDL2 representation.
///
/// Map 0.0 - 1.0 to 0 - 128 (sdl2::mixer::MAX_VOLUME).
fn to_sdl2_volume(volume: f64) -> i32 {
    (volume.max(MIN_VOLUME).min(MAX_VOLUME) * mixer::MAX_VOLUME as f64) as i32
}

/// Plays a music track.
pub fn play_music<T: Eq + Hash + 'static + Any>(val: &T, repeat: Repeat) {
    let _ = unsafe { current_music_tracks::<T>() }
        .get(val)
        .expect("music: Attempted to play value that is not bound to asset")
        .play(repeat.to_sdl2_repeats());
}

/// Plays a sound effect track.
///
/// The volume is set on a scale of 0.0 to 1.0, which means 0-100%.
/// Values greater than 1.0 will use 1.0.
/// Values less than 0.0 will use 0.0.
pub fn play_sound<T: Eq + Hash + 'static + Any>(val: &T, repeat: Repeat, volume: f64) {
    let channel = sdl2::mixer::Channel::all();
    channel.set_volume(to_sdl2_volume(volume));
    unsafe {
        channel
            .play(current_sound_tracks::<T>()
                      .get(val)
                      .expect("music: Attempted to play value that is not bound to asset"),
                  repeat.to_sdl2_repeats())
            .unwrap();
    }
}
