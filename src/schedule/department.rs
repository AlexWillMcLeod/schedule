use crate::prelude::*;

#[derive(Default)]
pub struct DepartmentBuilder {
  name: Option<String>,
  class_size: Option<usize>,
  class_count: Option<usize>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Department {
  pub name: String,
  class_size: usize,
  class_count: usize,
}

impl DepartmentBuilder {
  pub fn new() -> Self {
    Self::default()
  }
  pub fn name(self, new_name: impl Into<String>) -> Self {
    Self {
      name: Some(new_name.into()),
      ..self
    }
  }
  pub fn class_size(self, new_class_size: usize) -> Self {
    Self {
      class_size: Some(new_class_size),
      ..self
    }
  }
  pub fn class_count(self, new_class_count: usize) -> Self {
    Self {
      class_count: Some(new_class_count),
      ..self
    }
  }
  pub fn build(self) -> Result<Department> {
    let Some(name) = self.name else {
      return Err(Error::Generic("Cannot build department without name".to_string()));
    };
    let Some(class_size) = self.class_size else {
      return Err(Error::Generic("Cannot build department without class_size".to_string()));
    };
    let Some(class_count) = self.class_count else {
      return Err(Error::Generic("Cannot build department without class_count".to_string()));
    };
    Ok(Department {
      name,
      class_size,
      class_count,
    })
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  #[should_panic]
  fn create_department_missing_all_fields() {
    let department_builder = DepartmentBuilder::new();
    let department = department_builder.build().unwrap();
  }

  #[test]
  #[should_panic]
  fn create_department_missing_name() {
    let department_builder = DepartmentBuilder::new().class_count(21).class_size(11);
    let department = department_builder.build().unwrap();
  }

  #[test]
  #[should_panic]
  fn create_department_missing_class_count() {
    let department_builder = DepartmentBuilder::new()
      .name("Maths Department")
      .class_size(11);
    let department = department_builder.build().unwrap();
  }

  #[test]
  fn create_department() {
    let department_builder = DepartmentBuilder::new()
      .name("Maths Department")
      .class_size(11)
      .class_count(1);
    let department = department_builder.build().unwrap();
    assert_eq!(
      department,
      Department {
        name: "Maths Department".to_string(),
        class_size: 11,
        class_count: 1
      }
    );
  }
}
