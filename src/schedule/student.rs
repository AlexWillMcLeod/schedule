use crate::{department::*, prelude::*, subject::*};
use std::sync::Weak;

#[derive(Default)]
pub struct StudentBuilder {
  first_name: Option<String>,
  last_name: Option<String>,
  id: Option<String>,
  subject_list: Vec<Weak<Subject>>,
}

#[derive(Debug)]
pub struct Student {
  pub first_name: String,
  pub last_name: String,
  pub id: String,
  pub subject_list: Vec<Weak<Subject>>,
}

impl StudentBuilder {
  pub fn new() -> Self {
    Self::default()
  }
  pub fn first_name(self, new_first_name: impl Into<String>) -> Self {
    Self {
      first_name: Some(new_first_name.into()),
      ..self
    }
  }
  pub fn last_name(self, new_last_name: impl Into<String>) -> Self {
    Self {
      last_name: Some(new_last_name.into()),
      ..self
    }
  }
  pub fn id(self, new_id: impl Into<String>) -> Self {
    Self {
      id: Some(new_id.into()),
      ..self
    }
  }
  pub fn subject(self, new_subject: Weak<Subject>) -> Self {
    let mut new_subject_list = self.subject_list;
    new_subject_list.push(new_subject);
    Self {
      subject_list: new_subject_list,
      ..self
    }
  }
  // #[target_feature(enable = "let-else")]
  pub fn build(self) -> Result<Student> {
    let Some(first_name) = self.first_name else {
      return Err(Error::Generic("Cannot build student without first name".to_string()));
    };
    let Some(last_name) = self.last_name else {
      return Err(Error::Generic("Cannot build student without last name".to_string()));
    };
    let Some(id) = self.id else {
      return Err(Error::Generic("Cannot build student without id".to_string()));
    };
    let subject_list = self.subject_list;
    Ok(Student {
      first_name,
      last_name,
      id,
      subject_list,
    })
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::sync::Arc;

  #[test]
  #[should_panic]
  fn create_student_missing_all_fields() {
    let student_builder = StudentBuilder::new();
    let student = student_builder.build().unwrap();
  }

  #[test]
  #[should_panic]
  fn create_student_missing_name() {
    let student_builder = StudentBuilder::new().id("123");
    let student = student_builder.build().unwrap();
  }

  #[test]
  #[should_panic]
  fn create_student_missing_id() {
    let student_builder = StudentBuilder::new().first_name("Person").last_name("One");
    let student = student_builder.build().unwrap();
  }

  #[test]
  #[should_panic]
  fn create_student_only_last_name() {
    let student_builder = StudentBuilder::new().last_name("One");
    let student = student_builder.build().unwrap();
  }

  #[test]
  #[should_panic]
  fn create_student_only_last_name_with_subjects() {
    let maths_department = Arc::new(Department {
      name: "Maths Department".to_string(),
      class_count: 10,
      class_size: 10,
    });

    let calculus_subject = Arc::new(
      SubjectBuilder::new()
        .name("Calculus")
        .department(Arc::downgrade(&maths_department))
        .build()
        .unwrap(),
    );

    let student_builder = StudentBuilder::new()
      .last_name("One")
      .subject(Arc::downgrade(&calculus_subject));
    let student = student_builder.build().unwrap();
  }

  #[test]
  fn create_student_no_subjects() {
    let student_builder = StudentBuilder::new()
      .first_name("Person")
      .last_name("One")
      .id("123");
    let student = student_builder.build().unwrap();
  }

  #[test]
  fn create_student_with_subjects() {
    let maths_department = Arc::new(Department {
      name: "Maths Department".to_string(),
      class_count: 10,
      class_size: 10,
    });

    let calculus_subject = Arc::new(
      SubjectBuilder::new()
        .name("Calculus")
        .department(Arc::downgrade(&maths_department))
        .build()
        .unwrap(),
    );

    let statistics_subject = Arc::new(
      SubjectBuilder::new()
        .name("Statistics")
        .department(Arc::downgrade(&maths_department))
        .build()
        .unwrap(),
    );

    let student_builder = StudentBuilder::new()
      .first_name("Person")
      .last_name("One")
      .id("123")
      .subject(Arc::downgrade(&calculus_subject))
      .subject(Arc::downgrade(&statistics_subject));
    let student = student_builder.build().unwrap();
    assert_eq!(student.subject_list.len(), 2);
  }
}
