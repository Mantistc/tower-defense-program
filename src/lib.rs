use pinocchio::entrypoint;
use pinocchio_pubkey::declare_id;
use processor::process_instruction;
pub mod instructions;
pub mod processor;
pub mod states;

declare_id!("tdpUmm2N1bhmSfYAynVuWWFWSd5aF5LmiBTPXJEwoW6");
entrypoint!(process_instruction);
