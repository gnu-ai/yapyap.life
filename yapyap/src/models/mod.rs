// ==============================================================================
// yAp y Ap — Modèles de données (src/models/mod.rs)
// ==============================================================================
// Les « modèles » sont les structures Rust qui reflètent les entités métier
// (utilisateur, sortie, participation). Elles servent de contrat unique entre
// les couches : routes (HTTP) ↔ services (logique) ↔ dépôts (SQL).
// ==============================================================================

pub mod user;     // Utilisateur et position géographique
pub mod outing;   // Sortie (proposition de rencontre localisée)
pub mod session;  // Session d'authentification
