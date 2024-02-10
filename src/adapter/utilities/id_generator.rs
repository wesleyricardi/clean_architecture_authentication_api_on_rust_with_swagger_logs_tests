use uuid::Uuid;

use crate::domain::utilities::id_generator::IDGenerator;

pub struct NewID;

impl IDGenerator for NewID {
  fn new_uuid(&self) -> String {
    Uuid::new_v4().to_string()
  }
}
