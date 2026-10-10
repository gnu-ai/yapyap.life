// ==============================================================================
// yAp y Ap — Routes sorties (src/routes/outings.rs)
// ==============================================================================
// Créer une sortie (proposition de rencontre localisée), découvrir les
// sorties proches de soi, s'y inscrire ou s'en retirer.
// ==============================================================================

use crate::db::outings;
use crate::errors::AppError;
use crate::models::outing::NewOuting;
use crate::routes::users::utilisateur_courant;
use crate::routes::AppState;
use axum::extract::{Path, Query, State};
use axum::http::{HeaderMap, StatusCode};
use axum::Json;
use serde_json::json;

/// POST /api/sorties — crée une sortie (organisateur = appelant).
pub async fn creer(
    State(st): State<AppState>,
    headers: HeaderMap,
    Json(nouvelle): Json<NewOuting>,
) -> Result<(StatusCode, Json<crate::models::outing::Outing>), AppError> {
    let organisateur = utilisateur_courant(&st, &headers).await?;

    // Validations métier : titre exploitable, capacité raisonnable.
    if nouvelle.titre.trim().is_empty() {
        return Err(AppError::BadRequest("titre obligatoire".into()));
    }
    if !(-90.0..=90.0).contains(&nouvelle.latitude)
        || !(-180.0..=180.0).contains(&nouvelle.longitude)
    {
        return Err(AppError::BadRequest("coordonnées invalides".into()));
    }
    if let Some(c) = nouvelle.capacite {
        if !(1..=1000).contains(&c) {
            return Err(AppError::BadRequest("capacité doit être comprise entre 1 et 1000".into()));
        }
    }

    let sortie = outings::creer(&st.pool, organisateur, &nouvelle).await?;
    Ok((StatusCode::CREATED, Json(sortie)))
}

/// Paramètres d'URL de la recherche : ?latitude=..&longitude=..&rayon=..
#[derive(Debug, serde::Deserialize)]
pub struct ParamsSorties {
    pub latitude: f64,
    pub longitude: f64,
    pub rayon: Option<f64>,
}

/// GET /api/sorties — sorties à venir autour d'un point.
pub async fn proches(
    State(st): State<AppState>,
    headers: HeaderMap,
    Query(p): Query<ParamsSorties>,
) -> Result<Json<serde_json::Value>, AppError> {
    utilisateur_courant(&st, &headers).await?;
    let rayon = p.rayon.unwrap_or(st.cfg.geo.default_radius_km).min(st.cfg.geo.max_radius_km);
    let liste = outings::chercher_proches(&st.pool, p.latitude, p.longitude, rayon).await?;
    Ok(Json(json!({ "rayon_km": rayon, "sorties": liste })))
}

/// POST /api/sorties/{id}/participer — inscription à une sortie.
/// Le garde 409 « sortie pleine » est appliqué côté dépôt (atomicité SQL).
pub async fn participer(
    State(st): State<AppState>,
    headers: HeaderMap,
    Path(id): Path<i64>,
) -> Result<StatusCode, AppError> {
    let utilisateur = utilisateur_courant(&st, &headers).await?;
    outings::participer(&st.pool, utilisateur, id).await?;
    Ok(StatusCode::NO_CONTENT)
}

/// POST /api/sorties/{id}/retirer — désinscription.
pub async fn retirer(
    State(st): State<AppState>,
    headers: HeaderMap,
    Path(id): Path<i64>,
) -> Result<StatusCode, AppError> {
    let utilisateur = utilisateur_courant(&st, &headers).await?;
    outings::retirer(&st.pool, utilisateur, id).await?;
    Ok(StatusCode::NO_CONTENT)
}
