use std::sync::LazyLock;

struct MusicPlayer;

impl MusicPlayer {
    // Implement the play method that takes a song name and prints "Playing: <song>"
    fn play(&self, song: &str) {
        println!("Playing: {}", song);
    }
}

// Create a lazily-initialized static instance of MusicPlayer (a Singleton)
static PLAYER: LazyLock<MusicPlayer> = LazyLock::new(|| MusicPlayer);

fn main() {
    // Use the static instance to play "Song1.mp3"
    PLAYER.play("Song1.mp3");
}
