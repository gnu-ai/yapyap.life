// ==============================================================================
// yAp yAp — Point d'entrée du programme (main.rs)
// ==============================================================================
// Rôle de ce fichier (le « sommet » de l'arborescence applicative) :
//   1. initialiser la journalisation (tracing) ;
//   2. charger la configuration depuis le fichier TOML ;
//   3. établir le pool de connexions vers PostgreSQL (logiciel serveur tiers) ;
//   4. construire le routeur HTTP (axum) et l'exposer sur un port TCP.
//
// Conformité POSIX : le programme est un démon classique. Il lit sa
// configuration, journalise sur stderr/stdout (redirigés par le système
// d'init), et se laisse arrêter proprement par le signal SIGTERM.
// ==============================================================================

// Déclarations de modules : chaque `mod` correspond à un fichier .rs de
// l'arborescence. C'est la table des matières du programme.
mod config;      // Lecture du fichier de configuration (config.rs)
mod errors;      // Types d'erreurs applicatives (errors.rs)
mod db;          // Pool de connexions PostgreSQL (db/mod.rs)
mod models;     // Structures de données « métier » (models/)
mod routes;     // Points d'entrée HTTP (routes/)
mod services;   // Logique métier pure (services/)

// ------------------------------------------------------------------------------
// #[tokio::main] : transforme la fonction `main` en un environnement
// asynchrone complet. Sans cette macro, une fonction `async fn main()` ne
// serait pas exécutable : le runtime tokio fournit le réacteur et le
// planificateur de tâches (équivalent d'une machine à événements).
// ------------------------------------------------------------------------------
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // ---- 1. Journalisation ---------------------------------------------------
    // `tracing` émet des événements structurés. Le filtre peut être affiné par
    // la variable d'environnement RUST_LOG (ex. : RUST_LOG=yapyap=debug).
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "yapyap=info".into()),
        )
        .init();

    // ---- 2. Configuration ----------------------------------------------------
    // Le chemin du fichier est fourni par la variable d'environnement
    // YAPYAP_CONFIG ; sinon on retombe sur l'emplacement système POSIX
    // classique /etc/yapyap/yapyap.toml.
    let config_path = std::env::var("YAPYAP_CONFIG")
        .unwrap_or_else(|_| "/etc/yapyap/yapyap.toml".to_string());
    let cfg = config::Config::load(&config_path)?;
    tracing::info!("configuration chargée depuis {}", config_path);

    // ---- 3. Pool PostgreSQL --------------------------------------------------
    // Le SGBD est un logiciel serveur tiers : notre processus n'implémente
    // PAS la persistance lui-même, il s'y connecte par TCP via le pilote
    // tokio-postgres, en réutilisant des connexions déjà établies (pool).
    let pool = db::create_pool(&cfg)?;
    // Vérification immédiate : si PostgreSQL est injoignable, autant
    // s'arrêter net avec un message clair plutôt que de démarrer à l'aveugle.
    db::ping(&pool).await?;
    tracing::info!("connexion PostgreSQL vérifiée ({}):{}", cfg.db.host, cfg.db.port);

    // ---- 4. Routeur HTTP -----------------------------------------------------
    // Le routeur associe chaque « verbe + chemin » à une fonction de traitement
    // (un « handler »). L'état applicatif (pool, configuration) est injecté
    // dans chaque handler via `.with_state(...)` : pas de variable globale,
    // tout est explicite et donc testable.
    let app = routes::build_router(pool.clone(), cfg.clone());

    let addr = format!("{}:{}", cfg.http.bind, cfg.http.port);
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    tracing::info!("yAp yAp écoute sur http://{}", addr);

    // `axum::serve` bloque jusqu'à l'arrêt du processus. En production, le
    // service est placé derrière Nginx (proxy inverse) qui termine TLS et
    // transmet les requêtes — voir deploy/nginx/yapyap.conf.
    axum::serve(listener, app).await?;
    Ok(())
}
