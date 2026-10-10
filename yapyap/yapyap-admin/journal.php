<?php
// ==============================================================================
// yAp y Ap — Journal d'administration (journal.php)
// ==============================================================================
// Historique des actions de modération : qui, quoi, quand. Un back-office
// sans journal rend la modération invérifiable ; ce tableau est la mémoire
// de l'administration (cf. table journal_admin, migration 002).
// ==============================================================================

declare(strict_types=1);
$TITRE = 'Journal';
require_once __DIR__ . '/entete.php';

$journal = db()->query(
    "SELECT j.action, j.cree_le, a.courriel AS admin
       FROM journal_admin j
       LEFT JOIN administrateurs a ON a.id = j.admin_id
       ORDER BY j.cree_le DESC
       LIMIT 200"
)->fetchAll();
?>

<h2>Journal des actions <span class="compteur"><?= count($journal) ?></span></h2>

<table>
    <tr><th>Quand</th><th>Administrateur</th><th>Action</th></tr>
    <?php foreach ($journal as $ligne): ?>
    <tr>
        <td><?= e(substr((string) $ligne['cree_le'], 0, 19)) ?></td>
        <td><?= e($ligne['admin'] ?? '(compte supprimé)') ?></td>
        <td><?= e($ligne['action']) ?></td>
    </tr>
    <?php endforeach; ?>
    <?php if (!$journal): ?>
    <tr><td colspan="3" class="vide">Aucune action journalisée pour le moment.</td></tr>
    <?php endif; ?>
</table>

<?php require_once __DIR__ . '/pied.php'; ?>
