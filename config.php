<?php
// ==============================================================================
// yAp y Ap — Configuration de l'interface d'administration (config.php)
// ==============================================================================
// Rôle : unique point d'entrée pour tout ce qui varie selon l'environnement.
// À installer en /etc/yapyap/admin.php (hors racine web ! — un fichier de
// configuration ne doit JAMAIS être servi par Nginx, sous peine de livrer
// les identifiants PostgreSQL au premier curieux).
//
// L'interface d'administration dialogue DIRECTEMENT avec PostgreSQL, comme
// l'application Rust : elle ne passe PAS par l'API. Deux lecteurs, une seule
// source de vérité — c'est le modèle « plusieurs clients, un SGBD ».
// ==============================================================================

declare(strict_types=1); // typage strict de PHP 7+ : moins de conversions sournoises

return [
    // --- PostgreSQL (logiciel serveur tiers n°1) ---------------------------
    // Mêmes valeurs que /etc/yapyap/yapyap.toml section [db], mais avec un
    // compte DÉDIÉ à l'admin (privilèges moindres que le propriétaire si
    // possible ; en développement, le compte applicatif suffit).
    'db' => [
        'hote'    => '127.0.0.1',
        'port'    => 5432,
        'base'    => 'yapyap',
        'user'    => 'yapyap',
        'passe'  => 'yapyap_dev',       // CHANGER en production
    ],

    // --- Sécurité de la session administrateur ----------------------------
    'session' => [
        'nom'             => 'YAPYAP_ADMIN',   // nom du cookie de session
        'duree_secondes'  => 3600,            // 1 h d'inactivité = reconnexion
    ],

    // --- Affichage ---------------------------------------------------------
    'affichage' => [
        'lignes_par_page' => 25,               // pagination des listes
    ],
];
