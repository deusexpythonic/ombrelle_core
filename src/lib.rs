pub mod instruction;
pub mod processor;

use solana_program::{
    account_info::AccountInfo,
    entrypoint,
    entrypoint::ProgramResult,
    hash::{Hash, hash},
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};
use std::convert::TryInto;

// --- 0x00. LE PORTAIL DE LA BLOCKCHAIN ---
entrypoint!(process_instruction);

pub fn process_instruction(
    _program_id: &Pubkey,
    _accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    msg!("Ombrelle Invariant: Null/Clock In");

    // 1. Vérification de la taille des données entrantes (Sécurité de base)
    // Secret (variable), hash stocké (32), sel temporel (8), expiration (8), temps actuel (8).
    // Total minimum = 56 octets + longueur du secret.
    if instruction_data.len() < 57 {
        msg!("Erreur: Payload trop court ou corrompu.");
        return Err(ProgramError::InvalidInstructionData);
    }

    // 2. Extraction brute des composants depuis instruction_data
    let (secret_bytes, rest) = instruction_data.split_at(32);
    let (hash_bytes, rest) = rest.split_at(32);
    
    // Extraction des u64/i64 (8 octets chacun, format little-endian)
    let temporal_salt = u64::from_le_bytes(rest[0..8].try_into().unwrap());
    let expiration_time = i64::from_le_bytes(rest[8..16].try_into().unwrap());
    let current_time = i64::from_le_bytes(rest[16..24].try_into().unwrap());

    // Le hash stocké doit être converti en tableau de 32 octets fixe
    let stored_hash_array: [u8; 32] = hash_bytes.try_into().unwrap();

    // 3. Appel au moteur cryptographique pur
    let is_valid = verify_umbrella_shield(
        secret_bytes,
        &stored_hash_array,
        temporal_salt,
        expiration_time,
        current_time,
    );

    // 4. Le Verdict
    if is_valid {
        msg!("Validation Clock In: SUCCES. Le bouclier temporel tient bon.");
        Ok(())
    } else {
        msg!("Validation Clock In: ECHEC. Secret erroné ou temps dépassé.");
        Err(ProgramError::Custom(1)) // 1 = Erreur personnalisée (rejeté)
    }
}

// --- 0x01. STRUCTURE CENTRALE : L'OMBRELLE ---
// Invariant : Famine de données absolue. Aucun identifiant en clair.
pub struct OmbrelleState {
    pub anchor: Pubkey,         // Le point d'ancrage du PDA
    pub blind_hash: [u8; 32],   // Le secret cryptographique masqué
    pub entropy_shield: u64,    // Un sel temporel (Thème "Clock In")
    pub expiration_time: i64,   // Le couperet temporel du Clock In
    pub is_deployed: bool,
}

impl OmbrelleState {
    pub const LEN: usize = 32 + 32 + 8 + 8 + 1; // 81 octets stricts. Bloque l'analyse de taille.
}

// --- 0x02. LOGIQUE DE VALIDATION PURE ---
pub fn verify_umbrella_shield(
    provided_secret: &[u8],
    stored_hash: &[u8; 32],
    temporal_salt: u64,
    expiration_time: i64,
    current_time: i64
) -> bool {

    // 1. Couperet temporel: On rejette instantanément si l'heure est dépassée
    if current_time > expiration_time {
        return false;
    }

    // Concaténation du secret et du sel temporel pour résister aux attaques par dictionnaire
    let mut payload = provided_secret.to_vec();
    payload.extend_from_slice(&temporal_salt.to_le_bytes());

    // Hachage pur via l'implémentation native
    let computed_hash = hash(&payload);

    // Vérification en temps constant (idéalement) ou stricte
    computed_hash.to_bytes() == *stored_hash
}

// --- 0x03. ARÈNE DE TESTS HORS-LIGNE ---
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_umbrella_deployement_and_validation() {
        // 1. Initialisation des paramètres de test
        let original_secret = b"cypherpunk_invariant_null";
        let salt: u64 = 1718293847; // Timestamp simulé

        let deadline: i64 = 1800000000;  // Expiration dans le futur
        let time_now: i64 = 1750000000;  // Temps actuel (Avant l'expiration)
        let time_late: i64 = 1800000001; // Temps en retard (Après l'expiration)
        
        // 2. Génération du hachage de déploiement
        let mut payload = original_secret.to_vec();
        payload.extend_from_slice(&salt.to_le_bytes());
        let expected_hash = hash(&payload).to_bytes();

        // 3. Déploiement simulé de la structure
        let ombrelle = OmbrelleState {
            anchor: Pubkey::new_unique(),
            blind_hash: expected_hash,
            entropy_shield: salt,
            expiration_time: deadline,
            is_deployed: true,
        };

        // 4. Test 1: Clock In validé (Dans le temps + Bon secret) 
        let is_valid = verify_umbrella_shield(original_secret, &ombrelle.blind_hash, ombrelle.entropy_shield, ombrelle.expiration_time, time_now);
        assert!(is_valid, "[SUTURE] Alerte: Échec du Clock In valide.");
        
        // 5. Test d'intrusion (Dans le temps + rejet d'un mauvais secret)
        let fake_secret = b"panoptic_surveillance";
        let is_compromised = verify_umbrella_shield(fake_secret, &ombrelle.blind_hash, ombrelle.entropy_shield, ombrelle.expiration_time, time_now);
        assert!(!is_compromised, "[SUTURE] Alerte Critique: Le bouclier a cédé à une fausse clé.");
        
        // 6. Clock In raté (en retard + Bon secret)
        let is_expired = verify_umbrella_shield(original_secret, &ombrelle.blind_hash, ombrelle.entropy_shield, ombrelle.expiration_time, time_late);
        assert!(!is_expired, "[SUTURE] Alerte: Le validateur a accepté un Clock In expiré.");
    }
}
