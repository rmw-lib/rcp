use anyhow::Result;
use async_std::fs::{read_link, symlink_metadata};
use speedy::{Readable, Writable};

use super::{a, q};
use crate::ctx::Ctx;

#[derive(PartialEq, Eq, Debug, Readable, Writable)]
#[repr(u8)]
pub enum Q {
  Info(Info),
}

#[derive(PartialEq, Eq, Debug, Readable, Writable)]
pub struct Info {
  pub path: Box<[u8]>,
}

async fn info(ctx: &Ctx, info: q::Info) -> Result<a::Info> {
  let path = ctx.path(&*unsafe { std::str::from_boxed_utf8_unchecked(info.path) });
  let meta = symlink_metadata(&path).await?;
  Ok(if meta.is_dir() {
    a::Info::Dir
  } else {
    let file_type = meta.file_type();
    if file_type.is_symlink() {
      let r = read_link(&path).await?;
      a::Info::Link(Box::from(r.display().to_string().as_bytes()))
    } else {
      a::Info::File(meta.len())
    }
  })
}

impl Q {
  pub async fn on(self, ctx: &Ctx) -> Result<Box<[u8]>> {
    match self {
      Q::Info(i) => {
        Ok(info(ctx, i).await.unwrap_or(a::Info::None).dump()?)

        /*
        if let Ok(fp) = err::ok!(ctx.file_cache.get(&path).await) {
        if let Ok(meta) = err::ok!(fp.value().metadata().await) {
        }
        }
        */
        /*
        .canonicalize()
        {
        dbg!((&path, path.starts_with(&ctx.root)));
        if path.starts_with(&ctx.root) {
        dbg!(&path);
        }
        }
        */
      }
    }
  }
}

/*
   use super::On;
#[enum_dispatch(On)]
impl On for Info {
fn on<Addr: ToAddr>(&self, udp: Ctx<Addr>) {
if let Ok(path) = std::str::from_utf8(&self.path) {
udp.path(path);
dbg!(path);
}
}
}
*/
