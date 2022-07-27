use anyhow::Result;
use speedy::Readable;

use super::{a, Q};
use crate::ctx::Ctx;

#[derive(Debug)]
pub enum Tmp {
  Info(Info),
}

#[derive(Debug)]
pub struct Info {
  pub to: Box<[u8]>,
}

impl Tmp {
  pub async fn on(self, _ctx: &Ctx, q: &Q, msg: &[u8]) -> Result<()> {
    match self {
      Tmp::Info(info) => {
        if let Ok(a) = err::ok!(a::Info::load(msg)) {
          dbg!((a, info.to, q));
        }
      }
    }
    Ok(())
  }
}
