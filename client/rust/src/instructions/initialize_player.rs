use std::slice::from_raw_parts;

use solana_program::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey, system_program,
};

use crate::PROGRAM_ID;

use super::{write_bytes, InstructionDiscriminator, UNINIT_BYTE};

#[inline(always)]
pub fn initialize_player(
    player: &Pubkey,
    signer: &Pubkey,
    last_time_played: u64,
    bump: u8,
) -> Instruction {
    let account_metas: [AccountMeta; 3] = [
        AccountMeta::new(*player, false),
        AccountMeta::new(*signer, true),
        AccountMeta::new_readonly(system_program::ID, false),
    ];

    // Instruction data layout:
    // -  [0]: instruction discriminator (1 byte, u8)
    // -  [1..9]: last_time_played (8 byte, u64)
    // -  [8]: bump (1 byte, u8)
    let data_len = 10;

    let mut instruction_data = vec![UNINIT_BYTE; data_len];
    write_bytes(
        &mut instruction_data,
        &[InstructionDiscriminator::InitializePlayer as u8],
    );
    write_bytes(&mut instruction_data[1..9], &last_time_played.to_le_bytes());
    write_bytes(&mut instruction_data[9..10], &bump.to_le_bytes());

    Instruction {
        program_id: PROGRAM_ID,
        accounts: account_metas.to_vec(),
        data: unsafe { from_raw_parts(instruction_data.as_ptr() as _, data_len).to_vec() },
    }
}
