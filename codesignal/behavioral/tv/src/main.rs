mod command;
mod remote_control;
mod tv;
mod tv_change_channel_command;
mod tv_off_command;
mod tv_on_command;

use remote_control::RemoteControl;
use tv::TV;
use tv_change_channel_command::TVChangeChannelCommand;
use tv_off_command::TVOffCommand;
use tv_on_command::TVOnCommand;

fn main() {
    let mut tv = TV::new();
    let mut remote = RemoteControl::new();

    // TODO: Create and execute commands using the remote control
    // Example:
    // remote.press_button(Box::new(TVOnCommand::new()), &mut tv);
    // remote.press_button(Box::new(TVChangeChannelCommand::new(5)), &mut tv);
    // remote.press_button(Box::new(TVOffCommand::new()), &mut tv);

    remote.press_button(Box::new(TVOnCommand::new(tv.clone())), &mut tv);
    remote.press_button(Box::new(TVChangeChannelCommand::new(5)), &mut tv);
    remote.press_button(Box::new(TVOffCommand::new(tv.clone())), &mut tv);

    // TODO: Optionally, test undo functionality
    // Example:
    // remote.undo_button(&mut tv);
    remote.undo_button(&mut tv);
}
