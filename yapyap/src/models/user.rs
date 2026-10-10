// ==============================================================================
// yAp y Ap — Modèle « Utilisateur » (src/models/user.rs)
// ==============================================================================
// Représente un compte de l'application. Remarquez la séparation en DEUX
// structures :
//   - UserIn  : ce que le client ENVOIE (création de compte, mise à jour) ;
//   - User    : ce que le serveur RENVOIE (jamais le hachage de mot de passe !).
// Cette discipline évite le défaut classique « je renvoie tout mon enregistrement
// de base de données au client, y compris les champs sensibles ».
// ==============================================================================

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Vue publique d'un utilisateur : ce qui sort de l'API.
#[derive(Debug, Serialize)]
pub struct User {
    pub id: i64,
    pub pseudonyme: String,
    pub courriel: String,
    pub latitude: f64,
    pub longitude: f64,
    pub biographie: String,
    pub cree_le: DateTime<Utc>,
}

/// Données nécessaires à l'inscription.
#[derive(Debug, Deserialize)]
pub struct NewUser {
    pub pseudonyme: String,
    pub courriel: String,
    pub mot_de_passe: String,
    pub latitude: f64,
    pub longitude: f64,
    pub biographie: Option<String>, // champ facultatif
}

/// Données modifiables par l'utilisateur (son profil).
#[derive(Debug, Deserialize)]
pub struct UpdateProfile {
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub biographie: Option<String>,
}

/// Identifiants de connexion.
#[derive(Debug, Deserialize)]
pub struct Credentials {
    pub courriel: String,
    pub mot_de_passe: String,
}
