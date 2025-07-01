// TODO: Implement the Robot struct
// - Add public fields for the material, type, and power source of the robot
pub struct Robot {
    pub(crate) material: String,
    pub(crate) robot_type: String,
    pub(crate) power_source: String,
}

impl Robot {
    // TODO: Implement the show_robot method to display the robot's attributes
    // Example: "Robot made of $material, type $robot_type, powered by $power_source."
    pub fn show_robot(&self) {
        println!(
            "Robot made of {}, type {}, powered by {}",
            self.material, self.robot_type, self.power_source
        );
    }
}
