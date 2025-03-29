use crate::instructions::process_initialize_player;
use pinocchio::{
    account_info::AccountInfo, program_error::ProgramError, pubkey::Pubkey, ProgramResult,
};

#[repr(u8)]
pub enum Instructions {
    InitializePlayer,
    UpdatePlayer,
}

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let (discriminator, instruction_data) = instruction_data
        .split_first()
        .ok_or(ProgramError::InvalidInstructionData)?;

    match discriminator {
        0 => process_initialize_player(accounts, instruction_data, program_id),
        _ => Err(ProgramError::InvalidInstructionData),
    }
}
