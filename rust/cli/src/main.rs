use anyhow::Result;
use api::{Cmd, A, Q};
use speedy::{Readable, Writable};
use tungstenite::{connect, Message, Message::Binary};

fn main() -> Result<()> {
  rmw_log::init().apply()?;

  if let Ok((mut socket, _)) = err::ok!(connect("ws://127.0.0.1:4910")) {
    let q = Q {
      id: 0,
      cmd: Cmd::Stop, //Cmd::UserNew("test".to_string()),
    };

    socket.write_message(Message::Binary(q.dump()?.into()))?;

    if let Ok(Binary(bin)) = socket.read_message() {
      let a = A::load(&bin);
      println!("Received: {:?}", a);
    }
  }
  Ok(())
}
