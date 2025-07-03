mod task_system;

use task_system::{SimpleTask, Project, Task};

fn main() {
    let task1 = SimpleTask::new("Design", 5);
    let task2 = SimpleTask::new("Implementation", 10);

    let mut project = Project::new();
    project.add(Box::new(task1));
    project.add(Box::new(task2));

    println!("Project tasks after adding two tasks:");
    project.show_info();

    project.remove("Design");

    println!("Project tasks after removing one task:");
    project.show_info();
}
