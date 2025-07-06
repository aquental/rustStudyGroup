use crate::{command::Command, music_player::MusicPlayer};

// TODO: Implement the StopMusicCommand struct, implementing Command trait.
// - The struct should take a MusicPlayer object in the constructor.
// - Implement the execute method to stop the MusicPlayer.
pub struct StopMusicCommand {
    player: MusicPlayer,
}

impl StopMusicCommand {
    pub fn new(player: MusicPlayer) -> Self {
        Self { player }
    }
}

impl Command for StopMusicCommand {
    fn execute(&self) {
        self.player.stop();
    }
}
