use std::sync::Arc;

use anyhow::Result;
use api::{Cmd, Reply};
use net::Api;

pub async fn cmd(api: &Arc<Api>, cmd: api::Cmd) -> Result<Reply> {
  dbg!(&cmd);

  Ok(match cmd {
    // code_gen <
    Cmd::Info(addr, path, name) => {
      api.info(addr, path, name).await?;
      Reply::Undefined
    }
    Cmd::Stop => {
      api.stop().await?;
      Reply::Undefined
    }
    Cmd::Test(name) => {
      api.test(name)?;
      Reply::Undefined
    } // >
  })
}
