use mockall::automock;

#[automock]
pub trait IDGenerator: Sync + Send {
  fn new_uuid(&self) -> String;
}
