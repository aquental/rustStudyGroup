// Define the Device struct to represent a device
#[derive(Clone)]
struct Device {
    name: String,
}

impl Device {
    fn new(name: &str) -> Self {
        Device {
            name: name.to_string(),
        }
    }
}

// Define the Command trait for executing actions
trait Command {
    fn execute(&self);
}

// Concrete command for turning on the device
struct TurnOnCommand {
    device: Device,
}

impl TurnOnCommand {
    fn new(device: Device) -> Self {
        TurnOnCommand { device }
    }
}

impl Command for TurnOnCommand {
    fn execute(&self) {
        println!("Turning on device: {}", self.device.name);
    }
}

// Concrete command for turning off the device
struct TurnOffCommand {
    device: Device,
}

impl TurnOffCommand {
    fn new(device: Device) -> Self {
        TurnOffCommand { device }
    }
}

impl Command for TurnOffCommand {
    fn execute(&self) {
        println!("Turning off device: {}", self.device.name);
    }
}

// Controller to manage and execute commands
struct DeviceController {
    commands: Vec<Box<dyn Command>>,
}

impl DeviceController {
    fn new() -> Self {
        DeviceController {
            commands: Vec::new(),
        }
    }

    fn add_command(&mut self, command: Box<dyn Command>) {
        self.commands.push(command);
    }

    fn execute_all(&self) {
        for command in &self.commands {
            command.execute();
        }
    }
}

fn main() {
    // Create devices
    let light = Device::new("Light");
    let fan = Device::new("Fan");

    // Create a controller
    let mut controller = DeviceController::new();

    // Add commands to the controller
    controller.add_command(Box::new(TurnOnCommand::new(light.clone())));
    controller.add_command(Box::new(TurnOffCommand::new(light)));
    controller.add_command(Box::new(TurnOnCommand::new(fan.clone())));
    controller.add_command(Box::new(TurnOffCommand::new(fan)));

    // Execute all commands
    controller.execute_all();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_turn_on_command() {
        let device = Device::new("TestDevice");
        let command = TurnOnCommand::new(device);
        command.execute(); // Should print to stdout
    }

    #[test]
    fn test_turn_off_command() {
        let device = Device::new("TestDevice");
        let command = TurnOffCommand::new(device);
        command.execute(); // Should print to stdout
    }

    #[test]
    fn test_device_controller() {
        let device = Device::new("TestDevice");
        let mut controller = DeviceController::new();
        controller.add_command(Box::new(TurnOnCommand::new(device.clone())));
        controller.add_command(Box::new(TurnOffCommand::new(device)));
        controller.execute_all(); // Should print to stdout
    }
}
