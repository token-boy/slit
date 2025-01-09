use steel::*;

#[derive(Debug, Error, Clone, Copy, PartialEq, Eq, IntoPrimitive)]
#[repr(u32)]
pub enum SlitError {
    #[error("The maximum number of players has been reached")]
    ExceededMaxPlayers = 0,
    #[error("Invalid treasury address")]
    InvalidTreasuryAddress = 1,
    #[error("Invalid swap side")]
    InvalidSwapSide = 2,
    #[error("Invalid amount")]
    InvalidAmount = 3,
    #[error("Not enough lamports")]
    NotEnoughLamports = 4,
    #[error("Not enough chips")]
    NotEnoughChips = 5,
    #[error("Invalid fee vault")]
    InvalidFeeVault = 6,
    #[error("Invalid dealer")]
    InvalidDealer = 7,
    #[error("Invalid dealer")]
    InvalidPlayer = 8,
}

error!(SlitError);
