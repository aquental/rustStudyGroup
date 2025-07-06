mod command;
mod fan;
mod fan_off_command;
mod fan_on_command;
mod remote_control;

use fan::Fan;
use fan_off_command::FanOffCommand;
use fan_on_command::FanOnCommand;
use remote_control::RemoteControl;

fn main() {
    // TODO: Create an instance of the Fan struct.
    let mut fan = Fan::new();

    // TODO: Create an instance of the RemoteControl struct.
    let mut remote_control = RemoteControl::new();

    // TODO: Use the RemoteControl instance to press buttons for FanOnCommand and FanOffCommand, with the Fan instance.
    remote_control.press_button(Box::new(FanOnCommand::new(fan.clone())), &mut fan);
    remote_control.press_button(Box::new(FanOffCommand::new(fan.clone())), &mut fan);

    // TODO: Use the RemoteControl instance to undo commands.
    remote_control.undo_button(&mut fan);
}
