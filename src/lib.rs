use anchor_lang::prelude::*;

// L'identité immuable de votre manifeste sur le réseau
declare_id!("GBUBpHatnT5rkhYcCD7MGrq2FFhT3fC7upVeUUbf9uU2");

#[program]
pub mod ombrelle_core {
    use super::*;

    // L'instruction n'accepte rien d'autre que l'empreinte mathématique brute.
    // Aucun nom, aucune coordonnée, aucun métadonnée.
    pub fn clock_in(ctx: Context<ClockIn>, secret_hash: [u8; 32]) -> Result<()> {
        let threshold_account = &mut ctx.accounts.threshold_account;
        
        // Le franchissement du seuil. Binaire. Silencieux.
        threshold_account.crossed = true;
        
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(secret_hash: [u8; 32])]
pub struct ClockIn<'info> {
    // La dérivation aveugle (Hash-based blind state). 
    // Le signataire paie les frais, mais son identité n'infecte pas le PDA.
    #[account(
        init,
        payer = signer,
        space = 8 + 1, // Famine absolue : 8 octets pour Anchor, 1 octet pour le booléen.
        seeds = [secret_hash.as_ref()],
        bump
    )]
    pub threshold_account: Account<'info, Threshold>,
    
    #[account(mut)]
    pub signer: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

// Le Compte Minimal (Data Starvation).
#[account]
pub struct Threshold {
    pub crossed: bool,
}
