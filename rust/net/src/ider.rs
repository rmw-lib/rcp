use std::sync::{
  atomic::{AtomicU16, Ordering::Relaxed},
  Arc,
};

pub type IdType = u16;
pub const ID_SIZE: usize = std::mem::size_of::<IdType>();
pub type IdAtom = AtomicU16;
pub type IdBin = [u8; ID_SIZE];

#[derive(Debug, Clone)]
pub struct Ider {
  id: Arc<IdAtom>,
}

impl Ider {
  pub fn new() -> Self {
    let id = rand::random::<IdType>().wrapping_mul(2);
    Ider {
      id: Arc::new(IdAtom::new(id)),
    }
  }

  pub fn get(&self) -> IdType {
    loop {
      let r = self.id.fetch_add(2, Relaxed);
      if r != 0 {
        return r;
      }
    }
  }
}

impl Default for Ider {
  fn default() -> Self {
    Self::new()
  }
}
