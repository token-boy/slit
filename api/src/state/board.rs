use steel::*;

use super::SlitAccount;

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Pod, Zeroable)]
pub struct Board {
    pub chips: u64,
    pub dealer: Pubkey,
}

account!(SlitAccount, Board);
