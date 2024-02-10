use self::{crypto::Crypto, id_generator::IDGenerator};
pub mod crypto;
pub mod id_generator;

pub struct Utilities<C: Crypto, ID: IDGenerator> {
  pub crypto: C,
  pub id_generator: ID,
}
