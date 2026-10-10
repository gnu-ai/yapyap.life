// ==============================================================================
// yAp y Ap — Routes d'authentification (src/routes/auth.rs)
// ==============================================================================
// Cycle de vie du compte : inscription (création + hachage du mot de
// passe), connexion (vérification + émission d'un jeton), déconnexion
// (révocation du jeton). Le jeton circule ensuite dans l'en-tête
// « Authorization: Bearer <jeton> ».
// ==============================================================================

use crate::errors::AppError;
use crate::models::session::TokenResponse;
use crate::models::user::{Credentials, NewUser};
use crate::routes::AppState;
use crate::services::auth;
use crate::db::{users, sessions};
use axum::extract::State;
use axum::http::HeaderMap;
use axum::Json;

/// POST /api/auth/inscription
/// Corps attendu : { pseudonyme, courriel, mot_de_passe, latitude, longitude }
/// Réponse : 201 + profil public créé (jamais le mot de passe, même haché).
pub async fn inscription(
    State(st): State<AppState>,
    Json(nouveau): Json<NewUser>,
) -> Result<(axum::http::StatusCode, Json<crate::models::user::User>), AppError> {
    // Validations minimales côté serveur : ne JAMAIS faire confiance au
    // client (le contrôle du front n'est qu'un confort d'affichage).
    if nouveau.pseudonyme.trim().len() < 3 {
        return Err(AppError::BadRequest("pseudonyme trop court (min. 3)".into()));
    }
    if !nouveau.courriel.contains('@') {
        return Err(AppError::BadRequest("courriel invalide".into()));
    }
    if nouveau.mot_de_passe.len() < 8 {
        return Err(AppError::BadRequest("mot de passe trop court (min. 8)".into()));
    }
    // Coordonnées plausibles ? (WGS-84 : ±90, ±180)
    if !(-90.0..=90.0).contains(&nouveau.latitude)
        || !(-180.0..=180.0).contains(&nouveau.longitude)
    {
        return Err(AppError::BadRequest("coordonnées invalides".into()));
    }

    // Le hachage Argon2 est coûteux (c'est voulu) : il est fait ici, une
    // fois, dans la couche service — le dépôt SQL ne voit qu'une chaîne.
    let hash = auth::hacher_mot_de_passe(&nouveau.mot_de_passe)?;
    let user = users::creer(&st.pool, &nouveau, &hash).await?;

    Ok((axum::http::StatusCode::CREATED, Json(user)))
}

/// POST /api/auth/connexion
/// Corps : { courriel, mot_de_passe }. Réponse : { jeton, expire_le }.
pub async fn connexion(
    State(st): State<AppState>,
    Json(cred): Json<Credentials>,
) -> Result<Json<TokenResponse>, AppError> {
    // 1) Retrouver le compte (s'il existe) et son empreinte.
    let trouve = users::trouver_pour_auth(&st.pool, &cred.courriel).await?;
    let (id, hash) = match trouve {
        Some(x) => x,
        // Message volontairement générique : révéler « compte inexistant »
        // offrirait une énumération des adresses inscrites.
        None => return Err(AppError::Unauthorized),
    };

    // 2) Vérifier le mot de passe contre l'empreinte Argon2.
    if !auth::verifier_mot_de_passe(&hash, &cred.mot_de_passe) {
        return Err(AppError::Unauthorized);
    }

    // 3) Émettre le jeton de session et l'enregistrer en base.
    let (jeton, expire_le) = sessions::creer(&st.pool, id).await?;
    Ok(Json(TokenResponse { jeton, expire_le }))
}

/// POST /api/auth/deconnexion — exige l'en-tête Authorization : Bearer <jeton>.
pub async fn deconnexion(
    State(st): State<AppState>,
    headers: HeaderMap,
) -> Result<axum::http::StatusCode, AppError> {
    let jeton = extraire_jeton(&headers).ok_or(AppError::Unauthorized)?;
    sessions::supprimer(&st.pool, &jeton).await?;
    Ok(axum::http::StatusCode::NO_CONTENT)
}

// ------------------------------------------------------------------------------
// Utilitaire partagé : extraction du jeton de l'en-tête Authorization.
// Les handlers protégés l'appellent ; il est `pub(crate)` pour être visible
// des autres modules de la caisse (crate) sans faire partie de l'API publique.
// ------------------------------------------------------------------------------
pub(crate) fn extraire_jeton(headers: &HeaderMap) -> Option<&str> {
    headers
        .get("authorization")?
        .to_str()
        .ok()?
        .strip_prefix("Bearer ")
}
