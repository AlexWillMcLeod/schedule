mod class;
mod slot;

use crate::prelude::*;
use class::Class;
use slot::Slot;

pub struct Period {
  id: String,
}

#[derive(Default)]
pub struct Timetable {
  period_list: Vec<Period>,
  slot_list: Vec<Slot>,
}
