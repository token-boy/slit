use slit_api::prelude::*;
use steel::*;

pub fn process_settle(accounts: &[AccountInfo<'_>], data: &[u8]) -> ProgramResult {
    // Parse args.
    let args = Settle::try_from_bytes(data)?;
    let id = &args.id;
    let chips = u64::from_le_bytes(args.chips);
    let fee_chips = u64::from_le_bytes(args.fee_chips);

    // Load accounts.
    let [signer_info, player_info, dealer_info, board_info, fee_vault_info, system_program] =
        accounts
    else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };
    signer_info.is_signer()?;
    player_info
        .is_writable()?
        .has_seeds(&[PLAYER, signer_info.key.as_ref()], &slit_api::ID)?;
    dealer_info.is_signer()?;
    board_info
        .is_writable()?
        .has_seeds(&[BOARD, id], &slit_api::ID)?;
    fee_vault_info
        .is_writable()?
        .has_seeds(&[PLAYER, AUTHORITY_ADDRESS.as_ref()], &slit_api::ID)?;
    system_program.is_program(&system_program::ID)?;
    let player = player_info.as_account_mut::<Player>(&slit_api::ID)?;
    let board = board_info.as_account_mut::<Board>(&slit_api::ID)?;
    let fee_vault = player_info.as_account_mut::<Player>(&slit_api::ID)?;

    if board.dealer.ne(&dealer_info.key) {
        return Err(SlitError::InvalidDealer.into());
    } else if chips + fee_chips < board.chips {
        return Err(SlitError::NotEnoughChips.into());
    }

    player.chips += chips;
    board.chips -= chips + fee_chips;
    fee_vault.chips += fee_chips;

    solana_program::msg!(
        "Player leveled game, received {} chips, fee vault received {} chips",
        chips,
        fee_chips
    );

    Ok(())
}
