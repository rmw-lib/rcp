use std::{
  net::UdpSocket,
  path::{Component, Path, PathBuf},
  sync::Arc,
};

use file_cache::FileCache;

pub struct Inner {
  pub udp: UdpSocket,
  pub mtu: u16,
  pub root: Box<str>,
  pub file_cache: FileCache,
}

impl Inner {
  pub fn new(udp: UdpSocket, root: PathBuf, mtu: u16) -> Self {
    Self {
      udp,
      mtu,
      root: Box::from(root.to_str().unwrap()),
      file_cache: err::ok!(FileCache::new(2048)).unwrap(),
    }
  }

  pub fn path(&self, path: impl AsRef<Path>) -> PathBuf {
    let mut t = PathBuf::from(&*self.root);
    let len = t.as_os_str().len();
    for i in path.as_ref().components() {
      match i {
        Component::ParentDir => {
          if t.as_os_str().len() > len {
            t.pop();
          }
        }
        Component::Normal(i) => {
          t.push(i);
        }
        _ => {}
      }
    }

    t
  }
}

pub type Ctx = Arc<Inner>;
