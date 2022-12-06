pub mod department;
pub mod student;
pub mod subject;

extern crate rand;

pub use department::{Department, DepartmentBuilder};
pub use student::{Student, StudentBuilder};
pub use subject::{Subject, SubjectBuilder};

use crate::{prelude::*, timetable::Timetable};
use std::sync::Arc;

#[derive(Default)]
pub struct Schedule {
  student_list: Vec<Arc<Student>>,
  subject_list: Vec<Arc<Subject>>,
  department_list: Vec<Arc<Department>>,
  timetable: Timetable,
}

impl Schedule {
  pub fn new() -> Self {
    Self::default()
  }

  fn department_name_in_use(&self, name: impl Into<String>) -> bool {
    let name = name.into();
    for element in &self.department_list {
      if element.name == name {
        return true;
      }
    }
    false
  }

  fn add_department(&mut self, department: Department) -> Result<()> {
    let name = &department.name;
    if self.department_name_in_use(name) {
      return Err(Error::Generic(format!(
        "Name ({}) is already in use by another department",
        name
      )));
    }
    self.department_list.push(Arc::new(department));
    Ok(())
  }

  pub fn new_department(
    &mut self,
    name: impl Into<String>,
    class_size: usize,
    class_count: usize,
  ) -> Result<()> {
    let department_builder = DepartmentBuilder::new()
      .name(name)
      .class_size(class_size)
      .class_count(class_count);
    let department = match department_builder.build() {
      Ok(k) => k,
      Err(k) => return Err(k),
    };
    self.add_department(department)
  }

  fn get_department(&self, name: impl Into<String>) -> Option<Arc<Department>> {
    let name = name.into();
    for element in &self.department_list {
      if element.name == name {
        return Some(Arc::clone(element));
      }
    }
    None
  }

  fn subject_name_in_use(&self, name: impl Into<String>) -> bool {
    let name = name.into();
    for element in &self.subject_list {
      if element.name == name {
        return true;
      }
    }
    false
  }

  fn add_subject(&mut self, subject: Subject) -> Result<()> {
    let name = &subject.name;
    if self.subject_name_in_use(name) {
      return Err(Error::Generic(format!(
        "Name ({}) is already in use by another subject",
        name
      )));
    }
    self.subject_list.push(Arc::new(subject));
    Ok(())
  }

  fn get_subject(&self, name: impl Into<String>) -> Option<Arc<Subject>> {
    let name = name.into();
    for element in &self.subject_list {
      if element.name == name {
        return Some(Arc::clone(element));
      }
    }
    None
  }

  pub fn new_subject(
    &mut self,
    name: impl Into<String>,
    departments: Vec<impl Into<String>>,
  ) -> Result<()> {
    let mut subject_builder = SubjectBuilder::new().name(name);

    for element in departments {
      let element = element.into();
      let department = match self.get_department(&element) {
        Some(k) => k,
        None => {
          return Err(Error::Generic(format!(
            "No department with name ({}) found",
            element
          )))
        }
      };
      subject_builder = subject_builder.department(Arc::downgrade(&department));
    }

    let subject = match subject_builder.build() {
      Ok(k) => k,
      Err(k) => return Err(k),
    };

    self.add_subject(subject)
  }

  fn student_id_in_use(&self, student_id: impl Into<String>) -> bool {
    let student_id = student_id.into();
    for element in &self.student_list {
      if element.id == student_id {
        return true;
      }
    }
    false
  }

  fn add_student(&mut self, student: Student) -> Result<()> {
    if self.student_id_in_use(&student.id) {
      return Err(Error::Generic(format!(
        "Student with id ({}) already exists",
        student.id.clone()
      )));
    };
    self.student_list.push(Arc::new(student));
    Ok(())
  }

  fn get_student(&self, student_id: impl Into<String>) -> Option<Arc<Student>> {
    let student_id = student_id.into();
    for element in &self.student_list {
      if element.id == student_id {
        return Some(Arc::clone(element));
      }
    }
    None
  }

  pub fn new_student(
    &mut self,
    first_name: impl Into<String>,
    last_name: impl Into<String>,
    id: impl Into<String>,
    subjects: Vec<impl Into<String>>,
  ) -> Result<()> {
    let first_name = first_name.into();
    let last_name = last_name.into();
    let id = id.into();
    let mut student_builder = StudentBuilder::new()
      .first_name(&first_name)
      .last_name(&last_name)
      .id(&id);
    for subject_name in subjects {
      let subject_name = subject_name.into();
      let element = match self.get_subject(&subject_name) {
        Some(k) => k,
        None => return Err(Error::Generic(format!("Tried adding subject ({}) to student (name: {} {}, id: {} ) when no department with that name exists", subject_name, first_name, last_name, id))),
      };

      student_builder = student_builder.subject(Arc::downgrade(&element));
    }
    let student = match student_builder.build() {
      Ok(k) => k,
      Err(k) => return Err(k),
    };
    self.add_student(student)
  }

  pub fn sort(&mut self) -> Result<()> {
    todo!()
  }
}

#[cfg(test)]
mod tests {
  use std::ops::Deref;

  use super::*;

