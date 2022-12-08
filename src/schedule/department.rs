#[derive(Debug, PartialEq, Clone)]
pub struct Department {
  pub name: String,
  pub min_class_size: usize,
  pub max_class_size: usize,
  pub class_count: usize,
}
