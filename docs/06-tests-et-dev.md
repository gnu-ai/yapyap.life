# Tests et outils de développement

## Tests unitaires

```sh
make test        # ou cargo test, depuis yapyap/
```

| Test | Vérifie |
|---|---|
| services::geo::tests::distance_nulle | distance nulle entre points identiques |
| services::geo::tests::distance_paris_lyon | Haversine Paris–Lyon ≈ 392 km |
| services::auth::tests::hachage_puis_verification | Argon2 : hachage puis vérification |
| services::auth::tests::jetons_distincts | deux générations → jetons différents (CSPRNG) |

## Page de test de l'API (navigateur)

`test-api.html` : page autonome couvrant toute l'API avec formulaires
pré-remplis — inscription, connexion (jeton mémorisé et réinjecté dans
l'en-tête `Authorization: Bearer`), profil, personnes proches, sorties,
participations. Chaque action affiche le statut HTTP et la réponse JSON.

Ouvrir dans Firefox : `Fichier > Ouvrir un fichier…`. L'API Rust doit
tourner sur 127.0.0.1:8080.

## CORS en développement

Une page ouverte en origine `file://` ne peut pas appeler
`http://127.0.0.1:8080` sans autorisation (politique same-origin des
navigateurs). Le développement active donc :

```toml
# Cargo.toml
tower-http = { version = "0.5", features = ["cors"] }
```

```rust
// src/routes/mod.rs, dans build_router, avant .with_state(state)
.layer(tower_http::cors::CorsLayer::permissive())
```

⚠️ **À retirer avant la production** : vitrine et API partagent la même
origine (même Nginx), et cette couche affaiblirait la protection
same-origin pour rien.

## Débogage SQL

```sh
psql "host=127.0.0.1 port=5432 user=yapyap dbname=yapyap password=…" -c "SELECT 1;"
\dt    # lister les tables
```

## Erreurs courantes

| Symptôme | Cause | Remède |
|---|---|---|
| `Os { code: 2, NotFound }` au lancement | config introuvable | vérifier `YAPYAP_CONFIG` et le répertoire courant |
| `28P01 password authentication failed` | mdp rôle ≠ config | `ALTER ROLE yapyap WITH PASSWORD …` + même valeur dans le TOML |
| `{"erreur":"erreur interne"}` sur inscription | tables absentes | appliquer `migrations/001_init.sql` |
| 404 sur `/` dans le navigateur | normal : l'API ne sert pas de pages | tester `/api/sante` |
| « API injoignable » depuis test-api.html | serveur éteint ou CORS absent | relancer `cargo run`, vérifier la couche CORS |
| 404 sur `/admin` ou page blanche | restriction IP Nginx ou PHP-FPM arrêté | vérifier allow/deny, `systemctl status php*-fpm` |
