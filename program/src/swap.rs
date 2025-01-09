use slit_api::{
    consts::{SwapSide, AUTHORITY_ADDRESS, CHIPS_RATE, FEE_RATE, PLAYER, TREASURY},
    error::SlitError,
    instruction::Swap,
    state::Player,
};
use steel::*;

pub fn process_swap(accounts: &[AccountInfo<'_>], data: &[u8]) -> ProgramResult {
    // Parse args.
    let args = Swap::try_from_bytes(data)?;
    let side = args.side;
    let amount = u64::from_le_bytes(args.amount);
    if amount < CHIPS_RATE || amount % CHIPS_RATE != 0 {
        return Err(SlitError::InvalidAmount.into());
    }

    // Load accounts.
    let [signer_info, player_info, treasury_info, fee_vault_info, system_program] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };
    signer_info.is_signer()?;
    player_info
        .is_writable()?
        .has_seeds(&[PLAYER, signer_info.key.as_ref()], &slit_api::ID)?;
    treasury_info
        .is_writable()?
        .has_seeds(&[TREASURY], &slit_api::ID)?;
    fee_vault_info
        .is_writable()?
        .has_seeds(&[PLAYER, AUTHORITY_ADDRESS.as_ref()], &slit_api::ID)?;
    system_program.is_program(&system_program::ID)?;
    let player = player_info.as_account_mut::<Player>(&slit_api::ID)?;
    let fee_vault = fee_vault_info.as_account_mut::<Player>(&slit_api::ID)?;

    if side == SwapSide::Deposit as u8 {
        let lamports = amount / CHIPS_RATE;

        if signer_info.lamports() < lamports {
            return Err(SlitError::NotEnoughLamports.into());
        }

        let transfer_instruction = solana_program::system_instruction::transfer(
            signer_info.key,
            treasury_info.key,
            lamports,
        );
        solana_program::program::invoke(
            &transfer_instruction,
            &[
                signer_info.clone(),
                treasury_info.clone(),
                system_program.clone(),
            ],
        )?;

        player.chips += amount;

        solana_program::msg!("Deposited {} lamports for {} chips", lamports, amount);
    } else if side == SwapSide::Withdraw as u8 {
        if player.chips < amount {
            return Err(SlitError::NotEnoughChips.into());
        }
        let fee: u64 = if player_info.key.ne(fee_vault_info.key) {
            amount / FEE_RATE - (amount / FEE_RATE) % CHIPS_RATE
        } else {
            0
        };
        let lamports = (amount - fee) / CHIPS_RATE;

        // see https://stackoverflow.com/questions/71868505/how-to-transfer-sol-from-a-pda-which-created-a-token-associate-account-to-a-norm
        **treasury_info.try_borrow_mut_lamports()? -= lamports;
        **signer_info.try_borrow_mut_lamports()? += lamports;

        player.chips -= amount;
        fee_vault.chips += fee;

        solana_program::msg!("Withdrew {} chips for {} lamports", amount, lamports);
    } else {
        return Err(SlitError::InvalidSwapSide.into());
    }

    Ok(())
}
