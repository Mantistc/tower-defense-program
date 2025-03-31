use solana_sdk::pubkey::Pubkey;

pub mod instructions;

pub const PROGRAM_ID: Pubkey =
    Pubkey::from_str_const("tdpUmm2N1bhmSfYAynVuWWFWSd5aF5LmiBTPXJEwoW6");

pub mod seeds {
    pub const PLAYER_SEED: &'static [u8] = b"player";
}
