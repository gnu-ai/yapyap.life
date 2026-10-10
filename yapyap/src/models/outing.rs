// ==============================================================================
// yAp y Ap — Modèle « Sortie » (src/models/outing.rs)
// ==============================================================================
// Une sortie est une proposition de rencontre localisée : « pique-nique au
// parc, samedi 15h ». Les personnes proches du point de rendez-vous la
// voient et peuvent s'y inscrire.
// ==============================================================================

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Vue publique d'une sortie.
#[derive(Debug, Serialize)]
pub struct Outing {
    pub id: i64,
    pub titre: String,
    pub description: String,
    pub latitude: f64,
    pub longitude: f64,
    pub debut: DateTime<Utc>,
    pub capacite: i32,
    pub organisateur: i64,
    pub nb_participants: i64, // rempli par la requête SQL (jointure agrégée)
}

/// Données nécessaires à la création d'une sortie.
#[derive(Debug, Deserialize)]
pub struct NewOuting {
    pub titre: String,
    pub description: Option<String>,
    pub latitude: f64,
    pub longitude: f64,
    pub debut: DateTime<Utc>,
    pub capacite: Option<i32>,
}
