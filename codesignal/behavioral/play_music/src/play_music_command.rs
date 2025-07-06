use crate::{command::Command, music_player::MusicPlayer};

// TODO: Implement the PlayMusicCommand struct, implementing Command trait.
// - The struct should take a MusicPlayer object in the constructor.
// - Implement the execute method to play the MusicPlayer.
pub struct PlayMusicCommand {
    player: MusicPlayer,
}

impl PlayMusicCommand {
    pub fn new(player: MusicPlayer) -> Self {
        Self { player }
    }
}

impl Command for PlayMusicCommand {
    fn execute(&self) {
        self.player.play();
    }
}
