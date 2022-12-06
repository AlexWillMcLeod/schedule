mod class;
mod department;
mod student;
mod subject;

use std::{
  collections::hash_map::Entry,
  collections::HashMap,
  ops::Deref,
  sync::{Arc, Mutex, Weak},
};

use crate::class::Class;
use crate::department::Department;
use crate::student::Student;
use crate::subject::Subject;

#[derive(Debug, Clone)]
pub struct Schedule {
  pub department_list: Vec<Arc<Mutex<Department>>>,
  pub subject_list: Vec<Arc<Subject>>,
  pub student_list: Vec<Arc<Student>>,
  pub class_map: HashMap<usize, Vec<Arc<Mutex<Class>>>>,
}

impl Schedule {
  pub fn new(slot_count: usize) -> Schedule {
    Schedule {
      department_list: Vec::<Arc<Mutex<Department>>>::new(),
      subject_list: Vec::<Arc<Subject>>::new(),
      student_list: Vec::<Arc<Student>>::new(),
      class_map: Self::create_class_map(slot_count),
    }
  }
  pub fn create_class_map(slot_count: usize) -> HashMap<usize, Vec<Arc<Mutex<Class>>>> {
    let mut class_map = HashMap::<usize, Vec<Arc<Mutex<Class>>>>::new();
    (0..slot_count).for_each(|x| {
      class_map.insert(x, vec![]);
    });
    class_map
  }
  pub fn add_department(&mut self, department: Arc<Mutex<Department>>) -> Result<(), &str> {
    // Check that no department has the same name
    let department_name = {
      let department_guard = department.lock().unwrap();
      &department_guard.name.clone()[..]
    };

    for element in &self.department_list {
      let element_name = &element.deref().lock().unwrap().name[..];
      if element_name == department_name {
        return Err("Department with that name already exists");
      }
    }

    self.department_list.push(department);
    Ok(())
  }
  pub fn get_department(
    &mut self,
    department_name: impl Into<String>,
  ) -> Option<Weak<Mutex<Department>>> {
    let department_name = department_name.into();
    for element in &self.department_list {
      let element_name = &element.lock().unwrap().name[..];
      if element_name == department_name {
        return Some(Arc::downgrade(element));
      }
    }
    None
  }
  pub fn new_department(
    &mut self,
    name: impl Into<String>,
    class_size: usize,
    max_number: usize,
  ) -> Result<(), &str> {
    self.add_department(Arc::new(Mutex::new(Department::new(
      name.into(),
      max_number,
      class_size,
    ))))
  }
  pub fn add_subject(&mut self, subject: Arc<Subject>) -> Result<(), &str> {
    // Check that no subject has the same name
    let subject_name = &subject.deref().name[..];

    for element in &self.subject_list {
      let element_name = &element.deref().name[..];
      if element_name == subject_name {
        return Err("Subject with that name already exists");
      }
    }

    self.subject_list.push(subject);
    Ok(())
  }
  pub fn get_subject(&mut self, subject_name: impl Into<String>) -> Option<Weak<Subject>> {
    let subject_name = subject_name.into();
    for element in &self.subject_list {
      let element_name = &element.deref().name[..];
      if element_name == subject_name {
        return Some(Arc::downgrade(&element));
      }
    }
    None
  }
  pub fn new_subject(
    &mut self,
    name: impl Into<String>,
    department_name: impl Into<String>,
  ) -> Result<(), &str> {
    let name = name.into();
    let department_name = department_name.into();
    let department = match self.get_department(department_name) {
      Some(k) => k,
      None => return Err("Department not found"),
    };
    self.add_subject(Arc::new(Subject::new(name, department)))
  }
  pub fn add_student(&mut self, student: Arc<Student>) -> Result<(), &str> {
    // Check that no subject has the same name
    let (student_first_name, student_last_name, student_id) = (
      &student.deref().first_name[..],
      &student.deref().last_name[..],
      &student.deref().student_id[..],
    );

    for element in &self.student_list {
      let (element_first_name, element_last_name, element_id) = (
        &element.deref().first_name[..],
        &element.deref().last_name[..],
        &element.deref().student_id[..],
      );
      if element_first_name == student_first_name && element_last_name == student_last_name {
        return Err("Student with that name already exists");
      } else if student_id == element_id {
        return Err("Student with that student id already exists");
      }
    }

    self.student_list.push(student);
    Ok(())
  }
  pub fn get_student_by_name(
    &mut self,
    student_first_name: impl Into<String>,
    student_last_name: impl Into<String>,
  ) -> Option<Weak<Student>> {
    let student_first_name = student_first_name.into();
    let student_last_name = student_last_name.into();
    for element in &self.student_list {
      let (element_first_name, element_last_name) = (
        &element.deref().first_name[..],
        &element.deref().last_name[..],
      );
      if element_first_name == student_first_name && element_last_name == student_last_name {
        return Some(Arc::downgrade(&element));
      }
    }
    None
  }
  pub fn get_student_by_id(&mut self, student_id: impl Into<String>) -> Option<Weak<Student>> {
    let student_id = student_id.into();
    for element in &self.student_list {
      let element_id = &element.deref().student_id[..];
      if element_id == student_id {
        return Some(Arc::downgrade(&element));
      }
    }
    None
  }
  pub fn new_student(
    &mut self,
    first_name: impl Into<String>,
    last_name: impl Into<String>,
    student_id: impl Into<String>,
    subject_list_string: Vec<String>,
  ) -> Result<(), &str> {
    let first_name = first_name.into();
    let last_name = last_name.into();
    let student_id = student_id.into();

    let subject_list: Vec<Weak<Subject>> = subject_list_string
      .iter()
      .map(|subject_name| self.get_subject(subject_name).unwrap())
      .collect();
    self.add_student(Arc::new(Student::new(
      first_name,
      last_name,
      student_id,
      subject_list,
    )))
  }
  pub fn get_num_classes(&self, slot_id: usize, department: Weak<Mutex<Department>>) -> usize {
    let mut count: usize = 0;
    for element in match self.class_map.get(&slot_id) {
      Some(k) => k,
      None => return 0,
    } {
      let locked_element = element.lock().unwrap();
      if locked_element.department.ptr_eq(&department) {
        count += 1;
      }
    }
    count
  }
  pub fn add_class(&mut self, slot_id: usize, class: Arc<Mutex<Class>>) -> Result<(), &str> {
    // Check that there is enough space in the department for the subject

    let department = Arc::downgrade(&class.lock().unwrap().department.upgrade().unwrap());

    let max_number = department.upgrade().unwrap().lock().unwrap().max_number;

    // Get the current number
    let curr_number = self.get_num_classes(slot_id, department);

    if curr_number >= max_number {
      return Err("No classes left in department");
    }

    match self.class_map.entry(slot_id) {
      Entry::Vacant(e) => {
        e.insert(vec![class]);
      }
      Entry::Occupied(mut e) => {
        e.get_mut().push(class);
      }
    }

    Ok(())
  }
  pub fn new_class(&mut self, slot_id: usize, subject_name: impl Into<String>) -> Result<(), &str> {
    let subject_name = subject_name.into();
    let subject = match self.get_subject(subject_name) {
      Some(k) => k,
      None => return Err("Subject not found"),
    };

    let department = Arc::downgrade(&subject.upgrade().unwrap().department.upgrade().unwrap());

    self.add_class(
      slot_id,
      Arc::new(Mutex::new(Class::new(subject, department))),
    )
  }
  pub fn get_empty_slot_vec(&self, student: Weak<Student>) -> Vec<usize> {
    let mut empty_slots = Vec::<usize>::new();
    'slot: for (slot_id, class_list) in self.class_map.iter() {
      // Check is the student in any class in this slot
      for class in class_list.iter() {
        for other_student in class.lock().unwrap().student_list.iter() {
          if other_student.ptr_eq(&student) {
            continue 'slot;
          }
        }
      }
      // Slot is empty
      empty_slots.push(*slot_id);
    }
    empty_slots
  }
  pub fn add_student_to_class(
    &mut self,
    student: Weak<Student>,
    subject: Weak<Subject>,
  ) -> Result<bool, &str> {
    let empty_slot_vec = self.get_empty_slot_vec(Weak::clone(&student));

    if empty_slot_vec.len() == 0 {
      return Err("Student has no available slots in timetable");
    }

    for slot_id in empty_slot_vec.iter() {
      let class_list = self.class_map.get(&slot_id).unwrap();
      'class: for class in class_list.iter() {
        // If this class is for the right subject
        if !class.lock().unwrap().subject.ptr_eq(&subject) {
          continue 'class;
        }

        // Add student to class if possible
        match class.lock().unwrap().add_student(Weak::clone(&student)) {
          Ok(..) => return Ok(true),
          Err(..) => {}
        }
      }
    }

    // Try to create a class
    for slot_id in empty_slot_vec.iter() {
      let department = Weak::clone(&subject.upgrade().unwrap().department);
      let mut new_class = Class::new(Weak::clone(&subject), department);
      new_class.add_student(Weak::clone(&student)).unwrap();
      match self.add_class(*slot_id, Arc::new(Mutex::new(new_class))) {
        Ok(..) => return Ok(false),
        Err(..) => {}
      }
    }
    Err("Could not add student to a class")
  }
  pub fn new_student_to_class(
    &mut self,
    first_name: impl Into<String>,
    last_name: impl Into<String>,
    subject_name: impl Into<String>,
  ) -> Result<(), &str> {
    let student = self.get_student_by_name(first_name, last_name).unwrap();
    let subject = self.get_subject(subject_name).unwrap();
    return match self.add_student_to_class(student, subject) {
      Ok(..) => Ok(()),
      Err(k) => Err(k),
    };
  }
  pub fn add_student_to_requested_subjects(&mut self, student: Weak<Student>) -> Result<(), &str> {
    let subject_list = &student.upgrade().unwrap().subject_list;
    for subject in subject_list.iter() {
      self
        .add_student_to_class(Weak::clone(&student), Weak::clone(&subject))
        .unwrap();
    }
    Ok(())
  }

  pub fn new_student_to_requested_subject(
    &mut self,
    first_name: impl Into<String>,
    last_name: impl Into<String>,
  ) -> Result<(), &str> {
    let student = self.get_student_by_name(first_name, last_name).unwrap();
    self.add_student_to_requested_subjects(student).unwrap();
    Ok(())
  }
  pub fn new_student_to_schedule(
    &mut self,
    first_name: impl Into<String>,
    last_name: impl Into<String>,
    student_id: impl Into<String>,
    subject_list_string: Vec<String>,
  ) -> Result<(), &str> {
    let first_name = first_name.into();
    let last_name = last_name.into();
    self
      .new_student(
        first_name.clone(),
        last_name.clone(),
        student_id,
        subject_list_string,
      )
      .unwrap();
    self
      .new_student_to_requested_subject(first_name, last_name)
      .unwrap();
    Ok(())
  }
}
