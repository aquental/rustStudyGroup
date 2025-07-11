// Robot struct with robot_type and material properties
#[derive(Debug, PartialEq)]
struct Robot {
    robot_type: String,
    material: String,
}

impl Robot {
    fn new(robot_type: String, material: String) -> Self {
        Robot {
            robot_type,
            material,
        }
    }
}

// Builder for constructing Robot objects
struct RobotBuilder {
    robot_type: Option<String>,
    material: Option<String>,
}

impl RobotBuilder {
    fn new() -> Self {
        RobotBuilder {
            robot_type: None,
            material: None,
        }
    }

    fn set_robot_type(mut self, robot_type: &str) -> Self {
        self.robot_type = Some(robot_type.to_string());
        self
    }

    fn set_material(mut self, material: &str) -> Self {
        self.material = Some(material.to_string());
        self
    }

    fn build(self) -> Result<Robot, String> {
        let robot_type = self.robot_type.ok_or("robot_type is required")?;
        let material = self.material.ok_or("material is required")?;
        Ok(Robot::new(robot_type, material))
    }
}

fn main() {
    // Create a Robot with robot_type = "Android" and material = "Steel" using the builder
    let robot = RobotBuilder::new()
        .set_robot_type("Android")
        .set_material("Steel")
        .build()
        .expect("Failed to build robot");

    // Verify the properties
    println!(
        "Created robot: type = {}, material = {}",
        robot.robot_type, robot.material
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_robot_builder_success() {
        let robot = RobotBuilder::new()
            .set_robot_type("Android")
            .set_material("Steel")
            .build()
            .expect("Failed to build robot");

        assert_eq!(
            robot,
            Robot {
                robot_type: String::from("Android"),
                material: String::from("Steel"),
            }
        );
    }

    #[test]
    fn test_robot_builder_missing_robot_type() {
        let result = RobotBuilder::new().set_material("Steel").build();
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "robot_type is required");
    }

    #[test]
    fn test_robot_builder_missing_material() {
        let result = RobotBuilder::new().set_robot_type("Android").build();
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "material is required");
    }
}
