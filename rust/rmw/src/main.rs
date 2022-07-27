use anyhow::Result;
use async_std::task::block_on;

fn main() -> Result<()> {
  rmw_log::init()
    .level_for("surf", log::LevelFilter::Warn)
    .apply()?;

  let mut root = std::env::current_exe()?;
  root.pop();

  if let Ok(net) = err::ok!(net::Net::open(root)) {
    ws::run(&net.run, &net.api);
    block_on(net.run());
  }
  Ok(())
}
