use solana_sdk::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
};

use crate::PROGRAM_ID;

use super::InstructionDiscriminator;

#[inline(always)]
pub fn update_player_authority(
    player: &Pubkey,
    signer: &Pubkey,
    new_authority: &Pubkey,
) -> Instruction {
    let account_metas: [AccountMeta; 3] = [
        AccountMeta::new(*player, false),
        AccountMeta::new(*signer, true),
        AccountMeta::new(*new_authority, false),
    ];

    // Instruction data layout:
    // -  [0]: instruction discriminator (1 byte, u8)
    let data_len = 1;
    let instruction_data = vec![InstructionDiscriminator::UpdatePlayerAuthority as u8; data_len];

    Instruction {
        program_id: PROGRAM_ID,
        accounts: account_metas.to_vec(),
        data: instruction_data,
    }
}
