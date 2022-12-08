use crate::{prelude::*, subject, timetable::Class, Department, Student, Subject};
use std::sync::{Arc, Mutex, Weak};

#[derive(Default, Debug)]
pub struct Slot {
  pub class_list: Vec<Arc<Mutex<Class>>>,
}

impl Slot {
  pub fn new() -> Self {
    Self::default()
  }
  pub fn contains_student(&self, student: Weak<Student>) -> bool {
    for class in &self.class_list {
      if class.lock().unwrap().contains(Weak::clone(&student)) {
        return true;
      }
    }
    false
  }
  pub fn contains_subject(&self, subject: Weak<Subject>) -> bool {
    for class in &self.class_list {
      if class.lock().unwrap().subject.ptr_eq(&subject) {
        return true;
      }
    }
    false
  }
  pub fn contains_joinable_subject(&self, subject: Weak<Subject>) -> bool {
    for class in &self.class_list {
      let class = class.lock().unwrap();
      if !class.subject.ptr_eq(&subject) {
        continue;
      }
      let class_size = class.department.upgrade().unwrap().class_size;
      if class.student_list.len() >= class_size {
        continue;
      }
      return true;
    }
    false
  }

  pub fn add_student_to_subject(
    &mut self,
    student: Weak<Student>,
    subject: Weak<Subject>,
  ) -> Result<()> {
    for class in &self.class_list {
      let mut class = class.lock().unwrap();
      if !class.subject.ptr_eq(&subject) {
        continue;
      }
      let class_size = class.department.upgrade().unwrap().class_size;
      if class.student_list.len() >= class_size {
        continue;
      }
      class.student_list.push(Weak::clone(&student));
      return Ok(());
    }
    Err(Error::Generic(
      "Subject is not available in this slot".to_string(),
    ))
  }

  fn is_department_full(&self, department: Weak<Department>) -> bool {
    let class_count = department.upgrade().unwrap().class_count;
    let mut curr_class_count = 0;

    for class in &self.class_list {
      if class.lock().unwrap().department.ptr_eq(&department) {
        curr_class_count += 1;
      }
    }
    curr_class_count >= class_count
  }

