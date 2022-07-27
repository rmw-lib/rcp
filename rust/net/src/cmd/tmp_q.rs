use expire_map::Caller;
use speedy::Writable;

use super::{Tmp, Q};
use crate::{ctx::Ctx, r#var::net::Q_TIMEOUT, task::Task, ToAddr};

pub struct TmpQ {
  pub tmp: Tmp,
  pub q: Q,
}

impl<Addr: ToAddr> Caller<Ctx, Task<Addr>> for TmpQ {
  fn ttl() -> u8 {
    Q_TIMEOUT
  }

  fn call(&mut self, ctx: &Ctx, task: &Task<Addr>) -> u8 {
    if let Ok(q) = err::ok!(self.q.dump()) {
      let id = task.id;
      dbg!((task.addr, id));
      err::log!(ctx.udp.send_to(&[&id[..], &q[..]].concat(), task.addr));
    }
    Q_TIMEOUT
  }

  fn fail(&mut self, _: &Ctx, task: &Task<Addr>) {
    dbg!(("fail", task.addr));
  }
}
