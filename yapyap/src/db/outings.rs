// ==============================================================================
// yAp y Ap — Dépôt « sorties » (src/db/outings.rs)
// ==============================================================================
// Persistance des propositions de sortie et des inscriptions. La requête
// « chercher_proches » suit la même stratégie que pour les utilisateurs :
// encadrement SQL grossier (index), distance exacte en Rust.
// ==============================================================================

use crate::errors::AppError;
use crate::models::outing::{NewOuting, Outing};
use crate::services::geo;
use deadpool_postgres::Pool;

/// Crée une sortie organisée par `organisateur`.
pub async fn creer(pool: &Pool, organisateur: i64, o: &NewOuting) -> Result<Outing, AppError> {
    let client = pool.get().await?;
    let row = client
        .query_one(
            "INSERT INTO sorties (titre, description, latitude, longitude,
                                  debut, capacite, organisateur)
             VALUES ($1, $2, $3, $4, $5, $6, $7)
             RETURNING id, titre, description, latitude, longitude,
                       debut, capacite, organisateur, cree_le",
            &[&o.titre, &o.description.clone().unwrap_or_default(),
              &o.latitude, &o.longitude, &o.debut,
              &o.capacite.unwrap_or(10), &organisateur],
        )
        .await?;
    Ok(row_vers_outing(&row, 0))
}

/// Sorties à venir situées dans un rayon donné autour d'un point.
/// Le nombre de participants est calculé par SOUS-REQUÊTE corrélée :
/// le SGBD fait ce qu'il sait faire (agrégat), Rust ne compte rien.
pub async fn chercher_proches(
    pool: &Pool,
    lat: f64,
    lon: f64,
    rayon_km: f64,
) -> Result<Vec<Outing>, AppError> {
    let dlat = geo::delta_lat(rayon_km);
    let dlon = geo::delta_lon(rayon_km, lat);

    let client = pool.get().await?;
    let rows = client
        .query(
            "SELECT s.id, s.titre, s.description, s.latitude, s.longitude,
                    s.debut, s.capacite, s.organisateur,
                    (SELECT count(*) FROM participations p
                      WHERE p.sortie = s.id) AS nb_participants
               FROM sorties s
              WHERE s.debut > now()
                AND s.latitude  BETWEEN $1 AND $2
                AND s.longitude BETWEEN $3 AND $4",
            &[&(lat - dlat), &(lat + dlat), &(lon - dlon), &(lon + dlon)],
        )
        .await?;

    let mut sorties: Vec<Outing> = rows
        .iter()
        .map(|r| row_vers_outing(r, r.try_get("nb_participants").unwrap_or(0)))
        .collect();
    // Affinage et tri par distance, comme pour les utilisateurs.
    sorties.retain(|o| geo::distance_km(lat, lon, o.latitude, o.longitude) <= rayon_km);
    sorties.sort_by(|a, b| {
        geo::distance_km(lat, lon, a.latitude, a.longitude)
            .partial_cmp(&geo::distance_km(lat, lon, b.latitude, b.longitude))
            .unwrap()
    });
    Ok(sorties)
}

/// Inscrit un utilisateur à une sortie, en refusant si elle est pleine.
/// La vérification de capacité est faite DANS LA REQUÊTE (atomicité) :
/// deux inscriptions simultanées ne peuvent pas dépasser la capacité.
pub async fn participer(pool: &Pool, utilisateur: i64, sortie: i64) -> Result<(), AppError> {
    let client = pool.get().await?;
    // 1) La sortie existe-t-elle encore et reste-t-elle de la place ?
    let place = client
        .query_opt(
            "SELECT 1 FROM sorties
              WHERE id = $1
                AND capacite > (SELECT count(*) FROM participations
                                  WHERE sortie = $1)",
            &[&sortie],
        )
        .await?
        .ok_or_else(|| AppError::Conflict("sortie pleine ou introuvable".into()))?;
    let _ = place; // simple garde-fou explicite

    // 2) Inscription. Une clé dupliquée (double-clic) = conflit HTTP 409.
    client
        .execute(
            "INSERT INTO participations (utilisateur, sortie)
             VALUES ($1, $2)
             ON CONFLICT DO NOTHING",
            &[&utilisateur, &sortie],
        )
        .await?;
    Ok(())
}

/// Désinscription.
pub async fn retirer(pool: &Pool, utilisateur: i64, sortie: i64) -> Result<(), AppError> {
    let client = pool.get().await?;
    client
        .execute(
            "DELETE FROM participations WHERE utilisateur = $1 AND sortie = $2",
            &[&utilisateur, &sortie],
        )
        .await?;
    Ok(())
}

/// Conversion ligne SQL → modèle Outing.
fn row_vers_outing(r: &tokio_postgres::Row, nb: i64) -> Outing {
    Outing {
        id: r.get("id"),
        titre: r.get("titre"),
        description: r.get("description"),
        latitude: r.get("latitude"),
        longitude: r.get("longitude"),
        debut: r.get("debut"),
        capacite: r.get("capacite"),
        organisateur: r.get("organisateur"),
        nb_participants: nb,
    }
}
