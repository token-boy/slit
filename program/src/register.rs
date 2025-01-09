use slit_api::prelude::*;
use steel::*;

pub fn process_register(accounts: &[AccountInfo<'_>]) -> ProgramResult {
    // Load accounts.
    let [signer_info, player_info, system_program] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };
    signer_info.is_signer()?;
    player_info
        .is_empty()?
        .is_writable()?
        .has_seeds(&[PLAYER, signer_info.key.as_ref()], &slit_api::ID)?;
    system_program.is_program(&system_program::ID)?;

    // Initialize player.
    create_account::<Player>(
        player_info,
        system_program,
        signer_info,
        &slit_api::ID,
        &[PLAYER, signer_info.key.as_ref()],
    )?;

    let player = player_info.as_account_mut::<Player>(&slit_api::ID)?;
    player.chips = 0;

    solana_program::msg!("Player registered");

    Ok(())
}
