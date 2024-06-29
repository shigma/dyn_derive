use core::fmt::*;

use crate::inst::Instance;

macro_rules! impl_fmt_trait {
  ($t:ident) => {
      impl<T: $t, U> $t for Instance<T, U> {
          fn fmt(&self, f: &mut Formatter) -> Result {
              self.0.fmt(f)
          }
      }
  };
}

impl_fmt_trait!(Debug);
impl_fmt_trait!(Display);
impl_fmt_trait!(Binary);
impl_fmt_trait!(LowerExp);
impl_fmt_trait!(LowerHex);
impl_fmt_trait!(Octal);
impl_fmt_trait!(UpperExp);
impl_fmt_trait!(UpperHex);
