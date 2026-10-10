// ==============================================================================
// yAp y Ap — Modèle « Session » (src/models/session.rs)
// ==============================================================================
// Une session matérialise une connexion authentifiée : le client reçoit un
// jeton secret (chaîne aléatoire) et le présente à chaque requête dans
// l'en-tête HTTP « Authorization: Bearer <jeton> ». Le serveur vérifie en
// base que le jeton existe et n'a pas expiré (voir src/db/sessions.rs —
// qui renvoie directement le couple (jeton, expiration), d'où l'absence
// de structure intermédiaire ici).
// ==============================================================================

use chrono::{DateTime, Utc};
use serde::Serialize;

/// Réponse renvoyée au client à la connexion : le jeton à conserver.
#[derive(Debug, Serialize)]
pub struct TokenResponse {
    pub jeton: String,
    pub expire_le: DateTime<Utc>,
}
