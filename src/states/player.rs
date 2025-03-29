#[repr(C)]
#[derive(Default)]
pub struct Player {
    pub wave_reached: u8,
    pub last_played: u64,
    pub authority: [u8; 32],
}
pub const PLAYER_SEED: &'static [u8] = b"player";
