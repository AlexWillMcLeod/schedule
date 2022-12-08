use crate::prelude::*;

#[derive(Debug, PartialEq, Clone)]
pub struct Department {
  pub name: String,
  pub class_size: usize,
  pub class_count: usize,
}
