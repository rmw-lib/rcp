mod cmd;

use std::{
  net::{SocketAddrV4, SocketAddrV6},
  sync::Arc,
};

use parking_lot::Mutex;

use crate::gate::Gate;

pub struct Api {
  pub stop: Arc<Mutex<()>>,
  v4: Option<Gate<SocketAddrV4>>,
  v6: Option<Gate<SocketAddrV6>>,
}

impl Api {
  pub fn new(
    stop: Arc<Mutex<()>>,
    v4: Option<Gate<SocketAddrV4>>,
    v6: Option<Gate<SocketAddrV6>>,
  ) -> Self {
    std::mem::forget(stop.lock());
    Self { stop, v4, v6 }
  }
}
