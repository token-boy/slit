use steel::*;

use super::SlitAccount;

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Pod, Zeroable)]
pub struct Player {
    pub chips: u64
}

account!(SlitAccount, Player);
