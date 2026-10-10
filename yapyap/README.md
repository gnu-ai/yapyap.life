# yAp y Ap — serveur de rencontres / sorties entre proches

Implémentation Rust du service **yAp y Ap** : mise en relation de personnes
proches géographiquement, avec propositions de sorties localisées.

## Architecture logicielle (rôles)

| Composant         | Rôle                                                                 |
|-------------------|----------------------------------------------------------------------|
| **yapyap (Rust)** | Logique applicative : API REST, sécurité, calcul de distance          |
| **PostgreSQL**    | Logiciel serveur tiers : persistance (comptes, sessions, sorties)     |
| **Nginx**         | Logiciel serveur tiers : TLS, limitation de débit, fichiers statiques |

L'application Rust n'est **jamais exposée directement** : elle écoute sur
`127.0.0.1:8080` et seul Nginx lui transmet les requêtes.

## Arborescence (maintenabilité)

```
yapyap/
├── Cargo.toml               # manifeste des dépendances (Cargo)
├── Makefile                 # build/test/install (POSIX)
├── config/
│   └── yapyap.toml          # configuration (→ /etc/yapyap/)
├── migrations/
│   └── 001_init.sql         # schéma PostgreSQL initial
├── deploy/
│   ├── nginx/yapyap.conf    # proxy inverse + TLS + anti-brute-force
│   ├── init/yapyap.sh       # script d'init POSIX (start/stop/status)
│   └── README-deploiement.md
├── docs/
│   └── api.md               # référence de l'API REST
└── src/
    ├── main.rs              # point d'entrée : config → pool → routeur
    ├── config.rs            # lecture du fichier TOML
    ├── errors.rs            # type d'erreur unique → réponses HTTP
    ├── db/                  # ACCÈS AUX DONNÉES (tout le SQL est ici)
    │   ├── mod.rs           #   pool de connexions PostgreSQL
    │   ├── users.rs         #   requêtes utilisateurs
    │   ├── sessions.rs      #   jetons d'authentification
    │   └── outings.rs       #   sorties et participations
    ├── models/              # CONTRATS DE DONNÉES (aucune logique)
    │   ├── user.rs, outing.rs, session.rs
    ├── services/            # LOGIQUE MÉTIER PURE (testable sans serveur)
    │   ├── auth.rs          #   Argon2, jetons (avec tests unitaires)
    │   └── geo.rs           #   distance de Haversine (avec tests)
    └── routes/              # COUCHE HTTP (traduction requête ↔ métier)
        ├── mod.rs           #   table des routes de l'API
        ├── auth.rs, users.rs, outings.rs
```

**Règle de dépendance** (une seule direction) :
`routes → services → db`, les `models` traversent toutes les couches.
Un bug de distance se corrige dans `services/geo.rs` ; une requête lente
s'optimise dans `db/` ; jamais les deux en même temps.

## Démarrage rapide (développement)

```sh
make run          # compile et lance avec /etc/yapyap/yapyap.toml
# ou, sans installation :
cargo run         # avec YAPYAP_CONFIG=config/yapyap.toml
```

## Déploiement GNU/Linux

Voir `deploy/README-deploiement.md` (création de la base, installation
Nginx, script d'init POSIX). En résumé :

```sh
make install      # binaire → /usr/local/bin, config → /etc/yapyap
make -f Makefile test
/etc/init.d/yapyap start
```

## Tests

```sh
make test   # tests unitaires : hachage, jetons, distance géographique
```
