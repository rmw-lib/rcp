use api::Cmd;
use js_sys::Promise;
use wasm_bindgen::prelude::*;

use crate::W;

// code_gen 自动生成
#[wasm_bindgen]
impl W {
  // code_gen <

  pub fn info(&mut self, addr: String, path: String, name: Option<String>) -> Promise {
    self.req(Cmd::Info(addr, path, name))
  }

  pub fn stop(&mut self) -> Promise {
    self.req(Cmd::Stop)
  }

  pub fn test(&mut self, name: String) -> Promise {
    self.req(Cmd::Test(name))
  }

  // >
}
