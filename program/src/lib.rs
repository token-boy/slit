mod initialize;
mod register;
mod swap;
mod create;
mod stake;
mod redeem;

use initialize::*;
use register::*;
use swap::*;
use create::*;
use stake::*;
use redeem::*;

use slit_api::prelude::*;
use steel::*;

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: &[u8],
) -> ProgramResult {
    let (ix, data) = parse_instruction(&slit_api::ID, program_id, data)?;

    match ix {
        SlitInstruction::Initialize => process_initialize(accounts)?,
        SlitInstruction::Register => process_register(accounts)?,
        SlitInstruction::Swap => process_swap(accounts, data)?,
        SlitInstruction::Create => process_create(accounts, data)?,
        SlitInstruction::Stake => process_stake(accounts, data)?,
        SlitInstruction::Redeem => process_redeem(accounts, data)?,
    }

    Ok(())
}

entrypoint!(process_instruction);
