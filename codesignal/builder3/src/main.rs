mod robot;
mod robot_builder;

use robot_builder::{RobotBuilder, SteelRobotBuilder};

fn main() {
    let mut builder = SteelRobotBuilder::new(); // Create a new robot builder

    let robot = builder
        .build_material() // Construct the robot step by step
        .build_type()
        .build_power_source()
        .build();

    robot.show_robot(); // Display the robot details
}
