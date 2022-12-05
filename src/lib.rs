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

#[derive(Debug)]
pub struct Schedule {
  pub department_list: Vec<Arc<Mutex<Department>>>,
  pub subject_list: Vec<Arc<Subject>>,
  pub student_list: Vec<Arc<Student>>,
  pub class_map: HashMap<usize, Vec<Arc<Mutex<Class>>>>,
}

impl Schedule {
  pub fn new() -> Schedule {
    Schedule {
      department_list: Vec::<Arc<Mutex<Department>>>::new(),
      subject_list: Vec::<Arc<Subject>>::new(),
      student_list: Vec::<Arc<Student>>::new(),
      class_map: HashMap::<usize, Vec<Arc<Mutex<Class>>>>::new(),
    }
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
  pub fn get_department(&mut self, department_name: String) -> Option<Weak<Mutex<Department>>> {
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
    name: String,
    class_size: usize,
    max_number: usize,
  ) -> Result<(), &str> {
    self.add_department(Arc::new(Mutex::new(Department::new(
      name, max_number, class_size,
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
  pub fn get_subject(&mut self, subject_name: String) -> Option<Weak<Subject>> {
    for element in &self.subject_list {
      let element_name = &element.deref().name[..];
      if element_name == subject_name {
        return Some(Arc::downgrade(&element));
      }
    }
    None
  }
  pub fn new_subject(&mut self, name: String, department_name: String) -> Result<(), &str> {
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
        return Err("Student with that student id already eixsts");
      }
    }

    self.student_list.push(student);
    Ok(())
  }
  pub fn get_student_by_name(
    &mut self,
    student_first_name: String,
    student_last_name: String,
  ) -> Option<Weak<Student>> {
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
  pub fn get_student_by_id(&mut self, student_id: String) -> Option<Weak<Student>> {
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
    first_name: String,
    last_name: String,
    student_id: String,
    subject_list_string: Vec<String>,
  ) -> Result<(), &str> {
    let subject_list: Vec<Weak<Subject>> = subject_list_string
      .iter()
      .map(|subject_name| self.get_subject(subject_name.to_string()).unwrap())
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
  pub fn new_class(&mut self, slot_id: usize, subject_name: String) -> Result<(), &str> {
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
  pub fn add_student_to_class(
    &mut self,
    student: Weak<Student>,
    subject: Weak<Subject>,
  ) -> Result<(), &str> {
    'slot: for class_list in self.class_map.values() {
      // Check is the student in any class in this slot
      for class in class_list.iter() {
        for other_student in class.lock().unwrap().student_list.iter() {
          if other_student.ptr_eq(&student) {
            continue 'slot;
          }
        }
      }
      // Student is not found in the slot
    }
    Ok(())
  }
}
