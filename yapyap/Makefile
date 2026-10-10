# ==============================================================================
# yAp y Ap — Makefile POSIX (GNU/Linux)
# ==============================================================================
# Le Makefile reste LE standard de construction pour un logiciel serveur
# GNU/Linux : cible unique, options simples, aucun prérequis au-delà de
# `make` et `cargo`. Syntaxe portable (pas d'extension GNU spécifique).
#
# Cibles principales :
#   make            -> build
#   make build      -> compilation en mode debug (rapide)
#   make release    -> compilation optimisée pour la production
#   make test       -> tests unitaires (cargo test)
#   make run        -> lance le serveur en avant-plan (mode dev)
#   make install    -> copie binaire + config vers /usr/local et /etc
#   make clean      -> supprime les produits de compilation
# ==============================================================================

# Nom du programme et emplacements standards GNU/Linux (hiérarchie FHS).
PROG    = yapyap
PREFIX  = /usr/local
BINDIR  = $(PREFIX)/bin
ETCDIR  = /etc/yapyap
CONF    = $(ETCDIR)/yapyap.toml

# PHONY : cibles qui ne sont pas des fichiers. Indispensable pour que make
# exécute toujours les cibles, même si un fichier homonyme existe.
.PHONY: all build release test run install uninstall clean

all: build

build:
	cargo build

release:
	cargo build --release

test:
	cargo test

run: build
	YAPYAP_CONFIG=$(CONF) cargo run

# Installation « manuelle » mais simple et transparente : chaque commande
# affichée par make peut être reproduite à la main par l'administrateur.
install: release
	install -D -m 755 target/release/$(PROG) $(BINDIR)/$(PROG)
	install -d -m 755 $(ETCDIR)
	if [ ! -f $(CONF) ]; then \
		install -m 640 config/yapyap.toml $(CONF); \
	fi

uninstall:
	rm -f $(BINDIR)/$(PROG)

clean:
	cargo clean
