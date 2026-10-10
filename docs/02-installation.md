# Installation

## Prérequis

PostgreSQL 15+, Rust (rustup, edition 2021), PHP 8.2+ (php-fpm, php-pgsql),
Nginx (production).

## Développement

1. Base :

```sh
sudo -u postgres psql -c "CREATE USER yapyap WITH PASSWORD 'yapyap_dev';"
sudo -u postgres psql -c "CREATE DATABASE yapyap OWNER yapyap;"
psql -h 127.0.0.1 -U yapyap -d yapyap -f yapyap/migrations/001_init.sql
```

Confort : `~/.pgpass` (droits 600) :
`127.0.0.1:5432:yapyap:yapyap:yapyap_dev`

2. Configuration : `yapyap/config/yapyap.toml` — sections [db] et [http].
Le chemin est donné par la variable d'environnement `YAPYAP_CONFIG`.

3. Lancer :

```sh
cd yapyap
make test
YAPYAP_CONFIG=config/yapyap.toml cargo run
curl http://127.0.0.1:8080/api/sante
```

4. Administration (serveur PHP intégré) :

```sh
psql -h 127.0.0.1 -U yapyap -d yapyap -f yapyap/yapyap-admin/migrations/002_admin.sql
php -r 'echo password_hash("MOT_DE_PASSE_ADMIN", PASSWORD_DEFAULT), PHP_EOL;'
# puis INSERT dans administrateurs avec le hash affiché
cd yapyap-admin
YAPYAP_ADMIN_CONFIG=$(pwd)/config.php php -S 127.0.0.1:8090
# → http://127.0.0.1:8090/connexion.php
```

## Production

1. Compte de service :

```sh
sudo useradd --system --home /var/lib/yapyap --shell /usr/sbin/nologin yapyap
```

2. Fichiers : `/etc/yapyap/` (config, MOTS DE PASSE À CHANGER),
`/usr/local/bin/yapyap` (build release), `/var/www/yapyap/admin/` (PHP),
`/etc/yapyap/admin.php` en 640 root:www-data.

3. Démarrage automatique :

```sh
sudo install -m 755 deploy/init/yapyap.sh /etc/init.d/yapyap
sudo update-rc.d yapyap defaults && sudo service yapyap start
```

4. Nginx : copier `deploy/nginx/yapyap.conf` et le bloc /admin de
`yapyap-admin/`, activer les sites, `nginx -t`, reload ; activer php-fpm.

5. Certificats :

```sh
sudo certbot --nginx -d yapyap.life -d www.yapyap.life
```

6. **RETIRER le CORS de développement** avant mise en ligne
(voir docs/06-tests-et-dev.md).
