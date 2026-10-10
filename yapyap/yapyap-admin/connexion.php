<?php
// ==============================================================================
// yAp y Ap — Connexion administrateur (connexion.php)
// ==============================================================================
// Formulaire de connexion du back-office. Points de sécurité :
//   - requête PRÉPARÉE (aucune concaténation SQL) ;
//   - vérification par password_verify() contre l'empreinte Argon2id ;
//   - message d'échec GÉNÉRIQUE (ne pas révéler si le courriel existe) ;
//   - régénération d'identifiant de session après connexion (anti-fixation).
// ==============================================================================

declare(strict_types=1);
require_once __DIR__ . '/fonctions.php';

// La session (nom de cookie, paramètres, démarrage) est entièrement gérée
// par fonctions.php — cette page n'a plus qu'à vérifier l'expiration.
$duree = $CONFIG['session']['duree_secondes'];

$erreur = null;

// --- Traitement du formulaire ------------------------------------------------
if ($_SERVER['REQUEST_METHOD'] === 'POST') {
    verifier_csrf();
    $courriel = trim($_POST['courriel'] ?? '');
    $passe    = $_POST['mot_de_passe'] ?? '';

    $req = db()->prepare('SELECT id, mot_de_passe_hash FROM administrateurs WHERE courriel = $1');
    $req->execute([$courriel]);
    $admin = $req->fetch();

    // Condition unique : compte existant ET mot de passe correct.
    if ($admin && password_verify($passe, $admin['mot_de_passe_hash'])) {
        session_regenerate_id(true);          // anti-fixation de session
        $_SESSION['admin_id']      = (int) $admin['id'];
        $_SESSION['admin_courriel'] = $courriel;
        $_SESSION['derniere_activite'] = time();
        header('Location: index.php');
        exit;
    }
    // Message volontairement identique pour « compte inexistant » et
    // « mauvais mot de passe » : aucune énumération possible.
    $erreur = 'Identifiants incorrects.';
}

// ---Expiration automatique après inactivité ----------------------------------
if (!empty($_SESSION['derniere_activite']) && (time() - $_SESSION['derniere_activite']) > $duree) {
    session_unset();
    session_destroy();
    $erreur = 'Session expirée, veuillez vous reconnecter.';
} elseif (!empty($_SESSION['admin_id'])) {
    $_SESSION['derniere_activite'] = time();
}
?>
<!DOCTYPE html>
<html lang="fr">
<head>
    <meta charset="UTF-8">
    <title>Connexion — administration yAp y Ap</title>
    <link rel="stylesheet" href="style.css">
</head>
<body class="connexion">
<div class="boite-connexion">
    <h1>yAp y Ap <small>administration</small></h1>
    <?php if ($erreur): ?>
        <p class="erreur"><?= e($erreur) ?></p>
    <?php endif; ?>
    <form method="post">
        <input type="hidden" name="csrf" value="<?= e(jeton_csrf()) ?>">
        <label for="courriel">Courriel</label>
        <input type="email" id="courriel" name="courriel" required autofocus>
        <label for="mot_de_passe">Mot de passe</label>
        <input type="password" id="mot_de_passe" name="mot_de_passe" required>
        <button type="submit">Se connecter</button>
    </form>
</div>
</body>
</html>
