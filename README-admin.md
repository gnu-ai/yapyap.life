# Interface d'administration yAp y Ap — guide d'installation

Application PHP classique (aucune dépendance Composer), servie par
**Nginx + PHP-FPM** et lisant directement la base **PostgreSQL** de yAp y Ap.

## Contenu

```
yapyap-admin/
├── config.php                      # configuration (→ /etc/yapyap/admin.php)
├── fonctions.php                   # PDO, auth, CSRF, journalisation
├── connexion.php / deconnexion.php # connexion administrateur
├── entete.php / pied.php            # gabarit commun
├── index.php                       # tableau de bord (statistiques)
├── utilisateurs.php                # modération des comptes
├── sorties.php                     # gestion des sorties
├── sessions.php                    # jetons actifs, révocation, purge
├── journal.php                     # historique des actions
├── style.css
├── migrations/002_admin.sql        # tables administrateurs + journal_admin
└── deploy-nginx-yapyap-admin.conf  # bloc Nginx /admin (PHP-FPM)
```

## 1. Prérequis

```sh
sudo apt install php-fpm php-pgsql   # pilote PDO PostgreSQL indispensable
php -v                               # PHP 8.x recommandé
```

## 2. Migration SQL (tables propres à l'admin)

Depuis la racine du projet Rust :

```sh
psql -h 127.0.0.1 -U yapyap -d yapyap -f yapyap-admin/migrations/002_admin.sql
```

## 3. Création du premier compte administrateur

Le mot de passe est haché par PHP (Argon2id via password_hash), jamais
stocké en clair :

```sh
php -r 'echo password_hash("VOTRE_MOT_DE_PASSE_ADMIN", PASSWORD_DEFAULT), PHP_EOL;'
# copier la chaîne affichée, puis :
psql -h 127.0.0.1 -U yapyap -d yapyap \
  -c "INSERT INTO administrateurs (courriel, mot_de_passe_hash)
      VALUES ('admin@yapyap.life', 'COLLER_LA_CHAINE_ICI');"
```

## 4. Installation des fichiers

```sh
sudo mkdir -p /var/www/yapyap/admin
sudo cp yapyap-admin/*.php yapyap-admin/style.css /var/www/yapyap/admin/
sudo chown -R www-data:www-data /var/www/yapyap/admin

# Configuration HORS racine web (jamais servie !) :
sudo mkdir -p /etc/yapyap
sudo cp yapyap-admin/config.php /etc/yapyap/admin.php
sudo chmod 640 /etc/yapyap/admin.php && sudo chown root:www-data /etc/yapyap/admin.php
sudo nano /etc/yapyap/admin.php        # renseigner le mot de passe PostgreSQL
```

## 5. Nginx + PHP-FPM

```sh
sudo cp yapyap-admin/deploy-nginx-yapyap-admin.conf \
        /etc/nginx/sites-available/yapyap-admin
sudo ln -s /etc/nginx/sites-available/yapyap-admin /etc/nginx/sites-enabled/
# adapter le socket fastcgi_pass à votre version : ls /run/php/
sudo nginx -t && sudo systemctl reload nginx
sudo systemctl enable --now php8.2-fpm
```

## 6. Test local (sans Nginx, serveur PHP intégré)

```sh
cd yapyap-admin
YAPYAP_ADMIN_CONFIG=$(pwd)/config.php php -S 127.0.0.1:8090
# puis ouvrir : http://127.0.0.1:8090/connexion.php
```

## Sécurité — récapitulatif des dispositions incluses

- requêtes préparées partout (aucune concaténation SQL) ;
- échappement HTML systématique à l'affichage (anti-XSS) ;
- jeton CSRF obligatoire sur chaque formulaire ;
- session administrateur distincte des sessions applicatives ;
- expiration automatique après 1 h d'inactivité ;
- restriction IP du /admin dans Nginx (à adapter) ;
- journalisation de toutes les actions de modération ;
- configuration et mots de passe hors racine web.
