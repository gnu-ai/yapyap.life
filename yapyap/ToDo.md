# yApyAp — ToDo fonctionnelle

À cocher au fil du développement. Chaque bloc suit une phase du plan.md.
Les éléments `🔬` sont des points à trancher/prototyper d'abord.

---

## Phase 0 — Cadrage et prototypes

- [ ] Modèle de données cible (comptes, pièces, contenus, lives, droits)
- [ ] 🔬 Prototype Godot ↔ Prosody : connexion, message, salon (bibliothèque XMPP C#)
- [ ] 🔬 Prototype Godot ↔ Jamulus : un flux audio aller-retour
- [ ] 🔬 Prototype pipeline vidéo : mesure latence WebRTC/SFU vs RTMP
- [ ] 🔬 Choix du SFU vidéo (LiveKit / Janus / mediasoup…)
- [ ] 🔬 Choix prestataire de paiement (DSP2, frais, webhooks)
- [ ] 🔬 Stratégie hébergement (VPS dédiés Jamulus, région, bande passante)
- [ ] Dossier de choix techniques rédigé

## Phase 1 — Fondations (backend + comptes + présence)

- [ ] Squelette backend Python (FastAPI) : santé, config, logs
- [ ] Base de données : SQLite dev / PostgreSQL prod, migrations
- [ ] Comptes : inscription, connexion (Argon2), jetons de session
- [ ] Profils : pseudonyme, avatar, description
- [ ] Prosody : installation, TLS, domaine XMPP
- [ ] Provisionnement des comptes XMPP depuis le backend
- [ ] Client Godot : écran de connexion, session persistante
- [ ] Client Godot : scène 3D « hall » de base + avatar minimal
- [ ] Connexion XMPP automatique au login, affichage « en ligne »
- [ ] 🔬 Anti-spam à l'inscription (email, débit)

## Phase 2 — Messagerie et salons

- [ ] Tchat 1-à-1 en temps réel (XMPP)
- [ ] Messages différés + historique (MAM)
- [ ] Présence : en ligne / absent / ne pas déranger
- [ ] Notifications de message (badge, son)
- [ ] Salons publics (MUC) par centres d'intérêt
- [ ] Recherche/découverte des salons
- [ ] Liste d'amis + invitations
- [ ] UI Godot : fenêtres de tchat, vue salon, badges de présence
- [ ] 🔬 Modération des salons : rôles MUC (owner/moderator/mute)

## Phase 3 — Audio collectif (Jamulus)

- [ ] Orchestration : démarrer/arrêter un serveur Jamulus à la demande
- [ ] Créer une « scène audio » (sièges, accès public/privé)
- [ ] Connexion audio depuis Godot à une scène
- [ ] VU-mètres, volume individuel, mute des sièges
- [ ] Jusqu'à 9 interprètes simultanés sans décalage
- [ ] Enregistrement de session (export audio)
- [ ] Latence mesurée et affichée, indicateur de qualité
- [ ] 🔬 Anti-larsen/conseils de configuration client (ASIO/JACK)
- [ ] Arrêt automatique des scènes inactives (coûts)

## Phase 4 — Live vidéo + monde 3D

- [ ] Publication du flux vidéo du présentateur (SFU)
- [ ] Affichage du live dans la scène 3D (écran géant)
- [ ] Avatars : personnalisation, déplacement, animations
- [ ] Zones 3D : scène, public, salons
- [ ] Réactions visuelles 3D pendant le live
- [ ] Tchat live superposé (lien XMPP ↔ salon du live)
- [ ] Entrées/sorties du public visibles (présence)
- [ ] Audio public du live (mix retour des interprètes Jamulus)
- [ ] 🔬 Objectif latence vidéo < 1,5 s — mesuré de bout en bout
- [ ] Spectateurs : objectif de charge (100 / 500 / 1000)
- [ ] Rediffusion possible du live (enregistrement serveur)

## Phase 5 — Pièces et monétisation

- [ ] Portefeuille : soldes, opérations atomiques
- [ ] Journal des transactions append-only (qui/quoi/quand/pourquoi)
- [ ] Catalogue de cadeaux (objets 3D, valeurs en pièces)
- [ ] Envoi de cadeaux pendant un live (animation 3D + événement XMPP)
- [ ] Attribution des cadeaux au créateur (partage paramétrable)
- [ ] Mini-jeu **Morse** : envoyer des messages en Morse contre pièces
- [ ] Anti-triche Morse : plafonds journaliers, débit max, captchas doux
- [ ] Pièces pour temps de live (règles de gain affichées)
- [ ] Centrale de paiement : recharges (webhooks, idempotence)
- [ ] Reçus et historique de recharge
- [ ] Gestion des erreurs de paiement / remboursements
- [ ] 🔬 Conformité monétique (DSP2) + cadre légal (monnaie virtuelle)
- [ ] Retraits créateurs : seuils, KYC (plus tard, cadrer tôt)

## Phase 6 — Contenus (vidéo, audio, podcasts)

- [ ] Upload vidéo (limites, formats) + transcodage + miniatures
- [ ] Lecteur vidéo intégré au client
- [ ] Upload audio et enregistrements de sessions
- [ ] Podcasts : création, épisodes, artwork
- [ ] Flux/abonnements aux podcasts
- [ ] Lecture audio en arrière-plan dans le client
- [ ] Partage depuis d'autres plateformes (liens/embed)
- [ ] Commentaires et likes sur contenus
- [ ] Abonnements aux créateurs
- [ ] Modération des contenus (signalement, retrait)

## Phase 7 — Droits d'auteur et SACEM

- [ ] Module déclaratif des œuvres diffusées (titre, interprète, source)
- [ ] Déclaration lors de l'upload d'un contenu musical
- [ ] Déclaration des musiques jouées en live (déclaration manuelle du présentateur)
- [ ] Détection par empreinte audio sur les replay/contenus
- [ ] Mise en sourdine automatique si œuvre non déclarée en live
- [ ] Comptage des diffusions par œuvre
- [ ] Export de déclaration **SACEM** (format attendu)
- [ ] Calcul et suivi de la rétribution / reversements
- [ ] 🔬 Rappel : licence/protection des créateurs (consentement, CID)
- [ ] 🔬 Contact SACEM : cadre de répartition pour plateforme numérique

## Phase 8 — Sécurité, modération, production

- [ ] Modération : signalements (messages, lives, contenus)
- [ ] Sanctions : mute/ban XMPP, exclusion de live, retrait de contenu
- [ ] Journal d'administration auditable (modèle yAp y Ap existant)
- [ ] Sécurité : TLS partout, Argon2, plafonds de débit API
- [ ] Anti-fraude pièces : détection d'anomalies, alertes
- [ ] RGPD : export/suppression des données, consentements
- [ ] Protection des mineurs (âge, contenus filtrés)
- [ ] PostgreSQL en production + sauvegardes testées
- [ ] Supervision : sondes santé (backend, Prosody, Jamulus, SFU)
- [ ] Déploiement conteneurisé + orchestration
- [ ] Mise à l'échelle des serveurs Jamulus et du SFU
- [ ] CDN vidéo
- [ ] Documentation utilisateur + charte de communauté
- [ ] Beta fermée → beta publique → lancement

---

## Backlog après lancement

- [ ] Retraits créateurs complets (KYC, calendrier de versement)
- [ ] Stories/éphémères
- [ ] Duos et invités distants en vidéo
- [ ] Traductions du client
- [ ] Application mobile (portage Godot)
- [ ] Marché de créateurs (vente de contenus contre pièces)
- [ ] Statistiques créateurs (audience, revenus)
