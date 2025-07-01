use crate::robot::Robot;

pub trait RobotBuilder {
    // TODO: Define methods for building each part of the robot:
    // - build_material
    // - build_type
    // - build_power_source
    fn build_material(&mut self) -> &mut Self;
    fn build_type(&mut self) -> &mut Self;
    fn build_power_source(&mut self) -> &mut Self;

    // TODO: Define a method build for returning the fully built Robot object
    fn build(&mut self) -> Robot;
}

// TODO: Implement the SteelRobotBuilder struct
// - Add fields for material, robot_type, and power_source
pub struct SteelRobotBuilder {
    material: String,
    robot_type: String,
    power_source: String,
}

impl SteelRobotBuilder {
    pub fn new() -> Self {
        // TODO: Initialize a new SteelRobotBuilder with empty strings
        SteelRobotBuilder {
            material: String::new(),
            robot_type: String::new(),
            power_source: String::new(),
        }
    }
}

impl RobotBuilder for SteelRobotBuilder {
    // TODO: Implement the build_material method to set the material to "Steel"
    fn build_material(&mut self) -> &mut Self {
        self.material = "Steel".to_string();
        self
    }

    // TODO: Implement the build_type method to set the type to "Warrior"
    fn build_type(&mut self) -> &mut Self {
        self.robot_type = "Warrior".to_string();
        self
    }

    // TODO: Implement the build_power_source method to set the power source to "Nuclear"
    fn build_power_source(&mut self) -> &mut Self {
        self.power_source = "Nuclear".to_string();
        self
    }

    // TODO: Implement the build method to return the fully constructed Robot object
    fn build(&mut self) -> Robot {
        Robot {
            material: self.material.clone(),
            robot_type: self.robot_type.clone(),
            power_source: self.power_source.clone(),
        }
    }
}
