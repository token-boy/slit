use slit_api::prelude::*;
use steel::*;

pub fn process_create(accounts: &[AccountInfo<'_>], data: &[u8]) -> ProgramResult {
    // Parse args.
    let args = Create::try_from_bytes(data)?;
    let id = &args.id;

    // Load accounts.
    let [signer_info, dealer_info, board_info, system_program] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };
    signer_info.is_signer()?;
    dealer_info.is_signer()?;
    board_info
        .is_empty()?
        .is_writable()?
        .has_seeds(&[BOARD, id], &slit_api::ID)?;
    system_program.is_program(&system_program::ID)?;

    // Initialize board.
    create_account::<Board>(
        board_info,
        system_program,
        signer_info,
        &slit_api::ID,
        &[BOARD, id],
    )?;
    let board = board_info.as_account_mut::<Board>(&slit_api::ID)?;
    board.chips = 0;
    board.dealer = *dealer_info.key;

    solana_program::msg!("Board created");

    Ok(())
}
