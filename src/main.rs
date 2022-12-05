extern crate schedule;

use schedule::*;

fn main() {
  let mut high_school = Schedule::new();

  high_school
    .new_department(String::from("Maths Department"), 30, 1)
    .unwrap();
  high_school
    .new_department(String::from("English Department"), 30, 12)
    .unwrap();

  high_school
    .new_subject(String::from("Calculus"), String::from("Maths Department"))
    .unwrap();

  high_school
    .new_subject(String::from("Statistics"), String::from("Maths Department"))
    .unwrap();

  high_school
    .new_subject(String::from("English"), String::from("English Department"))
    .unwrap();

  high_school
    .new_subject(
      String::from("Medka Studies"),
      String::from("English Department"),
    )
    .unwrap();

  high_school
    .new_student(
      String::from("Alex"),
      String::from("McLeod"),
      String::from("507036952"),
      vec![
        String::from("Calculus"),
        String::from("Statistics"),
        String::from("English"),
      ],
    )
    .unwrap();

  high_school.new_class(0, String::from("Calculus")).unwrap();

  high_school
    .new_class(0, String::from("Statistics"))
    .unwrap();

  println!("{:#?}", high_school);

  println!(
    "Alex would like to take {} in the {}",
    high_school
      .get_student_by_id(String::from("507036952"))
      .unwrap()
      .upgrade()
      .unwrap()
      .subject_list
      .get(2)
      .unwrap()
      .upgrade()
      .unwrap()
      .name,
    high_school
      .get_student_by_id(String::from("507036952"))
      .unwrap()
      .upgrade()
      .unwrap()
      .subject_list
      .get(2)
      .unwrap()
      .upgrade()
      .unwrap()
      .department
      .upgrade()
      .unwrap()
      .lock()
      .unwrap()
      .name
  )
}
