use crate::{
  ider::{IdBin, IdType},
  ToAddr,
};

#[derive(Debug, Copy, Clone, Eq, Hash, PartialEq)]
pub struct Task<Addr: ToAddr> {
  pub addr: Addr,
  pub id: IdBin,
}

impl<Addr: ToAddr> Task<Addr> {
  pub fn new(id: IdType, addr: Addr) -> Self {
    Self {
      id: id.to_le_bytes(),
      addr,
    }
  }
}
