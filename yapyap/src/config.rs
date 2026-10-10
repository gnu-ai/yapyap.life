// ==============================================================================
// yAp yAp — Configuration de l'application (config.rs)
// ==============================================================================
// Principe (bonne pratique « douze facteurs ») : tout ce qui varie selon
// l'environnement (adresses, ports, secrets) vit dans un fichier de
// configuration, jamais dans le code. Ici, format TOML.
//
// Exemple de fichier (config/yapyap.toml) :
//   [http]
//   bind = "127.0.0.1"
//   port = 8080
//   [db]
//   host = "127.0.0.1"
//   port = 5432
//   user = "yapyap"
//   password = "secret"
//   dbname = "yapyap"
// ==============================================================================

use serde::Deserialize;

// ------------------------------------------------------------------------------
// Structure racine : miroir exact du fichier TOML (merci à serde, qui dérive
// automatiquement le lecteur de désérialisation).
// ------------------------------------------------------------------------------
#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub http: HttpConfig,   // section [http]
    pub db: DbConfig,      // section [db]
    pub geo: GeoConfig,    // section [geo]
}

/// Paramètres du serveur HTTP interne.
#[derive(Debug, Clone, Deserialize)]
pub struct HttpConfig {
    /// Adresse d'écoute. "127.0.0.1" : seul Nginx (proxy inverse local)
    /// peut dialoguer avec nous — l'application n'est pas exposée directement
    /// sur Internet, c'est Nginx qui termine TLS.
    pub bind: String,
    /// Port TCP d'écoute.
    pub port: u16,
    /// Taille maximale (octets) d'un corps de requête accepté.
    pub max_body_bytes: usize,
}

/// Paramètres de connexion au serveur PostgreSQL (logiciel tiers).
#[derive(Debug, Clone, Deserialize)]
pub struct DbConfig {
    pub host: String,
    pub port: u16,
    pub user: String,
    pub password: String,
    pub dbname: String,
    /// Nombre maximal de connexions ouvertes dans le pool.
    pub max_connections: usize,
}

/// Paramètres de la mise en relation géographique.
#[derive(Debug, Clone, Deserialize)]
pub struct GeoConfig {
    /// Rayon par défaut (kilomètres) de recherche de personnes.
    pub default_radius_km: f64,
    /// Rayon maximal autorisé (pour empêcher les requêtes déraisonnables).
    pub max_radius_km: f64,
}

impl Config {
    /// Lit et interprète le fichier TOML indiqué.
    ///
    /// En cas d'échec (fichier absent, syntaxe invalide, clé manquante),
    /// l'erreur est remontée telle quelle : le programme refusera de démarrer
    /// avec une configuration incomplète — comportement « fail-fast »
    // préféré à un démarrage silencieux avec des valeurs par défaut hasardeuses.
    pub fn load(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let raw = std::fs::read_to_string(path)?;
        let cfg: Config = toml::from_str(&raw)?;
        Ok(cfg)
    }
}
