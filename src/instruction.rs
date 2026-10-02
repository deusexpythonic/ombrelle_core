use solana_program::program_error::ProgramError;

pub enum OmbrelleInstruction {
    // Instruction 0 : NULL/PATRONAGE (Crypto World's Fair)
    InitierPatronage {
        blind_hash: [u8; 32],
        entropy_shield: u64,
        expiration_time: i64,
},
    // Instruction 1 : PANOPTIC/NULL (Clock In)
    ValiderClockIn {
    provided_secret: Vec<u8>,
    }
}
