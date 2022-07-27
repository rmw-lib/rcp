mod a;
mod q;
pub use q::Q;

mod tmp;
pub use tmp::Tmp;

mod tmp_q;
pub use tmp_q::TmpQ;

pub type Path = Box<[u8]>;
pub type To = Box<[u8]>;

#[derive(Debug)]
pub enum Cmd {
  Info(Path, To),
}

impl Cmd {
  pub fn split(self) -> TmpQ {
    macro_rules! cmd {
      ($($kind:ident / $($q:ident),* / $($t:ident),* ;)+) => {
        match self {
          $(
            Cmd::$kind($($q,)* $($t),*) => TmpQ{
              q:Q::$kind(q::$kind { $($q),* }),
              tmp:Tmp::$kind(tmp::$kind { $($t),* }),
            },
          )+
        }
      };
    }
    cmd!(
      Info / path / to ;
    )
  }
}
