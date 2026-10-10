// ==============================================================================
// yAp y Ap — Routes utilisateurs (src/routes/users.rs)
// ==============================================================================
// Consultation et mise à jour de son propre profil, et surtout la
// fonctionnalité cœur de yAp y Ap : découvrir les personnes proches.
// ==============================================================================

use crate::db::{sessions, users};
use crate::errors::AppError;
use crate::models::user::UpdateProfile;
use crate::routes::auth::extraire_jeton;
use crate::routes::AppState;
use axum::extract::{Query, State};
use axum::http::HeaderMap;
use axum::Json;
use serde::Deserialize;
use serde_json::json;

/// GET /api/utilisateurs/moi — profil de l'utilisateur authentifié.
pub async fn moi(
    State(st): State<AppState>,
    headers: HeaderMap,
) -> Result<Json<crate::models::user::User>, AppError> {
    let id = utilisateur_courant(&st, &headers).await?;
    let user = users::trouver_par_id(&st.pool, id).await?;
    Ok(Json(user))
}

/// PATCH /api/utilisateurs/moi — mise à jour position / biographie.
/// Seuls les champs fournis (non null) sont modifiés (sémantique PATCH).
pub async fn maj_profil(
    State(st): State<AppState>,
    headers: HeaderMap,
    Json(maj): Json<UpdateProfile>,
) -> Result<Json<crate::models::user::User>, AppError> {
    let id = utilisateur_courant(&st, &headers).await?;
    let user = users::mettre_a_jour(&st.pool, id, maj.latitude, maj.longitude, maj.biographie).await?;
    Ok(Json(user))
}

/// Paramètres d'URL de la recherche de proximité :
/// ?latitude=48.85&longitude=2.35&rayon=5
#[derive(Debug, Deserialize)]
pub struct ParamsProches {
    pub latitude: f64,
    pub longitude: f64,
    pub rayon: Option<f64>, // kilomètres ; facultatif (valeur par défaut)
}

/// GET /api/utilisateurs/proches — personnes à moins de `rayon` km.
/// Réponse enrichie : chaque personne est accompagnée de sa distance.
pub async fn proches(
    State(st): State<AppState>,
    headers: HeaderMap,
    Query(p): Query<ParamsProches>,
) -> Result<Json<serde_json::Value>, AppError> {
    // Route protégée : jeton obligatoire.
    utilisateur_courant(&st, &headers).await?;

    // Le rayon demandé est borné par la configuration (max_radius_km)
    // pour protéger la base contre les requêtes déraisonnables
    // (ex. rayon = 20000 km = table entière à scanner).
    let rayon = p
        .rayon
        .unwrap_or(st.cfg.geo.default_radius_km)
        .min(st.cfg.geo.max_radius_km);

    let liste = users::chercher_proches(&st.pool, p.latitude, p.longitude, rayon).await?;

    // Mise en forme de la réponse : tableau d'objets
    // { ...profil..., "distance_km": 1.7 }.
    let items: Vec<_> = liste
        .into_iter()
        .map(|(u, d)| {
            json!({
                "id": u.id,
                "pseudonyme": u.pseudonyme,
                "latitude": u.latitude,
                "longitude": u.longitude,
                "biographie": u.biographie,
                "distance_km": (d * 10.0).round() / 10.0, // arrondi à 100 m
            })
        })
        .collect();

    Ok(Json(json!({ "rayon_km": rayon, "personnes": items })))
}

// ------------------------------------------------------------------------------
// Garde d'authentification commun : vérifie le jeton et retourne l'id.
// Une requête non authentifiée est rejetée en 401 avant toute logique.
// ------------------------------------------------------------------------------
pub(crate) async fn utilisateur_courant(
    st: &AppState,
    headers: &HeaderMap,
) -> Result<i64, AppError> {
    let jeton = extraire_jeton(headers).ok_or(AppError::Unauthorized)?;
    sessions::verifier(&st.pool, jeton)
        .await?
        .ok_or(AppError::Unauthorized)
}
