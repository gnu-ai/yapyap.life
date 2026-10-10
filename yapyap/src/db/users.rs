// ==============================================================================
// yAp y Ap — Dépôt « utilisateurs » (src/db/users.rs)
// ==============================================================================
// Un « dépôt » (repository) concentre TOUT le SQL relatif à une entité.
// Les autres couches ne voient jamais une requête : elles appellent des
// fonctions au nom métier (creer, trouver_par_id, chercher_proches...).
//
// La recherche géographique applique la stratégie commentée dans
// services/geo.rs : encadrement grossier en SQL (avec index), calcul
// précis de la distance en Rust, tri par distance croissante.
// ==============================================================================

use crate::errors::AppError;
use crate::models::user::{NewUser, User};
use crate::services::geo;
use deadpool_postgres::Pool;

/// Crée un compte utilisateur. Le mot de passe arrive DÉJÀ HACHÉ par la
/// couche service ; ce dépôt ne sait même pas ce qu'est un mot de passe.
pub async fn creer(pool: &Pool, u: &NewUser, hash: &str) -> Result<User, AppError> {
    let client = pool.get().await?;
    let row = client
        .query_one(
            "INSERT INTO utilisateurs
                (pseudonyme, courriel, mot_de_passe_hash,
                 latitude, longitude, biographie)
             VALUES ($1, $2, $3, $4, $5, $6)
             RETURNING id, pseudonyme, courriel, latitude, longitude,
                       biographie, cree_le",
            &[&u.pseudonyme, &u.courriel, &hash,
              &u.latitude, &u.longitude, &u.biographie.clone().unwrap_or_default()],
        )
        .await?;
    Ok(row_vers_user(&row))
}

/// Recherche un utilisateur par identifiant.
pub async fn trouver_par_id(pool: &Pool, id: i64) -> Result<User, AppError> {
    let client = pool.get().await?;
    let row = client
        .query_opt(
            "SELECT id, pseudonyme, courriel, latitude, longitude,
                    biographie, cree_le
               FROM utilisateurs WHERE id = $1",
            &[&id],
        )
        .await?;
    row.map(|r| row_vers_user(&r)).ok_or_else(|| AppError::NotFound("utilisateur introuvable".into()))
}

/// Recherche par courriel — utilisée pour la connexion.
/// Retourne (id, hash du mot de passe) : le minimum pour authentifier.
pub async fn trouver_pour_auth(pool: &Pool, courriel: &str)
    -> Result<Option<(i64, String)>, AppError>
{
    let client = pool.get().await?;
    let row = client
        .query_opt(
            "SELECT id, mot_de_passe_hash FROM utilisateurs
              WHERE courriel = $1 AND actif = TRUE",
            &[&courriel],
        )
        .await?;
    Ok(row.map(|r| (r.get(0), r.get(1))))
}

/// Met à jour la position et/ou la biographie. Champs `None` = inchangés.
pub async fn mettre_a_jour(
    pool: &Pool,
    id: i64,
    lat: Option<f64>,
    lon: Option<f64>,
    bio: Option<String>,
) -> Result<User, AppError> {
    let client = pool.get().await?;
    let row = client
        .query_one(
            "UPDATE utilisateurs SET
                latitude   = COALESCE($2, latitude),
                longitude = COALESCE($3, longitude),
                biographie = COALESCE($4, biographie)
              WHERE id = $1
              RETURNING id, pseudonyme, courriel, latitude, longitude,
                        biographie, cree_le",
            &[&id, &lat, &lon, &bio],
        )
        .await?;
    Ok(row_vers_user(&row))
}

/// Recherche des personnes proches d'un point, dans un rayon donné.
/// Étape SQL : encadrement de latitude/longitude (utilise les index).
/// Étape Rust : distance exacte Haversine, filtrage et tri.
pub async fn chercher_proches(
    pool: &Pool,
    lat: f64,
    lon: f64,
    rayon_km: f64,
) -> Result<Vec<(User, f64)>, AppError> {
    let dlat = geo::delta_lat(rayon_km);
    let dlon = geo::delta_lon(rayon_km, lat);
    let lat_min = lat - dlat;
    let lat_max = lat + dlat;
    let lon_min = lon - dlon;
    let lon_max = lon + dlon;

    let client = pool.get().await?;
    let rows = client
        .query(
            "SELECT id, pseudonyme, courriel, latitude, longitude,
                    biographie, cree_le
               FROM utilisateurs
              WHERE actif = TRUE
                AND latitude  BETWEEN $1 AND $2
                AND longitude BETWEEN $3 AND $4",
            &[&lat_min, &lat_max, &lon_min, &lon_max],
        )
        .await?;

    // Affinage : on garde ceux réellement à distance ≤ rayon, avec leur
    // distance, puis on trie du plus proche au plus lointain.
    let mut proches: Vec<(User, f64)> = rows
        .iter()
        .map(|r| {
            let u = row_vers_user(r);
            let d = geo::distance_km(lat, lon, u.latitude, u.longitude);
            (u, d)
        })
        .filter(|(_, d)| *d <= rayon_km)
        .collect();
    proches.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
    Ok(proches)
}

/// Conversion ligne SQL → modèle User : unique endroit où les indices de
/// colonnes apparaissent ; toute évolution de requête se fait ici.
fn row_vers_user(r: &tokio_postgres::Row) -> User {
    User {
        id: r.get("id"),
        pseudonyme: r.get("pseudonyme"),
        courriel: r.get("courriel"),
        latitude: r.get("latitude"),
        longitude: r.get("longitude"),
        biographie: r.get("biographie"),
        cree_le: r.get("cree_le"),
    }
}
