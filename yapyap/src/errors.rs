// ==============================================================================
// yAp y Ap — Types d'erreurs applicatifs (errors.rs)
// ==============================================================================
// Philosophie Rust : les erreurs ne sont pas des « exceptions » interceptées
// n'importe où, mais des VALEURS de type explicite, propagées par `?` et
// `Result<T, E>`. Ce fichier définit LE type d'erreur unique de
// l'application, capable de représenter aussi bien :
//   - une erreur du pilote PostgreSQL (tokio-postgres) ;
//   - une erreur du POOL de connexions (plus de connexion disponible) ;
//   - une erreur d'authentification métier ;
//   - une requête cliente malformée.
// ==============================================================================

use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;

/// Erreur applicative unifiée. `#[from]` permet la conversion automatique
/// depuis les types des bibliothèques : `?` fonctionne alors sans collage
/// de code de conversion.
#[derive(Debug, thiserror::Error)]
pub enum AppError {
    /// Erreur émise par le pilote PostgreSQL.
    #[error("erreur base de données : {0}")]
    Db(#[from] tokio_postgres::Error),

    /// Erreur émise par le POOL de connexions (SGBD injoignable au moment
    /// de l'emprunt, plus de connexion disponible...). C'est cette variante
    /// qui manquait : `pool.get().await?` remonte désormais ici sans effort.
    #[error("erreur du pool de connexions : {0}")]
    Pool(#[from] deadpool::managed::PoolError<tokio_postgres::Error>),

    /// Ressource introuvable (profil, sortie...).
    #[error("ressource introuvable : {0}")]
    NotFound(String),

    /// Authentification invalide ou absente (jeton de session expiré...).
    #[error("authentification requise")]
    Unauthorized,

    /// Données envoyées par le client invalides.
    #[error("requête invalide : {0}")]
    BadRequest(String),

    /// Conflit d'unicité (pseudonyme déjà pris, sortie pleine...).
    #[error("conflit : {0}")]
    Conflict(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match &self {
            AppError::Db(e) => {
                tracing::error!("erreur base de données : {e}");
                (StatusCode::INTERNAL_SERVER_ERROR, "erreur interne".to_string())
            }
            AppError::Pool(e) => {
                // Même politique que pour le SGBD : détail interne en journal,
                // message générique au client (aucune fuite d'informations).
                tracing::error!("erreur du pool : {e}");
                (StatusCode::INTERNAL_SERVER_ERROR, "erreur interne".to_string())
            }
            AppError::NotFound(what) => (StatusCode::NOT_FOUND, what.clone()),
            AppError::Unauthorized => (StatusCode::UNAUTHORIZED, "authentification requise".to_string()),
            AppError::BadRequest(why) => (StatusCode::BAD_REQUEST, why.clone()),
            AppError::Conflict(why) => (StatusCode::CONFLICT, why.clone()),
        };

        let body = Json(serde_json::json!({ "erreur": message }));
        (status, body).into_response()
    }
}
