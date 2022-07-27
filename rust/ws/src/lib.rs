mod cmd;
mod tcp;

use std::{
  net::{Ipv4Addr, SocketAddrV4},
  sync::Arc,
};

use api::Cmd;
use async_std::net::TcpListener;
use cmd::cmd;
use log::info;
use net::Api;
use run::Run;

pub fn run(run: &Run, api: &Arc<Api>) {
  macro_rules! addr {
    () => {
      SocketAddrV4::new(Ipv4Addr::LOCALHOST, 4910)
    };
  }

  // web socket
  let ws_addr = match std::env::var("RMW_WS") {
    Ok(ws) => match ws.parse() {
      Ok(ws) => ws,
      Err(_) => addr!(),
    },
    Err(_) => addr!(),
  };

  info!("ws://{}", ws_addr);

  let ws_run = run.clone();
  let api = api.clone();

  run.spawn(async move {
    if let Ok(listener) = err::ok!(TcpListener::bind(&ws_addr).await) {
      while let Ok((stream, _)) = listener.accept().await {
        let api = api.clone();
        ws_run.spawn(async move {
          err::log!(tcp::ws(stream, api).await);
        });
      }
    } else {
      err::log!(cmd(&api, Cmd::Stop).await);
    }
  });
}
