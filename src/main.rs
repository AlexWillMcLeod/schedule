extern crate schedule;

use schedule::*;

fn main() {
  let mut high_school = Schedule::new(5);

  high_school
    .new_department("Maths Department", 30, 1)
    .unwrap();
  high_school
    .new_department("English Department", 30, 12)
    .unwrap();
  high_school
    .new_department("Computer Science Department", 30, 12)
    .unwrap();
  high_school
    .new_department("Science Department", 30, 12)
    .unwrap();

  high_school
    .new_department("Social Science Department", 30, 12)
    .unwrap();

  high_school
    .new_subject("Calculus", "Maths Department")
    .unwrap();

  high_school
    .new_subject("Statistics", "Maths Department")
    .unwrap();

  high_school
    .new_subject("English", "English Department")
    .unwrap();

  high_school
    .new_subject("Media Studies", "English Department")
    .unwrap();

  high_school
    .new_subject("Computer Science", "Computer Science Department")
    .unwrap();

  high_school
    .new_subject("Electronics", "Computer Science Department")
    .unwrap();

  high_school
    .new_subject("French", "English Department")
    .unwrap();

  high_school
    .new_subject("Spanish", "English Department")
    .unwrap();

  high_school
    .new_subject("Physics", "Science Department")
    .unwrap();

  high_school
    .new_subject("Chemistry", "Science Department")
    .unwrap();

  high_school
    .new_subject("Biology", "Science Department")
    .unwrap();

  high_school
    .new_subject("History", "Social Science Department")
    .unwrap();

  high_school
    .new_subject("Geography", "Social Science Department")
    .unwrap();

  high_school
    .new_subject("Economics", "Social Science Department")
    .unwrap();

  high_school
    .new_subject("Earth and Space Science", "Science Department")
    .unwrap();

  high_school
    .new_student_to_schedule(
      "Person",
      "One",
      "1",
      vec![
        String::from("Calculus"),
        String::from("Statistics"),
        String::from("English"),
        String::from("Physics"),
        String::from("Computer Science"),
      ],
    )
    .unwrap();

  high_school
    .new_student_to_schedule(
      "Person",
      "Two",
      "2",
      vec![
        String::from("Calculus"),
        String::from("Statistics"),
        String::from("Media Studies"),
        String::from("Geography"),
        String::from("Economics"),
      ],
    )
    .unwrap();

  high_school
    .new_student_to_schedule(
      "Person",
      "Three",
      "3",
      vec![
        String::from("Calculus"),
        String::from("English"),
        String::from("Media Studies"),
        String::from("French"),
        String::from("Economics"),
      ],
    )
    .unwrap();

  high_school
    .new_student_to_schedule(
      "Person",
      "Four",
      "4",
      vec![
        String::from("Computer Science"),
        String::from("English"),
        String::from("Media Studies"),
        String::from("French"),
        String::from("Geography"),
      ],
    )
    .unwrap();

  high_school
    .new_student_to_schedule(
      "Person",
      "Five",
      "5",
      vec![
        String::from("Computer Science"),
        String::from("English"),
        String::from("Calculus"),
        String::from("Statistics"),
        String::from("Economics"),
      ],
    )
    .unwrap();

  // high_school.new_class(0, "Calculus").unwrap();

  // high_school.new_class(1, "Statistics").unwrap();

  for (slot_id, class_list) in high_school.class_map.iter() {
    for class in class_list.iter() {
      let subject = class
        .lock()
        .unwrap()
        .subject
        .upgrade()
        .unwrap()
        .name
        .clone();
      let student_list = &class.lock().unwrap().student_list;
      let student_list: Vec<String> = student_list
        .iter()
        .map(|student| {
          let first_name = student.upgrade().unwrap().first_name.clone();
          let last_name = student.upgrade().unwrap().last_name.clone();
          vec![first_name, last_name].join(" ")
        })
        .collect();
      println!("{subject} is on in slot {slot_id} with {student_list:#?}");
    }
  }

  // println!("{:#?}", high_school);
}