  pub fn create_class_for_student(
    &mut self,
    student: Weak<Student>,
    subject: Weak<Subject>,
  ) -> Result<()> {
    // Find the first empty department for the subject
    for department in subject.upgrade().unwrap().department_list.clone() {
      if self.is_department_full(Weak::clone(&department)) {
        continue;
      }
      // Department is not full
      let new_class = Class {
        subject: Weak::clone(&subject),
        department,
        student_list: vec![Weak::clone(&student)],
      };
      self.class_list.push(Arc::new(Mutex::new(new_class)));
      return Ok(());
    }
    Err(Error::Generic("No classrooms left for subject".to_string()))
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::StudentBuilder;
  use crate::SubjectBuilder;

  #[test]
  fn test_is_department_full() {
    let department_one = Arc::new(Department {
      name: "Maths".to_string(),
      class_count: 30,
      class_size: 30,
    });
    let department_two = Arc::new(Department {
      name: "English".to_string(),
      class_count: 0,
      class_size: 30,
    });
    let slot = Slot::new();
    assert!(!slot.is_department_full(Arc::downgrade(&department_one)));
    assert!(slot.is_department_full(Arc::downgrade(&department_two)));
  }

  #[test]
  fn test_create_class_for_student() {
    let department = Arc::new(Department {
      name: "Maths".to_string(),
      class_count: 30,
      class_size: 30,
    });
    let subject = Arc::new(
      SubjectBuilder::new()
        .name("Calc")
        .department(Arc::downgrade(&department))
        .build()
        .unwrap(),
    );
    let student = Arc::new(
      StudentBuilder::new()
        .first_name("Person")
        .last_name("One")
        .id("1")
        .build()
        .unwrap(),
    );
    let mut slot = Slot::new();
    slot
      .create_class_for_student(Arc::downgrade(&student), Arc::downgrade(&subject))
      .unwrap();
  }

  #[test]
  #[should_panic]
  fn test_create_class_for_student_but_department_full() {
    let department = Arc::new(Department {
      name: "Maths".to_string(),
      class_count: 0,
      class_size: 30,
    });
    let subject = Arc::new(
      SubjectBuilder::new()
        .name("Calc")
        .department(Arc::downgrade(&department))
        .build()
        .unwrap(),
    );
    let student = Arc::new(
      StudentBuilder::new()
        .first_name("Person")
        .last_name("One")
        .id("1")
        .build()
        .unwrap(),
    );
    let mut slot = Slot::new();
    slot
      .create_class_for_student(Arc::downgrade(&student), Arc::downgrade(&subject))
      .unwrap();
  }

  #[test]
  fn test_create_class_for_student_but_only_one_department_full() {
    let department_one = Arc::new(Department {
      name: "Maths".to_string(),
      class_count: 0,
      class_size: 30,
    });
    let department_two = Arc::new(Department {
      name: "Science".to_string(),
      class_count: 30,
      class_size: 30,
    });
    let subject = Arc::new(
      SubjectBuilder::new()
        .name("Calc")
        .department(Arc::downgrade(&department_one))
        .department(Arc::downgrade(&department_two))
        .build()
        .unwrap(),
    );
    let student = Arc::new(
      StudentBuilder::new()
        .first_name("Person")
        .last_name("One")
        .id("1")
        .build()
        .unwrap(),
    );
    let mut slot = Slot::new();
    slot
      .create_class_for_student(Arc::downgrade(&student), Arc::downgrade(&subject))
      .unwrap();
  }

  #[test]
  fn test_add_student_to_subject() {
    let department = Arc::new(Department {
      name: "Maths".to_string(),
      class_count: 10,
      class_size: 30,
    });
    let subject = Arc::new(
      SubjectBuilder::new()
        .name("Calc")
        .department(Arc::downgrade(&department))
        .build()
        .unwrap(),
    );
    let student_one = Arc::new(
      StudentBuilder::new()
        .first_name("Person")
        .last_name("One")
        .id("1")
        .build()
        .unwrap(),
    );
    let student_two = Arc::new(
      StudentBuilder::new()
        .first_name("Person")
        .last_name("Two")
        .id("2")
        .build()
        .unwrap(),
    );
    let mut slot = Slot::new();
    slot
      .create_class_for_student(Arc::downgrade(&student_one), Arc::downgrade(&subject))
      .unwrap();
    slot
      .add_student_to_subject(Arc::downgrade(&student_two), Arc::downgrade(&subject))
      .unwrap();
    assert_eq!(slot.class_list.len(), 1);
    assert_eq!(
      slot
        .class_list
        .get(0)
        .unwrap()
        .lock()
        .unwrap()
        .student_list
        .len(),
      2
    );
    assert!(slot
      .class_list
      .get(0)
      .unwrap()
      .lock()
      .unwrap()
      .contains(Arc::downgrade(&student_one)));
    assert!(slot
      .class_list
      .get(0)
      .unwrap()
      .lock()
      .unwrap()
      .contains(Arc::downgrade(&student_two)));
  }

  #[test]
  fn test_contains_joinable_subject() {
    let department = Arc::new(Department {
      name: "Maths".to_string(),
      class_count: 10,
      class_size: 30,
    });
    let department_full = Arc::new(Department {
      name: "Science".to_string(),
      class_count: 0,
      class_size: 30,
    });
    let subject = Arc::new(
      SubjectBuilder::new()
        .name("Calc")
        .department(Arc::downgrade(&department))
        .build()
        .unwrap(),
    );
    let subject_full = Arc::new(
      SubjectBuilder::new()
        .name("Stats")
        .department(Arc::downgrade(&department_full))
        .build()
        .unwrap(),
    );
    let student_one = Arc::new(
      StudentBuilder::new()
        .first_name("Person")
        .last_name("One")
        .id("1")
        .build()
        .unwrap(),
    );
    let mut slot = Slot::new();
    assert!(!slot.contains_joinable_subject(Arc::downgrade(&subject)));
    assert!(!slot.contains_joinable_subject(Arc::downgrade(&subject_full)));
    slot
      .create_class_for_student(Arc::downgrade(&student_one), Arc::downgrade(&subject))
      .unwrap();
    assert!(slot.contains_joinable_subject(Arc::downgrade(&subject)));
  }

  #[test]
  fn test_contains_subject_and_student() {
    let department = Arc::new(Department {
      name: "Science".to_string(),
      class_count: 10,
      class_size: 30,
    });
    let subject = Arc::new(
      SubjectBuilder::new()
        .name("Calc")
        .department(Arc::downgrade(&department))
        .build()
        .unwrap(),
    );
    let student_one = Arc::new(
      StudentBuilder::new()
        .first_name("Person")
        .last_name("One")
        .id("1")
        .build()
        .unwrap(),
    );

    let mut slot = Slot::new();
    assert!(!slot.contains_subject(Arc::downgrade(&subject)));
    assert!(!slot.contains_student(Arc::downgrade(&student_one)));
    slot
      .create_class_for_student(Arc::downgrade(&student_one), Arc::downgrade(&subject))
      .unwrap();
    assert!(slot.contains_subject(Arc::downgrade(&subject)));
    assert!(slot.contains_student(Arc::downgrade(&student_one)));
  }
}
