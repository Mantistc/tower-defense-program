use crate::processor::{
    process_initialize_player, process_update_player_authority, process_update_player_game_values,
};
use pinocchio::{
    account_info::AccountInfo, entrypoint, program_error::ProgramError, pubkey::Pubkey,
    ProgramResult,
};

entrypoint!(process_instruction);

pub fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let (discriminator, instruction_data) = instruction_data
        .split_first()
        .ok_or(ProgramError::InvalidInstructionData)?;

    match discriminator {
        0 => process_initialize_player(accounts, instruction_data),
        1 => process_update_player_game_values(accounts, instruction_data),
        2 => process_update_player_authority(accounts),
        _ => Err(ProgramError::InvalidInstructionData),
    }
}
