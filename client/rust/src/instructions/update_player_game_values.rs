use std::slice::from_raw_parts;

use solana_sdk::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
};

use crate::PROGRAM_ID;

use super::{write_bytes, InstructionDiscriminator, UNINIT_BYTE};


#[inline(always)]
pub fn update_player_game_values(
    player: &Pubkey,
    signer: &Pubkey,
    last_time_played: u64,
    wave_count: u8,
) -> Instruction {
    let account_metas: [AccountMeta; 2] = [
        AccountMeta::new(*player, false),
        AccountMeta::new(*signer, true),
    ];

    // Instruction data layout:
    // -  [0]: instruction discriminator (1 byte, u8)
    // -  [1..9]: last_time_played (8 byte, u64)
    // -  [8]: wave count (1 byte, u8)
    let data_len = 10;

    let mut instruction_data = vec![UNINIT_BYTE; data_len];
    write_bytes(
        &mut instruction_data,
        &[InstructionDiscriminator::UpdatePlayerGameValues as u8],
    );
    write_bytes(&mut instruction_data[1..9], &last_time_played.to_le_bytes());
    write_bytes(&mut instruction_data[9..10], &wave_count.to_le_bytes());

    Instruction {
        program_id: PROGRAM_ID,
        accounts: account_metas.to_vec(),
        data: unsafe { from_raw_parts(instruction_data.as_ptr() as _, data_len).to_vec() },
    }
}
