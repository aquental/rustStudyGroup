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
    let mut fan = Fan::new();
    let mut remote = RemoteControl::new();

    remote.set_command(Box::new(FanOnCommand::new()));
    remote.press_button(&mut fan);
    remote.set_command(Box::new(FanOffCommand::new()));
    remote.press_button(&mut fan);
}
