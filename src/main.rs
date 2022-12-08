#![allow(unused)] // For beginning only

extern crate schedule;

use schedule::prelude::*;
use schedule::Schedule;

fn main() -> Result<()> {
  let mut high_school = Schedule::new();
  high_school.new_department("Maths", 30, 100);
  high_school.new_department("IT", 30, 100);
  high_school.new_department("English", 30, 100);
  high_school.new_department("Social Science", 30, 100);
  high_school.new_department("Science", 30, 100);

  high_school.new_subject("Calc", vec!["Maths"]);
  high_school.new_subject("Stats", vec!["Maths"]);
  high_school.new_subject("Computer Science", vec!["IT"]);
  high_school.new_subject("Design", vec!["IT"]);
  high_school.new_subject("English", vec!["English"]);
  high_school.new_subject("French", vec!["English"]);
  high_school.new_subject("Spanish", vec!["English"]);
  high_school.new_subject("History", vec!["Social Science"]);
  high_school.new_subject("Geography", vec!["Social Science"]);
  high_school.new_subject("Economics", vec!["Social Science"]);
  high_school.new_subject("Physics", vec!["Science"]);
  high_school.new_subject("Biology", vec!["Science"]);
  high_school.new_subject("Astronomy", vec!["Science"]);
  high_school.new_subject("Chemistry", vec!["Science"]);

  high_school.new_student(
    "Person",
    "One",
    "1",
    vec!["Calc", "Stats", "Computer Science", "English", "Physics"],
  );
  high_school.new_student(
    "Person",
    "Two",
    "2",
    vec!["Calc", "Stats", "French", "Spanish", "English"],
  );
  high_school.new_student(
    "Person",
    "Three",
    "3",
    vec!["Stats", "Calc", "Physics", "Astronomy", "Chemistry"],
  );
  high_school.new_student(
    "Person",
    "Four",
    "4",
    vec!["Physics", "Biology", "Chemistry", "History", "Geography"],
  );

  high_school.sort();
  high_school
    .timetable
    .slot_list
    .iter()
    .enumerate()
    .for_each(|(i, x)| {
      println!("Slot: {}", i);
      x.lock().unwrap().class_list.iter().for_each(|x| {
        let x = x.lock().unwrap();
        println!(
          "  Class:\n    Subject: {}\n    Department: {}\n    Students:",
          x.subject.upgrade().unwrap().name,
          x.department.upgrade().unwrap().name
        );
        x.student_list.iter().for_each(|student| {
          let student = student.upgrade().unwrap();
          println!(
            "      {} {} (id={})",
            student.first_name, student.last_name, student.id
          );
        });
        println!("");
      })
    });
  Ok(())
}
