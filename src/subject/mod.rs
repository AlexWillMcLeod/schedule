use crate::Department;
use std::sync::{Mutex, Weak};

#[derive(Debug)]
pub struct Subject {
  pub name: String,
  pub department: Weak<Mutex<Department>>,
}

impl Subject {
  pub fn new(name: String, department: Weak<Mutex<Department>>) -> Subject {
    Subject { name, department }
  }
}
