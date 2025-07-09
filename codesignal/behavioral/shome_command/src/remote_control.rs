use crate::command::Command;
use crate::device::Device;

// TODO: Create the RemoteControl struct
// - Define a method press_button to execute a Command on a Device
pub struct RemoteControl {
    commands: Vec<Option<Box<dyn Command>>>,
    devices: Vec<Option<Box<dyn Device>>>,
}

impl RemoteControl {
    pub fn new() -> RemoteControl {
        RemoteControl {
            commands: (0..10).map(|_| None).collect(), // Support up to 10 commands
            devices: (0..10).map(|_| None).collect(), // Support up to 10 devices
        }
    }

    pub fn set_command(&mut self, slot: usize, command: Box<dyn Command>, device: Box<dyn Device>) {
        if slot < self.commands.len() {
            self.commands[slot] = Some(command);
            self.devices[slot] = Some(device);
        }
    }

    pub fn press_button(&mut self, slot: usize) {
        if let (Some(Some(command)), Some(Some(device))) = (self.commands.get(slot), self.devices.get_mut(slot)) {
            command.execute(device.as_mut());
        }
    }
}
