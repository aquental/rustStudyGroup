mod tv;
mod command;
mod tv_on_command;
mod tv_off_command;
mod tv_change_channel_command;
mod remote_control;

use tv::TV;
use tv_on_command::TVOnCommand;
use tv_off_command::TVOffCommand;
use tv_change_channel_command::TVChangeChannelCommand;
use remote_control::RemoteControl;

fn main() {
    let mut tv = TV::new();
    let mut remote = RemoteControl::new();

    // TODO: Create and execute commands using the remote control
    // Example:
    // remote.press_button(Box::new(TVOnCommand::new()), &mut tv);
    // remote.press_button(Box::new(TVChangeChannelCommand::new(5)), &mut tv);
    // remote.press_button(Box::new(TVOffCommand::new()), &mut tv);

    // TODO: Optionally, test undo functionality
    // Example:
    // remote.undo_button(&mut tv);
}
