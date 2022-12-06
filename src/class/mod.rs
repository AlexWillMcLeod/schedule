use crate::{Department, Student, Subject};
use std::sync::{Mutex, Weak};

#[derive(Debug)]
pub struct Class {
  pub subject: Weak<Subject>,
  pub department: Weak<Mutex<Department>>,
  pub student_list: Vec<Weak<Student>>,
}

impl Class {
  pub fn new(subject: Weak<Subject>, department: Weak<Mutex<Department>>) -> Class {
    Class {
      subject,
      department,
      student_list: Vec::<Weak<Student>>::new(),
    }
  }
  pub fn add_student(&mut self, student: Weak<Student>) -> Result<(), &str> {
    if self.student_list.len()
      >= self
        .department
        .upgrade()
        .unwrap()
        .lock()
        .unwrap()
        .class_size
    {
      Err("Class is full")
    } else {
      self.student_list.push(student);
      Ok(())
    }
  }
  pub fn is_full(&self) -> bool {
    self.student_list.len()
      < self
        .department
        .upgrade()
        .unwrap()
        .lock()
        .unwrap()
        .class_size
  }
}
