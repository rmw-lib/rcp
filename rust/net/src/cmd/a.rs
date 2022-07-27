use speedy::{Readable, Writable};

#[derive(PartialEq, Eq, Debug, Readable, Writable)]
pub enum Info {
  None,
  Dir,
  File(u64),
  Link(Box<[u8]>),
}
