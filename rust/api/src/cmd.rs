use speedy::{Readable, Writable};

#[derive(PartialEq, Eq, Debug, Readable, Writable, Clone)]
#[repr(u16)]
#[speedy(tag_type=u16)]
pub enum Cmd {
  // code_gen <
  Stop,
  Info(
    String,         //addr
    String,         //path
    Option<String>, //name
  ),
  Test(
    String, //name
  ),
  // >
}
