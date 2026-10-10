<?php
// ==============================================================================
// yAp y Ap — En-tête HTML de l'administration (entete.php)
// ==============================================================================
// Partial d'en-tête : chaque page affiche le même bandeau (titre, navigation,
// identité de l'administrateur connecté). Un seul endroit pour la charte.
//
// $TITRE : variable fournie par la page appelante avant l'inclusion.
// ==============================================================================
require_once __DIR__ . '/fonctions.php';
exiger_admin();          // une page incluant l'en-tête est nécessairement protégée

// Expiration automatique après inactivité : vérifiée à CHAQUE page, pas
// seulement à la connexion. Une session administrative oubliée ouverte
// sur un poste partagé est un risque classique de back-office.
if (!empty($_SESSION['derniere_activite'])
    && (time() - (int) $_SESSION['derniere_activite']) > $CONFIG['session']['duree_secondes']) {
    header('Location: deconnexion.php');
    exit;
}
$_SESSION['derniere_activite'] = time();
?>
<!DOCTYPE html>
<html lang="fr">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1">
    <title><?= e($TITRE ?? 'Administration') ?> — yAp y Ap</title>
    <link rel="stylesheet" href="style.css">
</head>
<body>
<header>
    <h1>yAp y Ap <small>administration</small></h1>
    <nav>
        <a href="index.php">Tableau de bord</a>
        <a href="utilisateurs.php">Utilisateurs</a>
        <a href="sorties.php">Sorties</a>
        <a href="sessions.php">Sessions</a>
        <a href="journal.php">Journal</a>
    </nav>
    <div class="identite">
        <?= e($_SESSION['admin_courriel'] ?? '') ?>
        · <a href="deconnexion.php">Déconnexion</a>
    </div>
</header>
<main>
