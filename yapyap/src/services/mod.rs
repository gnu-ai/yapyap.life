// ==============================================================================
// yAp y Ap — Services (src/services/mod.rs)
// ==============================================================================
// La couche « services » contient la logique métier PURE : aucun SQL, aucun
// HTTP. Elle manipule des modèles et des primitives. C'est ce découpage qui
// rend le code maintenable et testable : on peut vérifier la logique de
// hachage ou de distance géographique SANS serveur ni base de données.
// ==============================================================================

pub mod geo;      // Distances géographiques (formule de Haversine)
pub mod auth;     // Hachage de mot de passe et jetons de session
