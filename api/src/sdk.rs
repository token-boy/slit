use steel::*;

use crate::prelude::*;

pub fn initialize(signer: Pubkey) -> Instruction {
    Instruction {
        program_id: crate::ID,
        accounts: vec![
            AccountMeta::new(signer, true),
            AccountMeta::new_readonly(system_program::ID, false),
        ],
        data: Initialize {}.to_bytes(),
    }
}

pub fn register(signer: Pubkey, player: Pubkey) -> Instruction {
    Instruction {
        program_id: crate::ID,
        accounts: vec![
            AccountMeta::new(signer, true),
            AccountMeta::new(player, false),
            AccountMeta::new_readonly(system_program::ID, false),
        ],
        data: Register {}.to_bytes(),
    }
}

pub fn swap(
    signer: Pubkey,
    player: Pubkey,
    treasury: Pubkey,
    side: u8,
    amount: u64,
) -> Instruction {
    Instruction {
        program_id: crate::ID,
        accounts: vec![
            AccountMeta::new(signer, true),
            AccountMeta::new(player, false),
            AccountMeta::new(treasury, false),
            AccountMeta::new_readonly(system_program::ID, false),
        ],
        data: Swap {
            side,
            amount: amount.to_le_bytes(),
        }
        .to_bytes(),
    }
}
