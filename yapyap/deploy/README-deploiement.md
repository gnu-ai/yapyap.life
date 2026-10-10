# Déploiement yAp y Ap sous GNU/Linux

Procédure complète, chaque étape est volontairement explicite : un
administrateur doit pouvoir reproduire toutes les opérations à la main.

## 1. Prérequis logiciels

```sh
# Debian/Ubuntu
apt install build-essential pkg-config curl nginx postgresql
curl https://sh.rustup.rs -sSf | sh      # chaîne Rust officielle
```

## 2. Base de données PostgreSQL (logiciel serveur tiers)

```sh
# Compte applicatif dédié — jamais « postgres » en super-admin.
sudo -u postgres psql <<'EOF'
CREATE USER yapyap WITH PASSWORD 'CHANGER_CE_MOT_DE_PASSE';
CREATE DATABASE yapyap OWNER yapyap;
EOF

# Application du schéma initial :
psql -h 127.0.0.1 -U yapyap -d yapyap -f migrations/001_init.sql
```

Authentification : le fichier `pg_hba.conf` doit autoriser
`host yapyap yapyap 127.0.0.1/32 scram-sha-256` (mot de passe transmis par
le pool de connexions de l'application).

## 3. Utilisateur système non privilégié

```sh
useradd --system --no-create-home --shell /usr/sbin/nologin yapyap
```

## 4. Installation du programme

```sh
make install          # → /usr/local/bin/yapyap + /etc/yapyap/yapyap.toml
chmod 640 /etc/yapyap/yapyap.toml && chown root:yapyap /etc/yapyap/yapyap.toml
```

Éditer `/etc/yapyap/yapyap.toml` : y reporter le vrai mot de passe PostgreSQL.

## 5. Script d'init POSIX

```sh
install -m 755 deploy/init/yapyap.sh /etc/init.d/yapyap
update-rc.d yapyap defaults          # Debian ; équivalent sur les autres distros
/etc/init.d/yapyap start
```

## 6. Nginx (proxy inverse + TLS)

```sh
cp deploy/nginx/yapyap.conf /etc/nginx/sites-available/yapyap
ln -s /etc/nginx/sites-available/yapyap /etc/nginx/sites-enabled/
# Zone anti-brute-force (déjà commentée dans le fichier, à décommenter
# dans le contexte http de nginx.conf) :
#   limit_req_zone $binary_remote_addr zone=auth:10m rate=5r/s;
mkdir -p /var/www/yapyap             # pages statiques du site vitrine
nginx -t && systemctl reload nginx
```

## 7. Vérifications finales

```sh
curl -s http://127.0.0.1:8080/api/sante    # l'application répond
curl -s https://yapyap.life/api/sante      # à travers Nginx (TLS)
tail -f /var/log/nginx/error.log           # en cas de souci
```
