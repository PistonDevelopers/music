extern crate piston_window;
extern crate music;

use piston_window::*;

#[derive(Copy, Clone, Hash, PartialEq, Eq)]
enum Music {
    Piano,
}

#[derive(Copy, Clone, Hash, PartialEq, Eq)]
enum Sound {
    Ding,
}

fn main() {
    let mut window: PistonWindow = WindowSettings::new("Test music", [640, 480])
        .exit_on_esc(true)
        .build()
        .unwrap();

    music::start::<Music, Sound, _>(None, || {
        music::bind_music_file(Music::Piano, "./assets/piano.mp3");
        music::bind_sound_file(Sound::Ding, "./assets/ding.mp3");

        music::play_music(&Music::Piano, music::Repeat::Forever);
        music::play_sound(&Sound::Ding, music::Repeat::Times(1));
        while let Some(e) = window.next() {
            window.draw_2d(&e, |_c, g| { clear([1.0; 4], g); });
        }
    });
}
