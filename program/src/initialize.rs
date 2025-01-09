use slit_api::prelude::*;
use steel::*;

pub fn process_initialize(accounts: &[AccountInfo<'_>]) -> ProgramResult {
    // Load accounts.
    let [signer_info, treasury_info, fee_vault_info, system_program] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };
    signer_info.is_signer()?.has_address(&AUTHORITY_ADDRESS)?;
    treasury_info
        .is_empty()?
        .is_writable()?
        .has_seeds(&[TREASURY], &slit_api::ID)?;
    fee_vault_info
        .is_empty()?
        .is_writable()?
        .has_seeds(&[PLAYER, signer_info.key.as_ref()], &slit_api::ID)?;
    system_program.is_program(&system_program::ID)?;

    // Initialize treasury.
    create_account::<Treasury>(
        treasury_info,
        system_program,
        signer_info,
        &slit_api::ID,
        &[TREASURY],
    )?;
    let treasury = treasury_info.as_account_mut::<Treasury>(&slit_api::ID)?;
    treasury.supply = 0;
    solana_program::msg!("Treasury initialized");

    // Initialize fee vault.
    create_account::<Player>(
        fee_vault_info,
        system_program,
        signer_info,
        &slit_api::ID,
        &[PLAYER, signer_info.key.as_ref()],
    )?;
    let fee_vault = fee_vault_info.as_account_mut::<Player>(&slit_api::ID)?;
    fee_vault.chips = 0;
    solana_program::msg!("Fee vault initialized");

    Ok(())
}
