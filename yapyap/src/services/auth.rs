// ==============================================================================
// yAp y Ap — Service d'authentification (src/services/auth.rs)
// ==============================================================================
// Deux responsabilités :
//   1. HACHER les mots de passe (Argon2id, la référence actuelle) : le mot
//      de passe n'est JAMAIS stocké, seulement son empreinte + un sel ;
//   2. GÉNÉRER les jetons de session : chaînes aléatoires de 256 bits,
//      imprévisibles grâce au générateur cryptographique du système.
//
// Rappel « pourquoi Argon2 et pas SHA-256 seul » : un hachage rapide est
// facile à attaquer par force brute ; Argon2 est volontairement LENT et
// gourmand en mémoire, ce qui décourage les attaques massives.
// ==============================================================================

use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use rand::Rng;

/// Durée de validité d'une session : 30 jours (en secondes).
pub const DUREE_SESSION_S: i64 = 30 * 24 * 3600;

/// Calcule l'empreinte Argon2id d'un mot de passe (génère un sel unique).
/// Retourne la chaîne PHC complète, prête à stocker telle quelle dans
/// la colonne mot_de_passe_hash.
pub fn hacher_mot_de_passe(mot_de_passe: &str) -> Result<String, crate::errors::AppError> {
    let sel = SaltString::generate(&mut OsRng);          // sel aléatoire cryptographique
    Argon2::default()
        .hash_password(mot_de_passe.as_bytes(), &sel)
        .map(|h| h.to_string())
        .map_err(|e| crate::errors::AppError::BadRequest(format!("hachage impossible : {e}")))
}

/// Vérifie un mot de passe candidat contre une empreinte stockée.
/// `Ok(true)` = connexion autorisée ; toute erreur de format d'empreinte
/// est traitée comme un échec (on refuse, on ne devine pas).
pub fn verifier_mot_de_passe(
    stocke: &str,
    propose: &str,
) -> bool {
    match PasswordHash::new(stocke) {
        Ok(hash) => Argon2::default()
            .verify_password(propose.as_bytes(), &hash)
            .is_ok(),
        Err(_) => false,
    }
}

/// Génère un jeton de session : 32 octets (256 bits) aléatoires, encodés en
/// hexadécimal (64 caractères). L'entropie rend l'énumération exhaustive
/// hors de portée.
pub fn generer_jeton() -> String {
    let mut octets = [0u8; 32];
    rand::thread_rng().fill(&mut octets);
    octets.iter().map(|b| format!("{b:02x}")).collect()
}

// ------------------------------------------------------------------------------
// Tests unitaires — aucune dépendance externe requise.
// ------------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hachage_puis_verification() {
        let h = hacher_mot_de_passe("s3cr3t!").unwrap();
        assert!(verifier_mot_de_passe(&h, "s3cr3t!"));
        assert!(!verifier_mot_de_passe(&h, "mauvais"));
    }

    #[test]
    fn jetons_distincts() {
        assert_ne!(generer_jeton(), generer_jeton());
        assert_eq!(generer_jeton().len(), 64);
    }
}
