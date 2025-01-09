use steel::*;

use super::SlitAccount;

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Pod, Zeroable)]
pub struct Treasury {
  pub supply: u64,
}

account!(SlitAccount, Treasury);
