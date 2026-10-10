// ==============================================================================
// yAp y Ap — Dépôt « sessions » (src/db/sessions.rs)
// ==============================================================================
// Gestion des jetons d'authentification en base : création, vérification,
// révocation. Toute la sécurité de l'API repose sur l'existence du jeton
// dans cette table (jeton = 256 bits aléatoires, voir services/auth.rs).
// ==============================================================================

use crate::errors::AppError;
use crate::services::auth::DUREE_SESSION_S;
use chrono::{Duration, Utc};
use deadpool_postgres::Pool;

/// Crée une session pour un utilisateur et retourne le jeton + expiration.
pub async fn creer(pool: &Pool, utilisateur: i64) -> Result<(String, chrono::DateTime<Utc>), AppError> {
    let jeton = crate::services::auth::generer_jeton();
    let expire = Utc::now() + Duration::seconds(DUREE_SESSION_S);

    let client = pool.get().await?;
    client
        .execute(
            "INSERT INTO sessions (jeton, utilisateur, expire_le)
             VALUES ($1, $2, $3)",
            &[&jeton, &utilisateur, &expire],
        )
        .await?;
    Ok((jeton, expire))
}

/// Vérifie un jeton : retourne l'identifiant de l'utilisateur si le jeton
/// existe et n'a pas expiré, sinon None. Les jetons expirés sont
/// opportunistement purgés à cette occasion (petit ménage régulier, sans
// tâche planifiée supplémentaire).
pub async fn verifier(pool: &Pool, jeton: &str) -> Result<Option<i64>, AppError> {
    let client = pool.get().await?;

    // Ménage : suppression des sessions échues.
    let _ = client
        .execute("DELETE FROM sessions WHERE expire_le < now()", &[])
        .await;

    let row = client
        .query_opt(
            "SELECT utilisateur FROM sessions
              WHERE jeton = $1 AND expire_le > now()",
            &[&jeton],
        )
        .await?;
    Ok(row.map(|r| r.get(0)))
}

/// Révoque une session (déconnexion).
pub async fn supprimer(pool: &Pool, jeton: &str) -> Result<(), AppError> {
    let client = pool.get().await?;
    client.execute("DELETE FROM sessions WHERE jeton = $1", &[&jeton]).await?;
    Ok(())
}
