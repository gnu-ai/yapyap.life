// ==============================================================================
// yAp y Ap — Service géographique (src/services/geo.rs)
// ==============================================================================
// Calcul de la distance « à vol d'oiseau » entre deux points terrestres
// donné en latitude/longitude (WGS-84). La formule de HAVERSINE fournit une
// excellente approximation pour des distances de quelques kilomètres à
// quelques centaines de kilomètres — bien suffisant pour organiser des
// sorties de quartier.
//
// Stratégie de recherche « proches » :
//   1. le SGBD fait un FILTRAGE GROSSIER par encadrement de latitude et
//      longitude (indexables, très rapides) ;
//   2. Rust affine en calculant la distance exacte pour chaque candidat.
// Ce partage des rôles (PostgreSQL = filtre brut, Rust = calcul précis)
// est simple et évite d'imposer une extension SIG au SGBD.
// ==============================================================================

/// Rayon moyen de la Terre, en kilomètres (valeur standard WGS-84).
const RAYON_TERRE_KM: f64 = 6_371.0;

/// Distance en kilomètres entre deux points (lat1, lon1) et (lat2, lon2).
/// Les angles sont fournis en DEGRÉS (convention des API grand public) et
/// convertis en radians, unité attendue par les fonctions trigonométriques.
pub fn distance_km(lat1: f64, lon1: f64, lat2: f64, lon2: f64) -> f64 {
    let lat1r = lat1.to_radians();
    let lat2r = lat2.to_radians();
    let dlat  = (lat2 - lat1).to_radians();
    let dlon  = (lon2 - lon1).to_radians();

    // Formule de Haversine :
    //   a = sin²(Δφ/2) + cos φ1 · cos φ2 · sin²(Δλ/2)
    //   c = 2 · atan2(√a, √(1−a))
    //   d = R · c
    let a = (dlat / 2.0).sin().powi(2)
          + lat1r.cos() * lat2r.cos() * (dlon / 2.0).sin().powi(2);
    let c = 2.0 * a.sqrt().atan2((1.0 - a).sqrt());
    RAYON_TERRE_KM * c
}

/// Traduit un rayon en degrés d'encadrement de latitude (grossier).
/// Un degré de latitude ≈ 111 km partout sur le globe.
pub fn delta_lat(radius_km: f64) -> f64 {
    radius_km / 111.0
}

/// Encadrement de longitude, corrigé par le cosinus de la latitude :
/// les parallèles se resserrent vers les pôles. On évite la division
/// par zéro aux pôles en majorant le cosinus par une valeur minimale.
pub fn delta_lon(radius_km: f64, lat: f64) -> f64 {
    let cos = lat.to_radians().cos().abs().max(0.01);
    radius_km / (111.0 * cos)
}

// ------------------------------------------------------------------------------
// Tests unitaires : ils s'exécutent par `cargo test`, SANS PostgreSQL ni
// Nginx — c'est toute la valeur du découpage en services purs.
// ------------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn distance_paris_lyon() {
        // Paris (48.8566, 2.3522) — Lyon (45.7640, 4.8357) ≈ 392 km.
        let d = distance_km(48.8566, 2.3522, 45.7640, 4.8357);
        assert!((d - 392.0).abs() < 5.0, "distance inattendue : {d}");
    }

    #[test]
    fn distance_nulle() {
        assert!(distance_km(48.85, 2.35, 48.85, 2.35) < 1e-9);
    }
}
