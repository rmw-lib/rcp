use std::{ops::Deref, path::PathBuf, sync::Arc};

use async_std::task::spawn;
use expire_map::RetryMap;
use log::info;
use run::Run;
use speedy::Readable;
use time::r#async::sleep;

use crate::{
  cmd::{self, Cmd, TmpQ},
  ctx::{Ctx, Inner},
  ider::{IdType, ID_SIZE},
  node::AddrNode,
  task::Task,
  var::net::Q_RETRY,
  ToAddr,
};

pub type Map<Addr> = RetryMap<Ctx, Task<Addr>, TmpQ>;

pub struct Gate<Addr: ToAddr> {
  map: Map<Addr>,
  node: AddrNode<Addr>, // 用来发流量控制
}

impl<Addr: ToAddr> Deref for Gate<Addr> {
  type Target = Inner;
  fn deref(&self) -> &Self::Target {
    self.map.ctx.deref()
  }
}

impl<Addr: ToAddr> Gate<Addr> {
  pub fn new(addr: Addr, mtu: u16, root: PathBuf) -> std::io::Result<Self> {
    let udp = std::net::UdpSocket::bind(addr)?;
    let map = RetryMap::new(Arc::new(Inner::new(udp, root, mtu)));
    Ok(Self {
      map,
      node: AddrNode::new(),
    })
  }

  pub fn port(&self) -> u16 {
    self.udp.local_addr().unwrap().port()
  }

  pub fn run(&self, run: &Run) {
    let map = self.map.clone();
    run.spawn(async move {
      loop {
        sleep(1000).await;
        map.do_expire();
      }
    });

    // 由于udp包头占8个字节，而在ip层进行封装后的ip包头占去20字节，所以这个是udp数据包的最大理论长度是2^16-1-8-20=65507
    const MAX_LEN: usize = 2944;
    let mut buf = [0; MAX_LEN];
    let map = self.map.clone();
    let ctx = map.ctx.clone();
    let udp: async_std::net::UdpSocket = ctx.udp.try_clone().unwrap().into();
    run.spawn(async move {
      loop {
        if let Ok((n, src)) = err::ok!(udp.recv_from(&mut buf).await) {
          info!("> {} {} {:?}", src, n, &buf[..n]);
          if n >= ID_SIZE {
            let ctx = ctx.clone();
            let id = IdType::from_le_bytes(buf[..ID_SIZE].try_into().unwrap());
            if id % 2 == 0 {
              spawn(async move {
                let buf = &buf[ID_SIZE..n];
                if let Ok(q) = cmd::Q::load(buf) {
                  if let Ok(a) = err::ok!(q.on(&ctx).await) {
                    err::log!(ctx
                      .udp
                      .send_to(&[&id.wrapping_add(1).to_le_bytes()[..], &a].concat(), src));
                  }
                }
              });
            } else {
              let map = map.clone();
              spawn(async move {
                let buf = &buf[ID_SIZE..n];
                if let Some(src) = Addr::from_if_can(src) {
                  let id = id.wrapping_sub(1);
                  if let Some(tmp_q) = map.remove(Task::new(id, src)) {
                    err::log!(tmp_q.tmp.on(&ctx, &tmp_q.q, buf).await);
                  }
                }
              });
            }
          }
        }
      }
    });
  }

  pub async fn add(&self, addr: Addr, cmd: Cmd) {
    let node = self.node.get_or_create(&addr);
    let ider = &node.value().ider;
    let tmp_q = cmd.split();
    loop {
      let id = ider.get();
      let task = Task::new(id, addr);

      if !self.map.has(task) {
        self.map.insert(task, tmp_q, Q_RETRY);
        break;
      }
      sleep(10).await;
    }
  }
}
