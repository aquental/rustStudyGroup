#![allow(dead_code)]

use crate::models::{Teachable, Discountable, OnlineCourse, PremiumCourse, EngagingCourse};
use std::fmt::{Debug, Display};

pub fn format_teachable<T: Teachable>(t: &T) -> String {
    format!(
        "Course: {} by {}, mode: {}, price: {}",
        t.title(),
        t.instructor(),
        t.teach_mode(),
        t.price()
    )
}

pub fn format_discounted<T: Teachable + Discountable>(t: &T) -> String {
    format!(
        "{} discounted: {}",
        t.title(),
        t.discount_price(20.0)
    )
}

pub fn print_course_comparison 
(
    a: &(impl Teachable + Debug),
    b: &(impl Teachable + Debug),
    c: &(impl Teachable + Discountable + Debug + Display),
) {
    println!("{:?} vs {:?} and {}", a, b, c);
}

// Trainig comparisons

#[derive(Debug)]
pub struct TrainingPair<T> {
    first: T,
    second: T,
}

impl<T> TrainingPair<T> {
    fn new(first: T, second: T) -> Self {
        Self { first, second }
    }
}

impl<T> TrainingPair<T>
where
    T: Teachable + Display + PartialOrd,
{
    fn cmp_display(&self) {
        if self.first.price() >= self.second.price() {
            println!("The more expensive training is {}", self.first.title());
        } else {
            println!("The more expensive training is {}", self.second.title());
        }
    }
}

fn make_online_course() -> impl Teachable {
    OnlineCourse {
        title: "Rust for Beginners".into(),
        instructor: "Alice".into(),
        url: "https://example.com".into(),
        price: 99.0,
    }
}

// fn make_course(kind: &str) -> impl Box<dyn Teachable> {
//     if kind == "online" {
//         OnlineCourse { ... }  // type A
//     } else {
//         InPersonWorkshop { ... } // type B
//     }
// }

fn discounted_course() -> impl Teachable + Discountable {
    OnlineCourse {
        title: "Advanced Rust Patterns".into(),
        instructor: "Bob".into(),
        url: "https://example.com/advanced".into(),
        price: 200.0,
    }
}

fn highlight_in_catalog<T: PremiumCourse>(_course: &T) {
    println!("*** Featured Premium Course ***");
}

fn promote<T: EngagingCourse>(_course: &T) {
    println!("Promoting an engaging course!");
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::OnlineCourse;

    #[test]
    fn test_training_pair() {
        let rust101 = OnlineCourse {
            title: "Rust 101".into(),
            instructor: "Alice".into(),
            url: "https://pachadata.example/rust101".into(),
            price: 100.0,
        };

        let rust_advanced = OnlineCourse {
            title: "Advanced Rust".into(),
            instructor: "Bob".into(),
            url: "https://pachadata.example/rust-adv".into(),
            price: 200.0,
        };

        let pair = TrainingPair::new(rust101, rust_advanced);

        // Test cmp_display
        pair.cmp_display();
    }

    #[test]
    fn test_engaging_course() {
        let rust101 = OnlineCourse {
            title: "Rust 101".into(),
            instructor: "Alice".into(),
            url: "https://pachadata.example/rust101".into(),
            price: 100.0,
        };

        promote(&rust101)
    }
}

