use crate::{department::*, prelude::*};
use std::sync::Weak;

#[derive(Default)]
pub struct SubjectBuilder {
  name: Option<String>,
  department_list: Vec<Weak<Department>>,
}

#[derive(Debug)]
pub struct Subject {
  pub name: String,
  pub department_list: Vec<Weak<Department>>,
}

impl SubjectBuilder {
  pub fn new() -> Self {
    Self::default()
  }
  pub fn name(self, new_name: impl Into<String>) -> Self {
    Self {
      name: Some(new_name.into()),
      ..self
    }
  }
  pub fn department(self, new_department: Weak<Department>) -> Self {
    let mut new_department_list = self.department_list;
    new_department_list.push(new_department);
    Self {
      department_list: new_department_list,
      ..self
    }
  }
  // #[target_feature(enable = "let-else")]
  pub fn build(self) -> Result<Subject> {
    let Some(name) = self.name else {
      return Err(Error::Generic("Cannot build subject without name".to_string()));
    };
    let department_list = self.department_list;
    if department_list.len() == 0 {
      return Err(Error::Generic(
        "Cannot build subject without any departments".to_string(),
      ));
    }
    Ok(Subject {
      name,
      department_list,
    })
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::sync::Arc;

  #[test]
  #[should_panic]
  fn create_subject_missing_all_fields() {
    let subject_builder = SubjectBuilder::new();
    let subject = subject_builder.build().unwrap();
  }

  #[test]
  #[should_panic]
  fn create_subject_missing_name() {
    let maths_department = Arc::new(Department {
      name: "Maths Department".to_string(),
      class_size: 30,
      class_count: 20,
    });

    let subject_builder = SubjectBuilder::new().department(Arc::downgrade(&maths_department));
    let subject = subject_builder.build().unwrap();
  }

  #[test]
  #[should_panic]
  fn create_subject_missing_department() {
    let subject_builder = SubjectBuilder::new().name("Calculus");
    let subject = subject_builder.build().unwrap();
  }

  #[test]
  fn create_subject() {
    let maths_department = Arc::new(Department {
      name: "Maths Department".to_string(),
      class_size: 30,
      class_count: 20,
    });

    let calculus_subject = Arc::new(
      SubjectBuilder::new()
        .name("Calculus")
        .department(Arc::downgrade(&maths_department))
        .build()
        .unwrap(),
    );

    assert_eq!(calculus_subject.department_list.len(), 1);
  }
}
