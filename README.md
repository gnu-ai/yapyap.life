# yAp y Ap — yapyap.life

Application de **mise en relation géographique locale** : les utilisateurs
s'inscrivent avec une position (latitude/longitude), découvrent les personnes
à proximité (distance de Haversine), proposent et rejoignent des sorties.

## Stack technique

| Composant | Technologie | Rôle |
|---|---|---|
| API applicative | Rust (axum, tokio, deadpool-postgres, Argon2) | logique métier, REST JSON, port 8080 |
| Base de données | PostgreSQL | utilisateurs, sessions, sorties, participations |
| Administration | PHP (PDO) | back-office de modération |
| Serveur web | Nginx + PHP-FPM | HTTPS, vitrine, proxy /api/, exécution PHP |

## Démarrage rapide

    sudo -u postgres psql -c "CREATE USER yapyap WITH PASSWORD 'yapyap_dev';"
    sudo -u postgres psql -c "CREATE DATABASE yapyap OWNER yapyap;"
    psql -h 127.0.0.1 -U yapyap -d yapyap -f yapyap/migrations/001_init.sql
    psql -h 127.0.0.1 -U yapyap -d yapyap -f yapyap/yapyap-admin/migrations/002_admin.sql
    make test && YAPYAP_CONFIG=config/yapyap.toml cargo run   # depuis yapyap/

Vérification : curl http://127.0.0.1:8080/api/sante

## Structure

- yapyap/ — API Rust (src/routes, src/services, src/db, src/models),
  migrations SQL, déploiement Nginx/init, Makefile
- yapyap/yapyap-admin/ — interface d'administration PHP
- docs/ — documentation détaillée

## Sécurité

Argon2id, jetons opaques révocables, SQL préparé, CSRF, XSS échappé,
limitation de taille des corps (413), HTTPS, restriction IP /admin.
Retirer le CORS de développement avant la production (cf. docs/).
