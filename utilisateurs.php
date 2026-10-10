<?php
// ==============================================================================
// yAp y Ap — Gestion des utilisateurs (utilisateurs.php)
// ==============================================================================
// Page centrale de modération :
//   - liste PAGINÉE + recherche par pseudonyme/courriel ;
//   - actions : activer / désactiver (modération douce, réversible),
//     supprimer (irréversible, avec confirmation) ;
//   - chaque action vérifie le CSRF et est journalisée.
//
// Principe de modération : préférer la désactivation à la suppression.
// Une suppression casse l'historique (organisateur de sorties...), la
// désactivation conserve l'intégrité référentielle.
// ==============================================================================

declare(strict_types=1);
$TITRE = 'Utilisateurs';
require_once __DIR__ . '/entete.php';

$action    = $_POST['action'] ?? null;
$idCible   = (int) ($_POST['id'] ?? 0);
$parPage   = (int) ($CONFIG['affichage']['lignes_par_page']);
$page      = max(1, (int) ($_GET['page'] ?? 1));
$recherche = trim($_GET['q'] ?? '');

// --- Traitement des actions (POST) -------------------------------------------
if ($action !== null && $idCible > 0) {
    verifier_csrf();
    switch ($action) {
        case 'desactiver':
            db()->prepare('UPDATE utilisateurs SET actif = FALSE WHERE id = $1')
                ->execute([$idCible]);
            journaliser("utilisateur $idCible désactivé");
            break;
        case 'reactiver':
            db()->prepare('UPDATE utilisateurs SET actif = TRUE WHERE id = $1')
                ->execute([$idCible]);
            journaliser("utilisateur $idCible réactivé");
            break;
        case 'supprimer':
            // La suppression en cascade (ON DELETE CASCADE du schéma) emporte
            // sessions, sorties organisées et participations : c'est voulu,
            // mais c'est IRRÉVERSIBLE — d'où la confirmation côté client.
            db()->prepare('DELETE FROM utilisateurs WHERE id = $1')
                ->execute([$idCible]);
            journaliser("utilisateur $idCible SUPPRIMÉ");
            break;
    }
    // Redirection après action (modèle POST-redirect-GET) : un rafraîchissement
    // du navigateur ne rejoue PAS l'action accidentellement.
    header('Location: utilisateurs.php?page=' . $page . '&q=' . urlencode($recherche));
    exit;
}

// --- Construction du filtre de recherche -------------------------------------
// Requête préparée même pour la recherche : le joker est COLLÉ à la valeur
// ($1), jamais construit par concaténation de chaîne SQL.
$filtre = '';
$params = [];
if ($recherche !== '') {
    $filtre = "WHERE pseudonyme ILIKE $1 OR courriel ILIKE $1";
    $params[] = '%' . $recherche . '%';
}

// Comptage (pour la pagination) puis page courante.
$reqC = db()->prepare("SELECT count(*) FROM utilisateurs $filtre");
$reqC->execute($params);
$nb = (int) $reqC->fetchColumn();
$pages = max(1, (int) ceil($nb / $parPage));
$page  = min($page, $pages);

$req = db()->prepare(
    "SELECT id, pseudonyme, courriel, latitude, longitude, biographie, actif, cree_le
       FROM utilisateurs $filtre
       ORDER BY cree_le DESC
       LIMIT $2 OFFSET $3"
);
$req->execute(array_merge($params, [$parPage, ($page - 1) * $parPage]));
$utilisateurs = $req->fetchAll();
?>

<h2>Utilisateurs <span class="compteur"><?= $nb ?></span></h2>

<form method="get" class="recherche">
    <input type="text" name="q" value="<?= e($recherche) ?>"
           placeholder="Rechercher un pseudonyme ou un courriel…">
    <button type="submit">Rechercher</button>
</form>

<table>
    <tr>
        <th>Id</th><th>Pseudonyme</th><th>Courriel</th><th>Position</th>
        <th>Biographie</th><th>Statut</th><th>Inscrit le</th><th>Actions</th>
    </tr>
    <?php foreach ($utilisateurs as $u): ?>
    <tr class="<?= $u['actif'] ? '' : 'inactif' ?>">
        <td><?= (int) $u['id'] ?></td>
        <td><?= e($u['pseudonyme']) ?></td>
        <td><?= e($u['courriel']) ?></td>
        <td><?= e(sprintf('%.3f, %.3f', (float) $u['latitude'], (float) $u['longitude'])) ?></td>
        <td class="bio"><?= e(mb_strimwidth($u['biographie'] ?? '', 0, 40, '…')) ?></td>
        <td><?= $u['actif'] ? 'actif' : 'désactivé' ?></td>
        <td><?= e(substr((string) $u['cree_le'], 0, 10)) ?></td>
        <td class="actions">
            <?php if ($u['actif']): ?>
            <form method="post">
                <input type="hidden" name="csrf" value="<?= e(jeton_csrf()) ?>">
                <input type="hidden" name="id" value="<?= (int) $u['id'] ?>">
                <input type="hidden" name="action" value="desactiver">
                <button type="submit">Désactiver</button>
            </form>
            <?php else: ?>
            <form method="post">
                <input type="hidden" name="csrf" value="<?= e(jeton_csrf()) ?>">
                <input type="hidden" name="id" value="<?= (int) $u['id'] ?>">
                <input type="hidden" name="action" value="reactiver">
                <button type="submit">Réactiver</button>
            </form>
            <?php endif; ?>
            <form method="post"
                  onsubmit="return confirm('Supprimer définitivement cet utilisateur ? Toutes ses sorties et participations seront supprimées.');">
                <input type="hidden" name="csrf" value="<?= e(jeton_csrf()) ?>">
                <input type="hidden" name="id" value="<?= (int) $u['id'] ?>">
                <input type="hidden" name="action" value="supprimer">
                <button type="submit" class="dangereux">Supprimer</button>
            </form>
        </td>
    </tr>
    <?php endforeach; ?>
    <?php if (!$utilisateurs): ?>
    <tr><td colspan="8" class="vide">Aucun utilisateur trouvé.</td></tr>
    <?php endif; ?>
</table>

<div class="pagination">
    <?php for ($i = 1; $i <= $pages; $i++): ?>
        <?php if ($i === $page): ?>
            <span class="page-courante"><?= $i ?></span>
        <?php else: ?>
            <a href="?page=<?= $i ?>&amp;q=<?= e(urlencode($recherche)) ?>"><?= $i ?></a>
        <?php endif; ?>
    <?php endfor; ?>
</div>

<?php require_once __DIR__ . '/pied.php'; ?>
