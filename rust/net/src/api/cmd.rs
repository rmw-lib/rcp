use std::{net::SocketAddr, path::PathBuf};

use anyhow::Result;

use crate::{api::Api, cmd::Cmd};

/*
use std::sync::atomic::Ordering::Relaxed;
macro_rules! id {
  ($db:ident, $name:ident) => {
    $db.$name.fetch_add(1, Relaxed).to_be_bytes()
  };
}
pub const LOGIN: &[u8] = b"login";
*/

impl Api {
  pub fn test(&self, name: String) -> Result<()> {
    dbg!(name);
    Ok(())
  }

  pub async fn info(&self, addr: String, path: String, name: Option<String>) -> Result<()> {
    let name = if name.is_none() {
      let path_buf: PathBuf = path.clone().into();
      path_buf.file_name().unwrap().to_str().unwrap().into()
    } else {
      name.unwrap()
    }
    .as_bytes()
    .into();
    let path = path.as_bytes().into();

    let cmd = Cmd::Info(path, name);
    if let Ok(addr) = addr.parse() {
      match addr {
        SocketAddr::V4(addr) => {
          if let Some(v4) = &self.v4 {
            v4.add(addr, cmd).await
          }
        }
        SocketAddr::V6(addr) => {
          if let Some(v6) = &self.v6 {
            v6.add(addr, cmd).await
          }
        }
      }
    }

    Ok(())
  }

  pub async fn stop(&self) -> Result<()> {
    unsafe { self.stop.force_unlock() };
    Ok(())
  }
}
