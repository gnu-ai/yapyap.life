#!/bin/sh
# ==============================================================================
# yAp y Ap — Script d'init POSIX (deploy/init/yapyap.sh)
# ==============================================================================
# Script de service conforme aux conventions SysV/LSB, utilisable sur toute
# distribution GNU/Linux (Debian avec /etc/init.d, mais aussi scripts d'init
# génériques, conteneurs, etc.). Actions : start | stop | restart | status.
#
# Installation :
#   install -m 755 deploy/init/yapyap.sh /etc/init.d/yapyap
#   update-rc.d yapyap defaults        # activation au démarrage (Debian)
# ==============================================================================

# --- Chemins et variables -----------------------------------------------------
PROG="/usr/local/bin/yapyap"
PIDFILE="/var/run/yapyap.pid"
CONFIG="/etc/yapyap/yapyap.toml"
LOGDIR="/var/log/yapyap"

# Utilisateur NON privilégié : principe du moindre privilège — une faille de
# l'application ne donne pas root à l'attaquant.
USER="yapyap"
GROUP="yapyap"

# Démarrage silencieux si le binaire est absent : le script est destiné à
# être appelé par le système, qui journalise l'échec.
[ -x "$PROG" ] || exit 5

# --- Fonction de démarrage ----------------------------------------------------
start() {
    if [ -f "$PIDFILE" ] && kill -0 "$(cat "$PIDFILE")" 2>/dev/null; then
        echo "yapyap : déjà en cours d'exécution"
        return 0
    fi
    echo "yapyap : démarrage..."
    mkdir -p "$LOGDIR" && chown "$USER:$GROUP" "$LOGDIR"
    # start-stop-daemon (outil standard des distributions) détache le
    # processus, change d'utilisateur et écrit le fichier PID.
    start-stop-daemon --start --background --make-pidfile --pidfile "$PIDFILE" \
        --chuid "$USER:$GROUP" \
        --startas "$PROG" \
        --chdir / -- \
        || { echo "yapyap : échec du démarrage"; return 1; }
    echo "yapyap : démarré (pid $(cat "$PIDFILE" 2>/dev/null))"
}

# --- Fonction d'arrêt ---------------------------------------------------------
stop() {
    if [ ! -f "$PIDFILE" ]; then
        echo "yapyap : non démarré"
        return 0
    fi
    PID=$(cat "$PIDFILE")
    echo "yapyap : arrêt (SIGTERM vers $PID)..."
    # SIGTERM : le programme s'arrête proprement (les connexions en cours
    # se terminent ; tokio remonte l'ordre d'arrêt).
    kill "$PID" 2>/dev/null
    # Attente bornée de la fin effective du processus (10 s max).
    i=0
    while kill -0 "$PID" 2>/dev/null && [ "$i" -lt 10 ]; do
        sleep 1
        i=$((i + 1))
    done
    kill -9 "$PID" 2>/dev/null      # dernier recours, après temporisation
    rm -f "$PIDFILE"
    echo "yapyap : arrêté"
}

# --- Aiguillage des actions (syntaxe POSIX) -----------------------------------
case "$1" in
    start)   start ;;
    stop)    stop ;;
    restart) stop; start ;;
    status)
        if [ -f "$PIDFILE" ] && kill -0 "$(cat "$PIDFILE")" 2>/dev/null; then
            echo "yapyap : en cours d'exécution"
            exit 0
        else
            echo "yapyap : arrêté"
            exit 3
        fi
        ;;
    *)
        echo "Usage : $0 {start|stop|restart|status}"
        exit 2
        ;;
esac
exit 0
