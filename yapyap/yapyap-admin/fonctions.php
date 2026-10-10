<?php
// ==============================================================================
// yAp y Ap — Fonctions communes de l'administration (fonctions.php)
// ==============================================================================
// Toutes les pages « incluent » ce fichier : il fournit
//   1. la connexion PDO à PostgreSQL (une seule, réutilisée) ;
//   2. le contrôle d'accès (exiger_admin) ;
//   3. la protection CSRF (jeton unique par session) ;
//   4. l'échappement HTML (e) — jamais de echo brut d'une donnée utilisateur ;
//   5. la journalisation des actions d'administration.
// ==============================================================================

declare(strict_types=1);

// ------------------------------------------------------------------------------
// Charger la configuration depuis /etc/yapyap/admin.php (hors racine web).
// Le chemin est absolu, indépendant du répertoire courant — convention POSIX.
// ------------------------------------------------------------------------------
$CHEMIN_CONFIG = getenv('YAPYAP_ADMIN_CONFIG') ?: '/etc/yapyap/admin.php';
if (!is_readable($CHEMIN_CONFIG)) {
    // Échec grave : on refuse de deviner une configuration par défaut qui
    // pourrait pointer vers une mauvaise base. Arrêt net et message clair.
    http_response_code(500);
    exit('Configuration introuvable : ' . htmlspecialchars($CHEMIN_CONFIG));
}
$CONFIG = require $CHEMIN_CONFIG;

// ------------------------------------------------------------------------------
// Démarrage de la session PHP UNIQUEMENT si ce n'est déjà fait, et AVANT tout
// affichage (en-têtes HTTP). Le nom et les paramètres du cookie (httponly :
// inaccessible en JavaScript, donc immunisé contre le vol par XSS ; samesite
// Lax : mitige le CSRF) sont définis ICI, une seule fois, pour tout le
// back-office — les pages n'ont plus à s'en occuper.
// ------------------------------------------------------------------------------
if (session_status() === PHP_SESSION_NONE) {
    session_name($CONFIG['session']['nom']);
    session_set_cookie_params(['httponly' => true, 'samesite' => 'Lax']);
    session_start();
}

// ------------------------------------------------------------------------------
// Connexion PDO à PostgreSQL.
// PDO est l'interface d'accès aux SGBD normalisée de PHP ; le pilote pgsql
// parle le protocole PostgreSQL — même protocole que notre application Rust.
// ATTR_ERRMODE = EXCEPTION : toute erreur SQL lève une exception au lieu
// d'être silencieusement ignorée (les erreurs muettes sont la plaque
// tournante des bugs de back-office).
// ATTR_DEFAULT_FETCH_MODE = ASSOC : lignes lues comme tableaux associatifs.
// ------------------------------------------------------------------------------
function db(): PDO
{
    static $pdo = null;              // connexion unique, réutilisée (memoization)
    if ($pdo === null) {
        global $CONFIG;
        $dsn = sprintf(
            'pgsql:host=%s;port=%d;dbname=%s',
            $CONFIG['db']['hote'],
            $CONFIG['db']['port'],
            $CONFIG['db']['base']
        );
        $pdo = new PDO($dsn, $CONFIG['db']['user'], $CONFIG['db']['passe'], [
            PDO::ATTR_ERRMODE            => PDO::ERRMODE_EXCEPTION,
            PDO::ATTR_DEFAULT_FETCH_MODE => PDO::FETCH_ASSOC,
        ]);
    }
    return $pdo;
}

// ------------------------------------------------------------------------------
// Contrôle d'accès : à appeler EN TÊTE de chaque page protégée.
// Si l'administrateur n'est pas connecté → redirection vers le formulaire.
// ------------------------------------------------------------------------------
function exiger_admin(): void
{
    if (empty($_SESSION['admin_id'])) {
        header('Location: connexion.php');
        exit;
    }
}

// ------------------------------------------------------------------------------
// Échappement HTML systématique. L'interface affichant des données saisies
// par les UTILISATEURS (pseudonymes, biographies...), tout affichage sans
// échappement serait une faille XSS ouverte.
// ------------------------------------------------------------------------------
function e(?string $texte): string
{
    return htmlspecialchars($texte ?? '', ENT_QUOTES, 'UTF-8');
}

// ------------------------------------------------------------------------------
// Protection CSRF : un formulaire légitime DOIT porter le jeton de session.
// Sans ce contrôle, une page malveillante visitée par l'administrateur
// pourrait soumettre des actions à son insu (désactivations en masse...).
// ------------------------------------------------------------------------------
function jeton_csrf(): string
{
    if (empty($_SESSION['csrf'])) {
        $_SESSION['csrf'] = bin2hex(random_bytes(32)); // 256 bits aléatoires
    }
    return $_SESSION['csrf'];
}

function verifier_csrf(): void
{
    $recu = $_POST['csrf'] ?? '';
    if (!hash_equals(jeton_csrf(), $recu)) {   // comparaison à temps constant
        http_response_code(403);
        exit('Jeton CSRF invalide — formulaire refusé.');
    }
}

// ------------------------------------------------------------------------------
// Journalisation d'une action d'administration (table journal_admin).
// Chaque action de modération laisse une trace : qui, quoi, quand.
// ------------------------------------------------------------------------------
function journaliser(string $action): void
{
    $req = db()->prepare(
        'INSERT INTO journal_admin (admin_id, action) VALUES ($1, $2)'
    );
    // pg_query_params style: PDO + pgsql utilise $1, $2...
    $req->execute([$_SESSION['admin_id'] ?? 0, $action]);
}
