use solana_program::{
    account_info::AccountInfo,
    entrypoint::ProgramResult,
    pubkey::Pubkey,
    msg,
};
use crate::instruction::OmbrelleInstruction;

pub struct Processor;

impl Processor {
    pub fn process(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        instruction_data: &[u8],
    ) -> ProgramResult {

        msg!("L'Obrelle : Traitement de l'instruction intercepté.");

        // Le routage invoquera soit InitierPatronage, soit ValiderClockIn ici

        Ok(())
    }
}
