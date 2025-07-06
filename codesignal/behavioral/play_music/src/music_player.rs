#[derive(Clone)]
pub struct MusicPlayer;

impl MusicPlayer {
    pub fn new() -> Self {
        MusicPlayer
    }

    pub fn play(&self) {
        println!("Music Player is playing.");
    }

    pub fn stop(&self) {
        println!("Music Player is stopped.");
    }
}
