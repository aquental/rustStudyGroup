pub trait Task {
    fn show_info(&self);
    fn get_name(&self) -> &str;
}

// TODO: Define the SimpleTask struct.
//       It should have fields description: String and duration: u32.
//       Implement the new method to initialize a SimpleTask with these fields.
//       Implement the Task trait for SimpleTask, including show_info() to output "<description> will take <duration> hours."
pub struct SimpleTask {
    description: String,
    duration: u32,
}

impl SimpleTask {
    pub fn new(description: &str, duration: u32) -> SimpleTask {
        SimpleTask {
            description: String::from(description),
            duration,
        }
    }
}
impl Task for SimpleTask {
    fn show_info(&self) {
        println!("{} will take {} hours.", self.description, self.duration);
    }
    fn get_name(&self) -> &str {
        "SimpleTask"
    }
}

// TODO: Define the Project struct.
//       It should maintain a vector of Box<dyn Task> objects.
//       Implement methods to add and remove tasks from this vector.
//       Implement the Task trait for Project, including show_info() to display details of all tasks within the project.
//       Note: for simplicity, get_name() can simply return a constant string "Project", it doesn't need to be a struct field.
pub struct Project {
    tasks: Vec<Box<dyn Task>>,
}
impl Project {
    pub fn new() -> Project {
        Project { tasks: Vec::new() }
    }
    pub fn add(&mut self, task: Box<dyn Task>) {
        self.tasks.push(task);
    }
    pub fn remove(&mut self, task_name: &str) {
        for task in self.tasks.iter_mut() {
            if task.get_name() == task_name {
                self.tasks.remove(
                    self.tasks
                        .iter()
                        .position(|t| t.get_name() == task_name)
                        .unwrap(),
                );
                break;
            }
        }
    }
}
impl Task for Project {
    fn show_info(&self) {
        for task in &self.tasks {
            task.show_info();
        }
    }
    fn get_name(&self) -> &str {
        "Project"
    }
}
