use solana_program::{pubkey, pubkey::Pubkey};

/// The authority allowed to initialize the program.
pub const AUTHORITY_ADDRESS: &Pubkey = &pubkey!("Ao8GyMY5mZ15H5RN3CaokXLdPAoUfBP2G5JEGrQa1hPK");

/// Seed of the board account PDA.
pub const BOARD: &[u8] = b"board";

/// Seed of the player account PDA.
pub const PLAYER: &[u8] = b"player";

/// Seed of the treasury account PDA.
pub const TREASURY: &[u8] = b"treasury";

/// The rate at which chips are converted to lamports.
pub const CHIPS_RATE: u64 = 1000;

/// The rate at which fee is charged.
pub const FEE_RATE: u64 = 100;

#[repr(u8)]
pub enum SwapSide {
    Deposit = 0,
    Withdraw = 1,
}
