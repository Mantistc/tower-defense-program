use pinocchio::{account_info::AccountInfo, pubkey::Pubkey, ProgramResult};
use pinocchio_log::log;

#[repr(u8)]
pub enum Instructions {
    Initialize,
    IncrementValue,
}

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    log!("Hello from my program!");
    Ok(())
}
