extern crate piston_window;
extern crate music;
extern crate sdl2_window;

use piston_window::*;
use sdl2_window::Sdl2Window;

#[derive(Copy, Clone, Hash, PartialEq, Eq)]
enum Music {
    Piano,
}

#[derive(Copy, Clone, Hash, PartialEq, Eq)]
enum Sound {
    Ding,
}

fn main() {
    let mut window: PistonWindow<Sdl2Window> = WindowSettings::new("Test music", [640, 480])
        .exit_on_esc(true)
        .build()
        .unwrap();

    // sdl context for audio
    let sdl = window.window.sdl_context.to_owned();

    music::start::<Music, Sound, _>(Option::from(sdl), || {
        music::bind_music_file(Music::Piano, "./assets/piano.wav");
        music::bind_sound_file(Sound::Ding, "./assets/ding.wav");

        music::set_volume(music::MAX_VOLUME);
        music::play_music(&Music::Piano, music::Repeat::Forever);
        music::play_sound(&Sound::Ding, music::Repeat::Times(1));
        while let Some(e) = window.next() {
            window.draw_2d(&e, |_c, g| { clear([1.0; 4], g); });
        }
    });
}
