<?php
// ==============================================================================
// yAp y Ap — Déconnexion administrateur (deconnexion.php)
// ==============================================================================
// Destruction complète de la session : données vidées, cookie effacé,
// session détruite côté serveur. Aucun état résiduel.
// ==============================================================================
require_once __DIR__ . '/fonctions.php';

session_unset();
if (ini_get('session.use_cookies')) {
    $p = session_get_cookie_params();
    setcookie(session_name(), '', time() - 42000, $p['path'], $p['domain'] ?? '');
}
session_destroy();

header('Location: connexion.php');
exit;
