#![allow(unused)] // For beginning only

extern crate schedule;

use schedule::prelude::*;
use schedule::{random, Schedule};

fn main() -> Result<()> {
  let mut high_school = random::random_schedule(2000, 30, 140, 20);

  high_school.sort();

  high_school
    .timetable
    .slot_list
    .iter()
    .enumerate()
    .for_each(|(i, x)| {
      println!("Slot: {}", i);
      x.lock().unwrap().class_list.iter().for_each(|x| {
        if !x.removed {
          println!(
            "  Class:\n    Subject: {}\n    Department: {}\n    Students:",
            x.subject.upgrade().unwrap().name,
            x.department.upgrade().unwrap().name,
          );
          x.student_list.iter().for_each(|student| {
            let student = student.upgrade().unwrap();
            println!(
              "      {} {} (id={})",
              student.first_name, student.last_name, student.id
            );
          });
        }
        println!("");
      })
    });

  // Get timetable for person four
  println!("\nPerson Four's Schedule:\n");
  high_school
    .timetable
    .slot_list
    .iter()
    .enumerate()
    .for_each(|(i, x)| {
      x.lock().unwrap().class_list.iter().for_each(|x| {
        x.student_list.iter().for_each(|student| {
          let student = student.upgrade().unwrap();
          if student.id == "4".to_string() {
            println!(
              "Slot {}: {} in the {} department",
              i,
              x.subject.upgrade().unwrap().name,
              x.department.upgrade().unwrap().name
            );
          }
        });
      })
    });

  Ok(())
}
