# yAp y Ap — yapyap.life

Application de **mise en relation géographique locale** : les utilisateurs
s'inscrivent avec une position (latitude/longitude), découvrent les personnes
à proximité (distance de Haversine), proposent et rejoignent des sorties.

## Stack technique

| Composant | Technologie | Rôle |
|---|---|---|
| API applicative | Rust (axum, tokio, deadpool-postgres, Argon2) | logique métier, REST JSON, port 8080 |
| Base de données | PostgreSQL | utilisateurs, sessions, sorties, participations |
| Administration | PHP (PDO) | back-office de modération, lit le SGBD |
| Serveur web | Nginx + PHP-FPM | HTTPS, vitrine, proxy /api/, exécution PHP |
| Supervision | Nagios / Prometheus | sonde /api/sante |

## Démarrage rapide (développement)

    sudo -u postgres psql -c "CREATE USER yapyap WITH PASSWORD 'yapyap_dev';"
    sudo -u postgres psql -c "CREATE DATABASE yapyap OWNER yapyap;"
    psql -h 127.0.0.1 -U yapyap -d yapyap -f yapyap/migrations/001_init.sql
    psql -h 127.0.0.1 -U yapyap -d yapyap -f yapyap/yapyap-admin/migrations/002_admin.sql
    # adapter le mot de passe [db] dans yapyap/config/yapyap.toml
    cd yapyap && make test && YAPYAP_CONFIG=config/yapyap.toml cargo run

Vérification : curl http://127.0.0.1:8080/api/sante

## Structure

- yapyap/ — API Rust (src/routes, src/services, src/db, src/models),
  migrations SQL, deploy (nginx, init), Makefile
- yapyap/yapyap-admin/ — interface d'administration PHP
- docs/ — documentation détaillée (ce dossier)

## Documentation

- docs/01-architecture.md — couches, flux de données, choix de conception
- docs/02-installation.md — développement et production
- docs/03-api.md — référence complète des routes REST
- docs/04-administration.md — guide du back-office PHP
- docs/05-securite.md — sécurité et exigences de production
- docs/06-tests-et-dev.md — tests, page de test, CORS, dépannage

## État du projet

- API Rust fonctionnelle (4 tests unitaires verts).
- Administration PHP complète (modération, sessions, journal).
- Reste à faire : pages vitrine, notifications, RGPD.

## Sécurité

Argon2id, jetons opaques révocables, SQL préparé, CSRF, XSS échappé,
limitation de taille des corps (413), HTTPS, restriction IP /admin.
Retirer le CORS de développement avant la production (cf. docs/06).
