# Architecture

## Vue d'ensemble

- Nginx (HTTPS) sert la vitrine et transmet `/api/` vers l'API Rust
  (127.0.0.1:8080) ; `/admin/` est exécuté par PHP-FPM.
- API Rust, admin PHP et vitrine lisent TOUS la même base PostgreSQL :
  trois clients, une seule source de vérité (cohérence, transactions,
  intégrité référentielle au même endroit).

## Couches de l'API Rust

| Couche | Dossier | Responsabilité |
|---|---|---|
| Transport | src/routes/ | routes REST, paramètres, codes HTTP |
| Métier | src/services/ | auth (Argon2, jetons), géographie (Haversine) |
| Accès aux données | src/db/ | SQL préparé, pool de connexions |
| Modèles | src/models/ | structures serde (JSON entrée/sortie) |
| Configuration | src/config.rs | lecture TOML, validation au démarrage |

Règle : chaque couche ne connaît que la couche inférieure. Les handlers
ne contiennent jamais de SQL ; les dépôts ignorent HTTP.

## Choix de conception

- **Jetons opaques + table sessions** (pas de JWT) : révocation immédiate
  (une ligne à supprimer), aucune signature à gérer, compromission
  limitée. Un JWT reste valide jusqu'à expiration même « révoqué ».
- **Distance de Haversine en SQL** : le SGBD filtre et trie par distance,
  passera à l'échelle avec un index géographique (GiST sur point).
- **Argon2id** pour les mots de passe (crate Rust et password_hash PHP).
- **Comptes administrateurs séparés** des comptes utilisateurs
  (table administrateurs, migration 002) : un rôle = une table.
- **Config hors racine web** (/etc/yapyap/) : jamais servie par Nginx.

## Flux d'une requête authentifiée

1. Nginx reçoit `GET /api/utilisateurs/proches...` → proxy vers 127.0.0.1:8080.
2. axum route vers `routes/users.rs::proches`.
3. Le jeton de l'en-tête `Authorization: Bearer` est validé (table sessions).
4. Le SGBD calcule et trie les distances.
5. Réponse JSON sérialisée par les modèles.
