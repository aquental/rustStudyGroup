mod command;
mod fan;
mod fan_high_command;
mod fan_low_command;
mod fan_medium_command;
mod fan_off_command;
mod remote_control;

use fan::Fan;
use fan_high_command::FanHighCommand;
use fan_low_command::FanLowCommand;
use fan_medium_command::FanMediumCommand;
use fan_off_command::FanOffCommand;
use remote_control::RemoteControl;

fn main() {
    // TODO: Test the Command pattern implementation
    // - Create a `Fan` object.
    // - Create a `RemoteControl` object.
    // - Use `RemoteControl` to execute commands to change the fan speed and turn it off.
    // - Implement undo functionality to revert the fan to its previous states.
}
