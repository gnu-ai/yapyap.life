<?php
// ==============================================================================
// yAp y Ap — Gestion des sorties (sorties.php)
// ==============================================================================
// Liste des propositions de sortie avec leur organisateur, leur nombre de
// participants (sous-requête corrélée, même approche que l'API Rust) et
// les actions de modération : annuler une sortie, la laisser expirer.
// ==============================================================================

declare(strict_types=1);
$TITRE = 'Sorties';
require_once __DIR__ . '/entete.php';

$action  = $_POST['action'] ?? null;
$idCible = (int) ($_POST['id'] ?? 0);
$parPage = (int) ($CONFIG['affichage']['lignes_par_page']);
$page    = max(1, (int) ($_GET['page'] ?? 1));

// --- Traitement des actions ---------------------------------------------------
if ($action === 'supprimer' && $idCible > 0) {
    verifier_csrf();
    db()->prepare('DELETE FROM sorties WHERE id = $1')->execute([$idCible]);
    journaliser("sortie $idCible annulée par un administrateur");
    header('Location: sorties.php?page=' . $page);
    exit;
}

// --- Comptage + page courante -------------------------------------------------
$reqC = db()->query('SELECT count(*) FROM sorties');
$nb = (int) $reqC->fetchColumn();
$pages = max(1, (int) ceil($nb / $parPage));
$page = min($page, $pages);

// --- Liste paginée : la jointure récupère le pseudonyme de l'organisateur,
// la sous-requête corrélée le nombre de participants.
$req = db()->prepare(
    "SELECT s.id, s.titre, s.description, s.debut, s.capacite, s.organisateur,
            u.pseudonyme AS organisateur_nom,
            (SELECT count(*) FROM participations p WHERE p.sortie = s.id) AS nb
       FROM sorties s
       JOIN utilisateurs u ON u.id = s.organisateur
       ORDER BY s.debut DESC
       LIMIT $1 OFFSET $2"
);
$req->execute([$parPage, ($page - 1) * $parPage]);
$sorties = $req->fetchAll();
?>

<h2>Sorties <span class="compteur"><?= $nb ?></span></h2>

<table>
    <tr>
        <th>Id</th><th>Titre</th><th>Organisateur</th><th>Début</th>
        <th>Participants</th><th>Statut</th><th>Actions</th>
    </tr>
    <?php foreach ($sorties as $s): ?>
    <tr>
        <td><?= (int) $s['id'] ?></td>
        <td><?= e($s['titre']) ?></td>
        <td><?= e($s['organisateur_nom']) ?></td>
        <td><?= e(substr((string) $s['debut'], 0, 16)) ?></td>
        <td><?= (int) $s['nb'] ?> / <?= (int) $s['capacite'] ?></td>
        <td><?= strtotime((string) $s['debut']) > time() ? 'à venir' : 'passée' ?></td>
        <td class="actions">
            <form method="post"
                  onsubmit="return confirm('Annuler cette sortie et prévenir son organisateur ?');">
                <input type="hidden" name="csrf" value="<?= e(jeton_csrf()) ?>">
                <input type="hidden" name="id" value="<?= (int) $s['id'] ?>">
                <input type="hidden" name="action" value="supprimer">
                <button type="submit" class="dangereux">Annuler</button>
            </form>
        </td>
    </tr>
    <?php endforeach; ?>
    <?php if (!$sorties): ?>
    <tr><td colspan="7" class="vide">Aucune sortie.</td></tr>
    <?php endif; ?>
</table>

<div class="pagination">
    <?php for ($i = 1; $i <= $pages; $i++): ?>
        <?php if ($i === $page): ?>
            <span class="page-courante"><?= $i ?></span>
        <?php else: ?>
            <a href="?page=<?= $i ?>"><?= $i ?></a>
        <?php endif; ?>
    <?php endfor; ?>
</div>

<?php require_once __DIR__ . '/pied.php'; ?>
