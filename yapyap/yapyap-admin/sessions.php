<?php
// ==============================================================================
// yAp y Ap — Sessions actives (sessions.php)
// ==============================================================================
// Liste des jetons d'authentification encore valides. Actions de sécurité :
//   - révoquer une session suspecte (déconnexion forcée d'un appareil) ;
//   - purger TOUTES les sessions (déconnexion générale, utile en cas
//     d'incident : force chaque appareil à se reconnecter).
// ==============================================================================

declare(strict_types=1);
$TITRE = 'Sessions';
require_once __DIR__ . '/entete.php';

$action = $_POST['action'] ?? null;

if ($action !== null) {
    verifier_csrf();
    if ($action === 'revoquer') {
        $jeton = $_POST['jeton'] ?? '';
        db()->prepare('DELETE FROM sessions WHERE jeton = $1')->execute([$jeton]);
        journaliser('session révoquée : ' . substr($jeton, 0, 8) . '…');
    } elseif ($action === 'purger') {
        // Suppression de TOUTES les sessions : chaque appareil devra
        // se reconnecter. (La session ADMIN du back-office est séparée,
        // elle n'est pas affectée — cf. migration 002.)
        $n = db()->exec('DELETE FROM sessions');
        journaliser("purge générale des sessions ($n jetons)");
    }
    header('Location: sessions.php');
    exit;
}

// Liste : jetons tronqués à l'affichage (un back-office n'affiche jamais un
// secret complet — le préfixe suffit à identifier la ligne).
$sessions = db()->query(
    "SELECT s.jeton, s.cree_le, s.expire_le, u.pseudonyme
       FROM sessions s
       JOIN utilisateurs u ON u.id = s.utilisateur
      WHERE s.expire_le > now()
      ORDER BY s.cree_le DESC
      LIMIT 200"
)->fetchAll();
?>

<h2>Sessions actives <span class="compteur"><?= count($sessions) ?></span></h2>

<form method="post"
      onsubmit="return confirm('Déconnecter TOUS les appareils de tous les utilisateurs ?');">
    <input type="hidden" name="csrf" value="<?= e(jeton_csrf()) ?>">
    <input type="hidden" name="action" value="purger">
    <button type="submit" class="dangereux">Purger toutes les sessions</button>
</form>

<table>
    <tr><th>Utilisateur</th><th>Jeton (préfixe)</th><th>Ouverte le</th><th>Expire le</th><th>Action</th></tr>
    <?php foreach ($sessions as $s): ?>
    <tr>
        <td><?= e($s['pseudonyme']) ?></td>
        <td class="mono"><?= e(substr($s['jeton'], 0, 8)) ?>…</td>
        <td><?= e(substr((string) $s['cree_le'], 0, 16)) ?></td>
        <td><?= e(substr((string) $s['expire_le'], 0, 16)) ?></td>
        <td class="actions">
            <form method="post">
                <input type="hidden" name="csrf" value="<?= e(jeton_csrf()) ?>">
                <input type="hidden" name="jeton" value="<?= e($s['jeton']) ?>">
                <input type="hidden" name="action" value="revoquer">
                <button type="submit">Révoquer</button>
            </form>
        </td>
    </tr>
    <?php endforeach; ?>
    <?php if (!$sessions): ?>
    <tr><td colspan="5" class="vide">Aucune session active.</td></tr>
    <?php endif; ?>
</table>

<?php require_once __DIR__ . '/pied.php'; ?>
