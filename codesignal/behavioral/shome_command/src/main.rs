mod command;
mod device;
mod remote_control;
mod turn_off_command;
mod turn_on_command;

use device::{Fan, Light};
use remote_control::RemoteControl;
use turn_off_command::TurnOffCommand;
use turn_on_command::TurnOnCommand;

fn main() {
    // TODO: Create command objects
    let turn_on_command = TurnOnCommand::new();

    // TODO: Create a remote control
    let mut remote_control = RemoteControl::new();

    // TODO: Control devices using the RemoteControl
    remote_control.set_command(0, Box::new(turn_on_command), Box::new(Light::new()));
    remote_control.set_command(1, Box::new(TurnOffCommand::new()), Box::new(Light::new()));
    remote_control.set_command(2, Box::new(TurnOnCommand::new()), Box::new(Fan::new()));
    remote_control.set_command(3, Box::new(TurnOffCommand::new()), Box::new(Fan::new()));

    remote_control.press_button(0);
    remote_control.press_button(1);
    remote_control.press_button(2);
    remote_control.press_button(3);
}
