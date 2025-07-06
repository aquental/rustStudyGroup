mod music_player;
mod command;
mod play_music_command;
mod stop_music_command;
mod remote_control;

use music_player::MusicPlayer;
use play_music_command::PlayMusicCommand;
use stop_music_command::StopMusicCommand;
use remote_control::RemoteControl;

fn main() {
    let player = MusicPlayer::new();
    let play_music = PlayMusicCommand::new(player.clone());
    let stop_music = StopMusicCommand::new(player.clone());

    let mut remote = RemoteControl::new();
    remote.set_command(Box::new(play_music));
    remote.press_button();

    remote.set_command(Box::new(stop_music));
    remote.press_button();
}
