-- ==============================================================================
-- yAp yAp — Migration initiale du schéma PostgreSQL (migrations/001_init.sql)
-- ==============================================================================
-- À appliquer UNE FOIS, en tant qu'administrateur du SGBD, avec psql :
--   psql -h 127.0.0.1 -U yapyap -d yapyap -f migrations/001_init.sql
--
-- Le schéma suit le principe « une table = une entité métier » :
--   - utilisateurs : comptes et position géographique déclarée ;
--   - sessions     : jetons d'authentification (aucun mot de passe stocké
--                    en clair : la table ne contient que le hachage Argon2) ;
--   - sorties      : propositions de rencontre/sortie localisées ;
--   - participations : n-n entre utilisateurs et sorties.
-- ==============================================================================

-- ---------------------------------------------------------------- utilisateurs
CREATE TABLE IF NOT EXISTS utilisateurs (
    id          BIGSERIAL PRIMARY KEY,
    pseudonyme  TEXT        NOT NULL UNIQUE,           -- identité publique
    courriel    TEXT        NOT NULL UNIQUE,           -- connexion
    mot_de_passe_hash TEXT NOT NULL,                   -- Argon2id, jamais en clair
    latitude    DOUBLE PRECISION NOT NULL,             -- position déclarée
    longitude   DOUBLE PRECISION NOT NULL,
    biographie  TEXT        NOT NULL DEFAULT '',       -- présentation libre
    actif       BOOLEAN     NOT NULL DEFAULT TRUE,
    cree_le     TIMESTAMPTZ NOT NULL DEFAULT now()
);

-- ------------------------------------------------------------------- sessions
-- Un utilisateur peut posséder plusieurs sessions (téléphone + ordinateur).
CREATE TABLE IF NOT EXISTS sessions (
    jeton       TEXT        PRIMARY KEY,              -- jeton aléatoire (secret)
    utilisateur BIGINT      NOT NULL REFERENCES utilisateurs(id) ON DELETE CASCADE,
    expire_le   TIMESTAMPTZ NOT NULL,
    cree_le     TIMESTAMPTZ NOT NULL DEFAULT now()
);

-- --------------------------------------------------------------------- sorties
CREATE TABLE IF NOT EXISTS sorties (
    id          BIGSERIAL PRIMARY KEY,
    titre       TEXT        NOT NULL,
    description TEXT        NOT NULL DEFAULT '',
    latitude    DOUBLE PRECISION NOT NULL,             -- point de rendez-vous
    longitude   DOUBLE PRECISION NOT NULL,
    debut       TIMESTAMPTZ NOT NULL,                  -- date et heure de la sortie
    capacite    INT         NOT NULL DEFAULT 10,      -- nombre maximum de participants
    organisateur BIGINT     NOT NULL REFERENCES utilisateurs(id) ON DELETE CASCADE,
    cree_le     TIMESTAMPTZ NOT NULL DEFAULT now()
);

-- ---------------------------------------------------------------- participations
-- Table de jointure : qui participe à quelle sortie. L'unicité
-- (utilisateur, sortie) est garantie par la clé primaire composite.
CREATE TABLE IF NOT EXISTS participations (
    utilisateur BIGINT NOT NULL REFERENCES utilisateurs(id) ON DELETE CASCADE,
    sortie      BIGINT NOT NULL REFERENCES sorties(id)     ON DELETE CASCADE,
    inscrit_le  TIMESTAMPTZ NOT NULL DEFAULT now(),
    PRIMARY KEY (utilisateur, sortie)
);

-- Indices utiles aux requêtes fréquentes (recherche par position :
-- filtrage grossier par latitude/longitude avant le calcul précis en Rust).
CREATE INDEX IF NOT EXISTS idx_utilisateurs_position ON utilisateurs (latitude, longitude);
CREATE INDEX IF NOT EXISTS idx_sorties_position     ON sorties (latitude, longitude);
CREATE INDEX IF NOT EXISTS idx_sorties_debut        ON sorties (debut);
