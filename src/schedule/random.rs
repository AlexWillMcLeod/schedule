use std::sync::Arc;

use rand::seq::SliceRandom;

use crate::{Department, Schedule, StudentBuilder, SubjectBuilder};

pub fn random_schedule(
  student_count: usize,
  subject_count: usize,
  classroom_count: usize,
  department_count: usize,
) -> Schedule {
  let mut random_schedule = Schedule::new();

  let department_class_count = classroom_count / department_count;

  (0..department_count).for_each(|i| {
    let department = Department {
      name: format!("Department {}", i),
      min_class_size: 20,
      max_class_size: 30,
      class_count: department_class_count,
    };
    random_schedule.add_department(department).unwrap();
  });

  (0..subject_count).for_each(|i| {
    let mut subject_builder = SubjectBuilder::new().name(format!("Subject {}", i));
    for department in random_schedule
      .department_list
      .choose_multiple(&mut rand::thread_rng(), 2)
    {
      subject_builder = subject_builder.department(Arc::downgrade(&department));
    }
    random_schedule
      .add_subject(subject_builder.build().unwrap())
      .unwrap();
  });

  (0..student_count).for_each(|i| {
    let mut student = StudentBuilder::new()
      .first_name("Person")
      .last_name(i.to_string())
      .id(i.to_string())
      .build()
      .unwrap();
    random_schedule
      .subject_list
      .choose_multiple(&mut rand::thread_rng(), 5)
      .for_each(|s| {
        student.subject_list.push(Arc::downgrade(&s));
      });
    random_schedule.add_student(student).unwrap();
  });

  random_schedule
}
