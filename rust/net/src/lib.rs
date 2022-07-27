#![feature(new_uninit)]
#![feature(trait_alias)]
#![feature(async_closure)]
#![feature(drain_filter)]

use std::net::{SocketAddr, SocketAddrV4, SocketAddrV6, ToSocketAddrs};

mod api;
mod cmd;
mod ctx;
mod gate;
mod ider;
mod net;
mod node;
mod task;
mod var;

pub trait FromIfCan {
  fn from_if_can(addr: SocketAddr) -> Option<Self>
  where
    Self: Sized;
}

pub trait ToAddr = std::hash::Hash
  + Clone
  + Copy
  + ToSocketAddrs
  + std::cmp::Eq
  + std::fmt::Debug
  + std::marker::Sync
  + std::marker::Send
  + FromIfCan
  + 'static;

impl FromIfCan for SocketAddrV6 {
  fn from_if_can(addr: SocketAddr) -> Option<Self> {
    match addr {
      SocketAddr::V6(t) => Some(t),
      _ => None,
    }
  }
}

impl FromIfCan for SocketAddrV4 {
  fn from_if_can(addr: SocketAddr) -> Option<Self> {
    match addr {
      SocketAddr::V4(t) => Some(t),
      _ => None,
    }
  }
}

pub use crate::{api::Api, net::Net};

/*
#[macro_export]
macro_rules! from {
($to:ident, $cls:ident) => {
impl From<$cls> for $to {
fn from(i: $cls) -> Self {
$to::$cls(i)
}
}
};
($to:ident, $($cls:ident),+) => {
$($to,from!($cls);)+
}
}
*/
