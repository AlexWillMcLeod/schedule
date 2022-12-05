use crate::Subject;
use std::sync::Weak;

#[derive(Debug)]
pub struct Student {
  pub first_name: String,
  pub last_name: String,
  pub student_id: String,
  pub subject_list: Vec<Weak<Subject>>,
}

impl Student {
  pub fn new(
    first_name: String,
    last_name: String,
    student_id: String,
    subject_list: Vec<Weak<Subject>>,
  ) -> Student {
    Student {
      first_name,
      last_name,
      student_id,
      subject_list,
    }
  }
}
