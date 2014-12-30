extern crate piston;
extern crate music;

#[deriving(Copy, Hash, PartialEq, Eq)]
enum Music {
    Piano,
}

fn main() {
    piston::start(
        piston::shader_version::OpenGL::_3_2,
        piston::event::WindowSettings::default(),
        || {
        music::start::<Music>(|| {
            music::bind_file(Music::Piano, "./assets/piano.wav");
            music::play(&Music::Piano, music::Repeat::Forever);
            
            for _e in piston::events() {}
        });
    });
}

