mod board;
mod player;
mod treasury;

pub use board::*;
pub use player::*;
pub use treasury::*;

use steel::*;

use crate::consts::*;

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, IntoPrimitive, TryFromPrimitive)]
pub enum SlitAccount {
    Board = 100,
    Player = 101,
    Treasury = 102,
    FeeVault = 103,
}

/// Fetch PDA of the board account.
pub fn board_pda(id: &[u8; 16]) -> (Pubkey, u8) {
    Pubkey::find_program_address(&[BOARD, id], &crate::id())            
}

/// Fetch PDA of the player account.
pub fn player_pda(authority: Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(&[PLAYER, authority.as_ref()], &crate::id())            
}
