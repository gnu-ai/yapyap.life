-- ==============================================================================
-- yAp y Ap — Migration 002 : comptes administrateurs (migrations/002_admin.sql)
-- ==============================================================================
-- L'interface d'administration PHP possède SES PROPRES comptes, totalement
-- distincts des comptes utilisateurs de l'application Rust. Un administrateur
-- n'est PAS un utilisateur : la confusion des deux rôles dans une même table
-- est une erreur de conception classique (escalade de privilèges facilitée).
--
-- Application :
--   psql -h 127.0.0.1 -U yapyap -d yapyap -f migrations/002_admin.sql
--
-- Puis création du PREMIER compte administrateur (voir README-admin.md,
-- section « Premier compte » — le mot de passe est haché par PHP en ligne
-- de commande, jamais stocké en clair).
-- ==============================================================================

CREATE TABLE IF NOT EXISTS administrateurs (
    id                  BIGSERIAL PRIMARY KEY,
    courriel            TEXT        NOT NULL UNIQUE,
    mot_de_passe_hash   TEXT        NOT NULL,        -- password_hash() PHP (Argon2id)
    cree_le            TIMESTAMPTZ NOT NULL DEFAULT now()
);

-- Journal des actions d'administration : qui a fait quoi, et quand.
-- Indispensable pour l'audit d'un back-office (traçabilité POSIX-spirit).
CREATE TABLE IF NOT EXISTS journal_admin (
    id          BIGSERIAL PRIMARY KEY,
    admin_id    BIGINT      NOT NULL REFERENCES administrateurs(id) ON DELETE SET NULL,
    action      TEXT        NOT NULL,               -- ex. « utilisateur 42 désactivé »
    cree_le     TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX IF NOT EXISTS idx_journal_admin_date ON journal_admin (cree_le DESC);
