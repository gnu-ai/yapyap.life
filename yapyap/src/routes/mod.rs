// ==============================================================================
// yAp y Ap — Routes HTTP (src/routes/mod.rs)
// ==============================================================================
// Ce module construit le ROUTEUR axum : la carte complète de l'API REST.
// Les handlers sont répartis dans un fichier par domaine fonctionnel
// (auth, users, outings), ce qui maintient chaque fichier lisible et
// favorise le travail en parallèle sur un projet d'équipe.
// ==============================================================================

pub mod auth;
pub mod users;
pub mod outings;

use crate::config::Config;
use axum::routing::{get, post};
use axum::Router;
use axum::extract::DefaultBodyLimit;
use deadpool_postgres::Pool;

/// État applicatif partagé par tous les handlers.
/// Clonable à coût quasi nul (les deux champs le sont).
#[derive(Clone)]
pub struct AppState {
    pub pool: Pool,      // pool de connexions PostgreSQL
    pub cfg: Config,    // configuration chargée au démarrage
}

/// Construit le routeur complet. La structure est « plate » pour la
/// lisibilité : chaque ligne EST une route documentée de l'API.
pub fn build_router(pool: Pool, cfg: Config) -> Router {
    // On lit la limite AVANT de déplacer `cfg` dans l'état : une fois la
    // structure AppState construite, la config n'appartient plus à cette
    // fonction (sémantique de déplacement de Rust). `usize` est un entier
    // copié : cette lecture est gratuite.
    let limite_corps = cfg.http.max_body_bytes;

    let state = AppState { pool, cfg };

    Router::new()
        // --- Santé : sonde utilisée par Nginx et le supervision (Nagios,
        //     Prometheus...) pour vérifier que le service répond.
        .route("/api/sante", get(sante))
        // --- Authentification -------------------------------------------------
        .route("/api/auth/inscription", post(auth::inscription))
        .route("/api/auth/connexion",    post(auth::connexion))
        .route("/api/auth/deconnexion", post(auth::deconnexion))
        // --- Utilisateurs -----------------------------------------------------
        .route("/api/utilisateurs/moi",      get(users::moi)
                                                  .patch(users::maj_profil))
        .route("/api/utilisateurs/proches",  get(users::proches))
        // --- Sorties ----------------------------------------------------------
        .route("/api/sorties",           post(outings::creer)
                                              .get(outings::proches))
        .route("/api/sorties/:id/participer", post(outings::participer))
        .route("/api/sorties/:id/retirer",    post(outings::retirer))
        // Limitation de la taille des corps de requête : les requêtes JSON
        // de l'API sont petites ; tout corps dépassant la limite configurée
        // (http.max_body_bytes) est rejeté en 413 (Payload Too Large) —
        // protection cohérente avec client_max_body_size de la config Nginx.
        .layer(DefaultBodyLimit::max(limite_corps))
        .with_state(state)
}

/// GET /api/sante — répond { "statut": "ok" }. Volontairement minimal :
// la sonde doit fonctionner même si PostgreSQL est momentanément lent.
async fn sante() -> axum::Json<serde_json::Value> {
    axum::Json(serde_json::json!({ "statut": "ok", "service": "yapyap" }))
}
