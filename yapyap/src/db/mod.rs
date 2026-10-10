// ==============================================================================
// yAp yAp — Accès à PostgreSQL (src/db/mod.rs)
// ==============================================================================
// PostgreSQL est un LOGICIEL SERVEUR TIERS : notre application ne gère pas
// elle-même la persistance, elle s'y connecte en client TCP. Ce module
// centralise tout ce qui touche à cette connexion :
//   - construction du pool de connexions (deadpool-postgres) ;
//   - vérification de disponibilité (« ping ») au démarrage ;
//   - exécution des requêtes des différents dépôts (repositories).
//
// Le POOL est essentiel : ouvrir une connexion TCP + authentification
// PostgreSQL coûte cher ; on en ouvre donc un lot réutilisable.
// ==============================================================================

pub mod users;
pub mod sessions;
pub mod outings;

use crate::config::Config;
use deadpool_postgres::{Manager, ManagerConfig, Pool, RecyclingMethod};
use std::str::FromStr;

// ------------------------------------------------------------------------------
// Construit le pool de connexions à partir de la configuration.
// Le type `Pool` est clonable à coût quasi nul (compteur de références) :
// chaque handler reçoit sa copie et pioche une connexion le temps de sa
// requête, puis la restitue au pool.
// ------------------------------------------------------------------------------
pub fn create_pool(cfg: &Config) -> Result<Pool, Box<dyn std::error::Error>> {
    // Chaîne de connexion au format « clé=valeur » compris par PostgreSQL.
    // Exemple : "host=127.0.0.1 port=5432 user=yapyap password=... dbname=yapyap"
    let dsn = format!(
        "host={} port={} user={} password={} dbname={}",
        cfg.db.host, cfg.db.port, cfg.db.user, cfg.db.password, cfg.db.dbname
    );

    // deadpool exige un objet « manager » : il sait comment créer et recycler
    // les connexions. RecyclingMethod::Fast : on teste brièvement la
    // connexion avant réutilisation (compromis performance/robustesse).
    let mgr = Manager::from_config(
        tokio_postgres::Config::from_str(&dsn)?,
        tokio_postgres::NoTls,
        ManagerConfig { recycling_method: RecyclingMethod::Fast },
    );

    let pool = Pool::builder(mgr)
        .max_size(cfg.db.max_connections)
        .build()?;

    Ok(pool)
}

// ------------------------------------------------------------------------------
// Vérifie que le SGBD répond. Appelé UNIQUEMENT au démarrage, afin
// d'échouer tôt et clairement (message explicite pour l'administrateur).
// ------------------------------------------------------------------------------
pub async fn ping(pool: &Pool) -> Result<(), crate::errors::AppError> {
    let client = pool.get().await?;
    client.simple_query("SELECT 1").await?;
    Ok(())
}
