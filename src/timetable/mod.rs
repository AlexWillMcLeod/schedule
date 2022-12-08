mod class;
mod slot;

use std::sync::{Arc, Mutex, Weak};

use crate::{prelude::*, Student, Subject};
pub use class::Class;
pub use slot::Slot;

#[derive(Default, Clone)]
pub struct Timetable {
  pub slot_list: Vec<Arc<Mutex<Slot>>>,
}

impl Timetable {
  pub fn new(slot_count: usize) -> Self {
    let mut new_timetable = Self {
      slot_list: Vec::with_capacity(slot_count),
    };
    (0..slot_count).for_each(|_| {
      new_timetable
        .slot_list
        .push(Arc::new(Mutex::new(Slot::new())))
    });
    new_timetable
  }
  pub fn clear(&mut self) {
    *self = Self::new(self.slot_list.capacity());
  }
  // TODO! Returns the number of subjects that a student could not join
  pub fn add_student_to_timetable(&mut self, student: Weak<Student>) -> Result<usize> {
    let mut student_subjects = student.upgrade().unwrap().subject_list.clone();
    loop {
      let Ok(subject_added) =
      self.add_student_to_least_available(Weak::clone(&student), &student_subjects) else {
        return Ok(student_subjects.len())
      };
      student_subjects.remove(
        student_subjects
          .iter()
          .position(|x| subject_added.ptr_eq(x))
          .unwrap(),
      );
    }
  }

  pub fn add_student_to_least_available(
    &mut self,
    student: Weak<Student>,
    subject_list: &Vec<Weak<Subject>>,
  ) -> Result<Weak<Subject>> {
    if subject_list.len() == 0 {
      return Err(Error::Generic(
        "Student has no subjects to add them to".to_string(),
      ));
    }

    // Check that some subjects are available
    let Some(available_subject) = self.get_available_subject(Weak::clone(&student), &subject_list) else {
      // If none are available add an arbitrary one
      match self.add_student_to_subject(student, Weak::clone(&subject_list.get(0).unwrap())) {
        Ok(..) => {},
        Err(k) => return Err(k)
      }
      return Ok(Weak::clone(&subject_list.get(0).unwrap()));
    };

    // Find the least available subject that is still available
    let (mut least_available_subject, mut least_nonzero_available_slots) = (
      Weak::clone(&available_subject),
      self.available_slots(Weak::clone(&student), Weak::clone(&available_subject)),
    );

    for subject in subject_list {
      let available_slot_number =
        self.available_slots(Weak::clone(&student), Weak::clone(&subject));
      if available_slot_number == 0 {
        continue;
      } else if available_slot_number < least_nonzero_available_slots {
        (least_available_subject, least_nonzero_available_slots) =
          (Weak::clone(&subject), available_slot_number);
      }
    }

    // Add student to the least available subject that is still available
    self
      .add_student_to_subject(student, Weak::clone(&least_available_subject))
      .unwrap();

    Ok(least_available_subject)
  }

  pub fn get_available_subject(
    &self,
    student: Weak<Student>,
    subject_list: &Vec<Weak<Subject>>,
  ) -> Option<Weak<Subject>> {
    for subject in subject_list {
      if self.available_slots(Weak::clone(&student), Weak::clone(&subject)) != 0 {
        return Some(Weak::clone(subject));
      }
    }
    None
  }

  pub fn add_student_to_subject(
    &mut self,
    student: Weak<Student>,
    subject: Weak<Subject>,
  ) -> Result<()> {
    for slot in self.get_student_free_slots(Weak::clone(&student)) {
      if slot
        .upgrade()
        .unwrap()
        .lock()
        .unwrap()
        .contains_joinable_subject(Weak::clone(&subject))
      {
        return slot
          .upgrade()
          .unwrap()
          .lock()
          .unwrap()
          .add_student_to_subject(student, subject);
      }
    }
    self.create_class_for_student(student, subject)
  }

  pub fn create_class_for_student(
    &mut self,
    student: Weak<Student>,
    subject: Weak<Subject>,
  ) -> Result<()> {
    match self.get_student_free_slots(Weak::clone(&student)).get(0) {
      Some(slot) => slot
        .upgrade()
        .unwrap()
        .lock()
        .unwrap()
        .create_class_for_student(student, subject),
      None => Err(Error::Generic(
        "Student has no free slots available".to_string(),
      )),
    }
  }

  pub fn get_student_free_slots(&self, student: Weak<Student>) -> Vec<Weak<Mutex<Slot>>> {
    let mut free_slots = Vec::<Weak<Mutex<Slot>>>::new();
    for slot in &self.slot_list {
      if !slot.lock().unwrap().contains_student(Weak::clone(&student)) {
        free_slots.push(Arc::downgrade(&slot));
      }
    }
    free_slots
  }
  pub fn available_slots(&self, student: Weak<Student>, subject: Weak<Subject>) -> usize {
    let mut count = 0;
    for slot in self.get_student_free_slots(Weak::clone(&student)) {
      if slot
        .upgrade()
        .unwrap()
        .lock()
        .unwrap()
        .contains_joinable_subject(Weak::clone(&subject))
      {
        count += 1;
      }
    }
    count
  }
}

#[cfg(test)]
mod tests {
  #[test]
  fn test_get_free_slots() {}
}
