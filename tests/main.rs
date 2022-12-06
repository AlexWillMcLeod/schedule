mod tests {

  extern crate schedule;
  use schedule::*;

  #[test]
  fn create_high_school() {
    let mut high_school = Schedule::new();

    high_school
      .new_department("Maths Department", 30, 10)
      .unwrap();
    high_school
      .new_department("English Department", 30, 10)
      .unwrap();
    high_school
      .new_department("Science Department", 30, 10)
      .unwrap();
    high_school
      .new_department("Social Science Department", 30, 10)
      .unwrap();
    high_school.new_department("IT Department", 30, 10).unwrap();

    high_school
      .new_subject("Calculus", vec!["Maths Department"])
      .unwrap();
    high_school
      .new_subject("Statistics", vec!["Maths Department"])
      .unwrap();
    high_school
      .new_subject("Physics", vec!["Science Department"])
      .unwrap();
    high_school
      .new_subject("Computer Science", vec!["IT Department"])
      .unwrap();
    high_school
      .new_subject("Chemistry", vec!["Science Department"])
      .unwrap();

    high_school
      .new_student(
        "Person",
        "One",
        "1",
        vec![
          "Statistics",
          "Calculus",
          "Physics",
          "Computer Science",
          "Chemistry",
        ],
      )
      .unwrap();
  }

  #[test]
  fn sort_high_school() {
    let mut high_school = Schedule::new();

    high_school
      .new_department("Maths Department", 30, 10)
      .unwrap();
    high_school
      .new_department("English Department", 30, 10)
      .unwrap();
    high_school
      .new_department("Science Department", 30, 10)
      .unwrap();
    high_school
      .new_department("Social Science Department", 30, 10)
      .unwrap();
    high_school.new_department("IT Department", 30, 10).unwrap();

    high_school
      .new_subject("Calculus", vec!["Maths Department"])
      .unwrap();
    high_school
      .new_subject("Statistics", vec!["Maths Department"])
      .unwrap();
    high_school
      .new_subject("Physics", vec!["Science Department"])
      .unwrap();
    high_school
      .new_subject("Computer Science", vec!["IT Department"])
      .unwrap();
    high_school
      .new_subject("Chemistry", vec!["Science Department"])
      .unwrap();

    high_school
      .new_student(
        "Person",
        "One",
        "1",
        vec![
          "Statistics",
          "Calculus",
          "Physics",
          "Computer Science",
          "Chemistry",
        ],
      )
      .unwrap();

    high_school.sort();
  }
}
