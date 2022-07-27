use std::ops::Deref;

use dashmap::{mapref::one::RefMut, DashMap};

use crate::{ider::Ider, ToAddr};

#[derive(Debug, Clone, Default)]
pub struct Node {
  pub ider: Ider,
}

#[derive(Debug, Default)]
pub struct AddrNode<Addr: ToAddr> {
  map: DashMap<Addr, Node>,
}

impl<Addr: ToAddr> Deref for AddrNode<Addr> {
  type Target = DashMap<Addr, Node>;

  fn deref(&self) -> &<Self as Deref>::Target {
    &self.map
  }
}

impl<'a, Addr: ToAddr> AddrNode<Addr> {
  pub fn new() -> Self {
    Self {
      map: DashMap::new(),
    }
  }

  pub fn get_or_create(&'a self, key: &Addr) -> RefMut<'a, Addr, Node> {
    loop {
      if let Some(r) = self.map.get_mut(key) {
        return r;
      }
      self.map.insert(*key, Node::default());
    }
  }
}