  #[test]
  fn new_department() {
    let mut high_school = Schedule::new();

    high_school
      .new_department("Maths Department", 25, 10)
      .unwrap();
    high_school
      .new_department("English Department", 30, 10)
      .unwrap();
    high_school
      .new_department("Science Department", 30, 15)
      .unwrap();
    high_school
      .new_department("Social Science Department", 30, 10)
      .unwrap();
    high_school.new_department("IT Department", 30, 20).unwrap();

    let department_list: Vec<Department> = high_school
      .department_list
      .iter()
      .map(|x| x.deref().clone())
      .collect();

    assert_eq!(
      department_list,
      vec![
        DepartmentBuilder::new()
          .name("Maths Department")
          .class_size(25)
          .class_count(10)
          .build()
          .unwrap(),
        DepartmentBuilder::new()
          .name("English Department")
          .class_size(30)
          .class_count(10)
          .build()
          .unwrap(),
        DepartmentBuilder::new()
          .name("Science Department")
          .class_size(30)
          .class_count(15)
          .build()
          .unwrap(),
        DepartmentBuilder::new()
          .name("Social Science Department")
          .class_size(30)
          .class_count(10)
          .build()
          .unwrap(),
        DepartmentBuilder::new()
          .name("IT Department")
          .class_size(30)
          .class_count(20)
          .build()
          .unwrap(),
      ]
    )
  }

  #[test]
  fn get_department() {
    let mut high_school = Schedule::new();

    high_school
      .new_department("Computer Science Department", 25, 10)
      .unwrap();
    high_school
      .new_department("Maths Department", 30, 5)
      .unwrap();
    high_school
      .new_department("Science Department", 15, 8)
      .unwrap();

    let maths_department = high_school
      .get_department("Maths Department")
      .unwrap()
      .deref()
      .clone();

    assert_eq!(
      maths_department,
      DepartmentBuilder::new()
        .name("Maths Department")
        .class_size(30)
        .class_count(5)
        .build()
        .unwrap()
    );
  }

  #[test]
  fn new_subject() {
    let mut high_school = Schedule::new();
    high_school
      .new_department("Maths Department", 30, 10)
      .unwrap();
    high_school
      .new_subject("Maths", vec!["Maths Department"])
      .unwrap();

    let department = high_school
      .subject_list
      .get(0)
      .unwrap()
      .department_list
      .get(0)
      .unwrap()
      .upgrade()
      .unwrap()
      .deref()
      .clone();

    assert_eq!(
      department,
      DepartmentBuilder::new()
        .name("Maths Department")
        .class_size(30)
        .class_count(10)
        .build()
        .unwrap()
    );
  }

  #[test]
  #[should_panic]
  fn new_subject_wrong_assertion() {
    let mut high_school = Schedule::new();
    high_school
      .new_department("Maths Department", 30, 10)
      .unwrap();
    high_school
      .new_subject("Maths", vec!["Maths Department"])
      .unwrap();

    let department = high_school
      .subject_list
      .get(0)
      .unwrap()
      .department_list
      .get(0)
      .unwrap()
      .upgrade()
      .unwrap()
      .deref()
      .clone();

    assert_eq!(
      department,
      DepartmentBuilder::new()
        .name("Maths Department")
        .class_size(30)
        .class_count(25)
        .build()
        .unwrap()
    );
  }

  #[test]
  fn get_subject() {
    let mut high_school = Schedule::new();

    high_school
      .new_department("Maths Department", 30, 10)
      .unwrap();

    high_school
      .new_subject("Calculus", vec!["Maths Department"])
      .unwrap();

    let calculus = high_school.get_subject("Calculus").unwrap();
  }

  #[test]
  fn new_student() {
    let mut high_school = Schedule::new();

    high_school
      .new_department("Computer Science Department", 30, 10)
      .unwrap();

    high_school
      .new_subject("Computer Science", vec!["Computer Science Department"])
      .unwrap();

    high_school
      .new_student("Person", "One", "1", vec!["Computer Science"])
      .unwrap();
  }

  #[test]
  fn get_student() {
    let mut high_school = Schedule::new();

    high_school
      .new_department("Computer Science Department", 30, 10)
      .unwrap();

    high_school
      .new_subject("Computer Science", vec!["Computer Science Department"])
      .unwrap();

    high_school
      .new_student("Person", "One", "1", vec!["Computer Science"])
      .unwrap();

    high_school
      .new_student("Person", "Two", "2", vec!["Computer Science"])
      .unwrap();

    high_school
      .new_student("Person", "Three", "3", vec!["Computer Science"])
      .unwrap();

    let last_name = high_school.get_student("2").unwrap().last_name.clone();
    assert_eq!(last_name, "Two".to_string());
  }

  #[test]
  #[should_panic]
  fn new_department_same_name() {
    let mut high_school = Schedule::new();

    high_school.new_department("Math", 30, 10).unwrap();
    high_school.new_department("Math", 30, 10).unwrap();
  }

  #[test]
  #[should_panic]
  fn new_subject_same_name() {
    let mut high_school = Schedule::new();

    high_school.new_department("Math", 30, 10).unwrap();

    high_school.new_subject("Calculus", vec!["Math"]).unwrap();
    high_school.new_subject("Calculus", vec!["Math"]).unwrap();
  }

  #[test]
  #[should_panic]
  fn new_student_same_id() {
    let mut high_school = Schedule::new();

    high_school.new_department("Math", 30, 10).unwrap();

    high_school.new_subject("Calculus", vec!["Math"]).unwrap();

    high_school
      .new_student("Person", "One", "1", vec!["Math"])
      .unwrap();
    high_school
      .new_student("Person", "One", "1", vec!["Math"])
      .unwrap();
  }

  #[test]
  fn new_student_no_subjects() {
    let mut high_school = Schedule::new();

    high_school
      .new_student("Person", "One", "1", Vec::<String>::new())
      .unwrap();
  }

  #[test]
  #[should_panic]
  fn new_subject_no_departments() {
    let mut high_school = Schedule::new();

    high_school
      .new_subject("Maths", Vec::<String>::new())
      .unwrap();
  }
}
