<?php
// ==============================================================================
// yAp y Ap — Tableau de bord (index.php)
// ==============================================================================
// Vue d'ensemble : compteurs essentiels, dernières inscriptions, dernières
// actions d'administration. Un tableau de bord se limite à des REQUÊTES
// LÉGÈRES (count, order by desc limit) : jamais de calcul lourd, sinon la
// page d'accueil du back-office devient la plus lente du site.
// ==============================================================================

declare(strict_types=1);
$TITRE = 'Tableau de bord';
require_once __DIR__ . '/entete.php';

// --- Compteurs (une requête agrégée par table, SGBD = fait pour ça) ---------
$stats = db()->query(
    "SELECT
        (SELECT count(*) FROM utilisateurs WHERE actif)              AS utilisateurs_actifs,
        (SELECT count(*) FROM utilisateurs WHERE NOT actif)         AS utilisateurs_inactifs,
        (SELECT count(*) FROM sorties WHERE debut > now())          AS sorties_a_venir,
        (SELECT count(*) FROM sorties WHERE debut <= now())         AS sorties_passees,
        (SELECT count(*) FROM participations)                        AS participations,
        (SELECT count(*) FROM sessions WHERE expire_le > now())      AS sessions_actives"
)->fetch();
?>

<h2>Vue d'ensemble</h2>

<div class="cartes">
    <div class="carte">
        <span class="valeur"><?= (int) $stats['utilisateurs_actifs'] ?></span>
        <span class="etiquette">utilisateurs actifs</span>
    </div>
    <div class="carte">
        <span class="valeur"><?= (int) $stats['utilisateurs_inactifs'] ?></span>
        <span class="etiquette">désactivés</span>
    </div>
    <div class="carte">
        <span class="valeur"><?= (int) $stats['sorties_a_venir'] ?></span>
        <span class="etiquette">sorties à venir</span>
    </div>
    <div class="carte">
        <span class="valeur"><?= (int) $stats['participations'] ?></span>
        <span class="etiquette">participations</span>
    </div>
    <div class="carte">
        <span class="valeur"><?= (int) $stats['sessions_actives'] ?></span>
        <span class="etiquette">sessions ouvertes</span>
    </div>
</div>

<?php
// --- Dernières inscriptions (10 plus récentes) ------------------------------
$derniers = db()->query(
    'SELECT id, pseudonyme, courriel, latitude, longitude, cree_le
       FROM utilisateurs ORDER BY cree_le DESC LIMIT 10'
)->fetchAll();
?>
<h2>Dernières inscriptions</h2>
<table>
    <tr><th>Id</th><th>Pseudonyme</th><th>Courriel</th><th>Position</th><th>Inscrit le</th></tr>
    <?php foreach ($derniers as $u): ?>
    <tr>
        <td><?= (int) $u['id'] ?></td>
        <td><?= e($u['pseudonyme']) ?></td>
        <td><?= e($u['courriel']) ?></td>
        <td><?= e(sprintf('%.3f, %.3f', (float) $u['latitude'], (float) $u['longitude'])) ?></td>
        <td><?= e(substr((string) $u['cree_le'], 0, 16)) ?></td>
    </tr>
    <?php endforeach; ?>
    <?php if (!$derniers): ?>
    <tr><td colspan="5" class="vide">Aucun utilisateur pour le moment.</td></tr>
    <?php endif; ?>
</table>

<?php require_once __DIR__ . '/pied.php'; ?>
