use crate::schedule::timetable::{Class, Period};
use std::sync::{Arc, Weak};

pub struct Slot {
  period_list: Vec<Weak<Period>>,
  class_list: Vec<Arc<Class>>,
}
