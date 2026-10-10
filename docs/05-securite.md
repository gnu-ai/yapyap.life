# Sécurité

## Dispositions en place

### API Rust

- **Mots de passe** : Argon2id (recommandation OWASP), jamais en clair.
- **Jetons de session** : 256 bits aléatoires (CSPRNG), stockés côté
  serveur, expiration configurée, révocables individuellement.
- **SQL** : exclusivement préparé avec paramètres (`$1, $2…`) — aucune
  concaténation, donc pas d'injection SQL.
- **Limite de corps** : `http.max_body_bytes` (middleware axum
  DefaultBodyLimit), rejet 413 ; cohérente avec Nginx.
- **Comptes désactivables** sans suppression (modération réversible).

### Administration PHP

- PDO préparé, échappement HTML systématique (anti-XSS).
- CSRF obligatoire sur tous les formulaires (`hash_equals`).
- Session distincte de l'application, cookie `httponly` + `samesite=Lax`,
  anti-fixation, expiration après inactivité.
- Journal d'audit de toutes les actions ; configuration hors racine web.

### Serveur

- Nginx : HTTPS (Let's Encrypt), proxy limité à `/api/`, restriction IP
  du `/admin`, vitrine servie sans exécution.
- PostgreSQL : écoute 127.0.0.1, authentification `scram-sha-256`,
  comptes dédiés par application.

## Exigences de production

1. **Retirer le CORS permissif** de développement (docs/06-tests-et-dev.md).
2. **Changer tous les mots de passe** (`yapyap_dev` interdit en prod).
3. **Restreindre `/admin`** à l'IP de l'administrateur (allow/deny Nginx).
4. **pg_hba.conf** : `scram-sha-256`, aucun `trust`.
5. **Sauvegardes** : `pg_dump` quotidien + tests de restauration.

## Recommandations (reste à faire)

- Complexité minimale de mot de passe côté API.
- Rate limiting sur inscription et connexion.
- Journal applicatif persistant (tracing + rotation).
- RGPD : export/suppression des données, politique de rétention.
- En-têtes de sécurité HTTP (CSP, HSTS).
