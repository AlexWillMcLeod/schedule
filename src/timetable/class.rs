use crate::student::Student;
use crate::{department::Department, prelude::*, subject::Subject};
use std::sync::{Arc, Weak};

pub struct Class {
  subject: Weak<Subject>,
  department: Weak<Department>,
  student_list: Vec<Weak<Student>>,
}
