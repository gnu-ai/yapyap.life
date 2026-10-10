# Guide d'administration (back-office PHP)

## Accès

Production : `https://yapyap.life/admin/` (restriction IP Nginx).
Développement : `http://127.0.0.1:8090/connexion.php` (serveur PHP intégré).

Comptes : table `administrateurs` (migration 002), **totalement distincts**
des comptes utilisateurs. Mot de passe haché en Argon2id via
`password_hash()`.

Création d'un compte administrateur :

```sh
php -r 'echo password_hash("MOT_DE_PASSE", PASSWORD_DEFAULT), PHP_EOL;'
psql -h 127.0.0.1 -U yapyap -d yapyap \
  -c "INSERT INTO administrateurs (courriel, mot_de_passe_hash) VALUES ('admin@yapyap.life', 'HASH_GENERE');"
```

## Pages

- `index.php` : tableau de bord (compteurs, 10 dernières inscriptions)
- `utilisateurs.php` : recherche, pagination, activer/désactiver/supprimer
- `sorties.php` : listage avec organisateur et participants, annulation
- `sessions.php` : jetons actifs, révocation unitaire, purge générale
- `journal.php` : historique des actions d'administration

## Pratiques de modération

- **Préférer la désactivation à la suppression** : la suppression est
  irréversible et casse l'historique (cascade sur sorties/participations).
- **La purge générale des sessions** force chaque appareil à se
  reconnecter : réflexe en cas de suspicion de compromission.
- **Toute action est journalisée** (table journal_admin) : la modération
  doit rester auditable.

## Configuration

`/etc/yapyap/admin.php` (hors racine web, 640 root:www-data) :
PostgreSQL, session, pagination. `YAPYAP_ADMIN_CONFIG` surcharge le
chemin en développement.

## Sécurité incluse

Requêtes préparées (PDO), échappement HTML à chaque affichage (anti-XSS),
jeton CSRF obligatoire, régénération d'ID de session à la connexion,
expiration après 1 h d'inactivité, messages d'échec génériques,
POST-redirect-GET.
