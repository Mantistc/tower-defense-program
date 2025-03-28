use pinocchio::{
    account_info::AccountInfo, entrypoint, pubkey::Pubkey, ProgramResult
};
use pinocchio_log::log;
use pinocchio_pubkey::declare_id;

entrypoint!(process_instruction);

declare_id!("tdpUmm2N1bhmSfYAynVuWWFWSd5aF5LmiBTPXJEwoW6");

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    log!("Hello from my program!");
    Ok(())
}
