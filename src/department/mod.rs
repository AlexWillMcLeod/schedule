#[derive(Debug)]
pub struct Department {
  pub name: String,
  pub max_number: usize,
  pub class_size: usize,
}

impl Department {
  pub fn new(name: String, max_number: usize, class_size: usize) -> Department {
    Department {
      name,
      max_number,
      class_size,
    }
  }
}
