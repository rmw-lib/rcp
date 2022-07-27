use std::{
  net::{SocketAddr, UdpSocket},
  path::PathBuf,
  sync::Arc,
};

use anyhow::Result;
use config::config;
use log::info;
use run::Run;

use crate::{
  api::Api,
  var::{mtu, net::UPNP_SLEEP_SECONDS},
};

pub struct Net {
  pub run: Run,
  pub api: Arc<Api>,
}

impl Net {
  pub async fn run(mut self) {
    self.run.join().await;
  }

  pub fn open(root: PathBuf) -> Result<Net> {
    let run = Run::new();

    run.spawn(time::update());

    config!(rmw);

    macro_rules! srv {
      ($v:expr, $enable:expr, $ip:expr) => {{
        paste::paste! {
          if get!( run / [<v $v>] , $enable) {
            let addr = get!([<v $v>]/ udp, {
              match UdpSocket::bind($ip)
                .unwrap()
                .local_addr()
                .unwrap()
              {
                SocketAddr::[<V $v>](addr) => Some(addr),
                _ => None,
              }
              .unwrap()
            });

            // udp 端口
            info!("udp://{}", &addr);
            let mtu = get!([<v $v>] / mtu, mtu::[<UDP_IPV $v>]);

            let udp = crate::gate::Gate::new(addr, mtu, root.clone())?;
            udp.run(&run);

            Some(udp)
          } else {
            None
          }
        }
      }};
    }

    let v4 = srv!(
      4,
      true,
      std::net::SocketAddrV4::new(std::net::Ipv4Addr::UNSPECIFIED, 0)
    );
    if let Some(udp) = &v4 {
      // upnp 端口映射
      if cfg!(feature = "upnp") && get!(v4 / upnp, true) {
        run.spawn(upnp::daemon("rmw", udp.port(), UPNP_SLEEP_SECONDS));
      }
    }

    let v6 = srv!(
      6,
      false,
      std::net::SocketAddrV6::new(std::net::Ipv6Addr::UNSPECIFIED, 0, 0, 0)
    );

    Ok(Net {
      api: Arc::new(Api::new(run.stop.clone(), v4, v6)),
      run,
    })
  }
}
