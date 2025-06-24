/**
Focus on the Single Responsibility Principle.
The code single_resp.rs manages a university system where the Course struct has multiple responsibilities, including managing course details and grading students.
Refactor the code so that each struct has a single responsibility, leading to more maintainable and understandable code.
Refactor the code to ensure that it adheres to the Single Responsibility Principle by distributing responsibilities across multiple structs. Specifically, split the grading functionality into a separate struct, such as a GradeManager, and remove the grading-related methods and data from the Course struct.
**/
use std::collections::HashMap;

// Course struct now only handles course details and student enrollment
struct Course {
    name: String,
    students: Vec<String>,
}

impl Course {
    fn new(name: &str) -> Course {
        Course {
            name: name.to_string(),
            students: Vec::new(),
        }
    }

    fn add_student(&mut self, student_name: &str) {
        self.students.push(student_name.to_string());
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_students(&self) -> &Vec<String> {
        &self.students
    }
}

// GradeManager struct handles all grading functionality
struct GradeManager {
    grades: HashMap<String, f64>,
}

impl GradeManager {
    fn new() -> GradeManager {
        GradeManager {
            grades: HashMap::new(),
        }
    }

    fn assign_grade(&mut self, student_name: &str, grade: f64) {
        self.grades.insert(student_name.to_string(), grade);
    }

    fn get_grade(&self, student_name: &str) -> Option<f64> {
        self.grades.get(student_name).copied()
    }

    fn print_student_grades(&self, course: &Course) {
        println!("Grades for course: {}", course.get_name());
        for student in course.get_students() {
            match self.get_grade(student) {
                Some(grade) => println!("Student: {}, Grade: {}", student, grade),
                None => println!("Student: {}, Grade: N/A", student),
            }
        }
    }
}

// TODO: Modify the main function to utilize your new GradeManager and test the refactored code.

fn main() {
    let mut course = Course::new("History 101");
    course.add_student("John Doe");
    course.add_student("Jane Smith");

    let mut grade_manager = GradeManager::new();
    grade_manager.assign_grade("John Doe", 85.5);
    grade_manager.assign_grade("Jane Smith", 92.0);

    println!("Course: {}", course.get_name());
    grade_manager.print_student_grades(&course);
}
