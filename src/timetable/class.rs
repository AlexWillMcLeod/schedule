use crate::{prelude::*, Department, Student, Subject, SubjectBuilder};
use std::sync::{Arc, Weak};

#[derive(Debug)]
pub struct Class {
  pub subject: Weak<Subject>,
  pub department: Weak<Department>,
  pub student_list: Vec<Weak<Student>>,
}

impl Class {
  pub fn contains(&self, student: Weak<Student>) -> bool {
    for other in &self.student_list {
      if other.ptr_eq(&student) {
        return true;
      }
    }
    false
  }
}

#[cfg(test)]
mod tests {
  use crate::{StudentBuilder, SubjectBuilder};

  use super::*;

  #[test]
  fn test_contains() {
    let department = Arc::new(Department {
      name: "Maths".to_string(),
      class_count: 30,
      class_size: 30,
    });
    let subject = Arc::new(
      SubjectBuilder::new()
        .name("Calc")
        .department(Arc::downgrade(&department))
        .build()
        .unwrap(),
    );
    let student = Arc::new(
      StudentBuilder::new()
        .first_name("Person")
        .last_name("One")
        .id("1")
        .build()
        .unwrap(),
    );
    let student_two = Arc::new(
      StudentBuilder::new()
        .first_name("Person")
        .last_name("Two")
        .id("2")
        .build()
        .unwrap(),
    );
    let class = Class {
      subject: Arc::downgrade(&subject),
      department: Arc::downgrade(&department),
      student_list: vec![Arc::downgrade(&student)],
    };
    assert!(class.contains(Arc::downgrade(&student)));
    assert!(!class.contains(Arc::downgrade(&student_two)));
  }
}
