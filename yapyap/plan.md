# yApyAp — Plan de développement

Application open-source combinant **vidéo en direct**, **animations 3D**,
**audio ultra-faible latence** et **monétisation**, autour de deux protocoles
éprouvés : **XMPP** (messagerie, présence, salons) et **Jamulus** (audio
collectif en temps réel).

## Vision produit

- Créer des lives à faible latence et interagir dans un environnement 3D.
- Discuter en direct ou en différé (serveur Jabber/XMPP), salons par
  centres d'intérêt, rencontres.
- Recevoir des **pièces** : cadeaux des autres membres, récompenses de live ;
  en gagner en tant que spectateur (messages en Morse), ou recharger via
  la centrale de paiement.
- Partager vidéos, créations musicales, enregistrements audio, podcasts ;
  diffuser depuis d'autres plateformes **dans le respect des droits
  d'auteur, avec rétribution SACEM**.

## Stack technique

| Brique | Technologie | Rôle |
|---|---|---|
| Backend | **Python** (FastAPI recommandé) | API métier : comptes, pièces, contenus, lives, droits |
| Messagerie / présence | **Prosody** (XMPP) | tchat public/privé, salons (MUC), notifications |
| Audio temps réel | **Jamulus** (serveur) | lives jusqu'à 9 personnes sans décalage audio |
| Frontend | **Godot Engine (C#)** | client 3D, interface live, tchat intégré |
| Base de données | **SQLite** (dev) / **PostgreSQL** (prod) | comptes, soldes, contenus, catalogues |
| Vidéo live | WebRTC ou SFU (à choisir en phase 4) | streaming vidéo faible latence |
| Paiement | centrale de paiement (à intégrer) | recharges de pièces |

## Principes d'architecture

1. **XMPP = colonne vertébrale temps réel.** Le tchat, la présence et les
   salons ne passent PAS par le backend Python : Prosody gère, le client
   Godot se connecte directement (bibliothèque XMPP). Le backend gère le
   métier (soldes, catalogues, droits) et publie des événements.
2. **Jamulus = audio uniquement.** Un serveur Jamulus par « scène audio »
   (jusqu'à 9 interprètes). Le frontal ne réinvente pas la latence basse.
3. **Pièces = table SQL + journal.** Toute transaction de pièces est
   journalisée (qui, quoi, quand, pourquoi) — même logique d'audit que
   l'interface d'administration yAp y Ap existante.
4. **Droits d'auteur dès la conception.** Toute diffusion musicale
   enregistrée (podcast, replay, fond sonore de live) passe par un
   module « droits » déclaratif avec déclaration SACEM — pas une
   fonctionnalité ajoutée après coup.

---

## Les phases

### Phase 0 — Cadrage et prototypes (2-3 semaines)

Objectif : lever les risques techniques AVANT d'écrire la plateforme.

- Définir le modèle de données cible (comptes, pièces, contenus, lives).
- Prototype 1 : client Godot ↔ Prosody (connexion XMPP, un message,
  un salon) — valider la bibliothèque XMPP en C#.
- Prototype 2 : client Godot ↔ serveur Jamulus (un flux audio aller-retour).
- Prototype 3 : pipeline vidéo (WebRTC/SFU vs relay RTMP) — mesure de
  latence réelle de bout en bout.
- Choix définitifs : SFU vidéo, prestataire de paiement, hébergement.

**Livrable : dossier de choix techniques + trois prototypes fonctionnels.**

### Phase 1 — Fondations (3-4 semaines)

Objectif : comptes et socle temps réel.

- Backend Python : comptes, authentification (jetons), profils.
- Prosody : installation, provisionnement automatique des comptes XMPP
  depuis le backend (mod_auth ou API admin), TLS.
- Client Godot : écran de connexion, fenêtre 3D de base (scène « hall »),
  connexion XMPP au login.
- Base de données : schéma initial (SQLite en dev).

**Livrable : un utilisateur se connecte et apparaît « en ligne » aux autres.**

### Phase 2 — Messagerie et salons (3 semaines)

Objectif : la vie sociale de la plateforme.

- Tchat 1-à-1 (XMPP), historique (MAM), état de présence, notifications.
- Salons publics par centres d'intérêt (MUC), découverte des salons.
- Liste d'amis, invitations.
- Interface Godot : fenêtres de tchat, vue salon, badges de présence.

**Livrable : discuter en direct et en différé, rejoindre des salons.**

### Phase 3 — Audio collectif Jamulus (3-4 semaines)

Objectif : jouer/parler à plusieurs sans décalage.

- Orchestration des serveurs Jamulus (un par scène audio, 9 sièges max).
- Réservation de scène depuis l'application (le backend démarre/arrête
  un serveur Jamulus dédié).
- Client Godot : connexion audio à une scène, VU-mètres, sièges,
  mixage, enregistrement possible de la session.
- Règles d'accès : public sur invitation, modération (couper un siège).

**Livrable : un groupe de 9 personnes joue ensemble en temps réel.**

### Phase 4 — Live vidéo et environnement 3D (5-6 semaines)

Objectif : le produit « live » visible.

- Streaming vidéo du présentateur (SFU choisi en phase 0), intégration
  dans la scène 3D (écran géant dans l'environnement).
- Environnement 3D : avatars, déplacement, zones (scène, public, salon).
- Interactions live : réactions 3D, entrées/sorties visibles (présence
  XMPP), tchat live superposé.
- Co-présence : jusqu'à N spectateurs, latence < 1,5 s pour la vidéo,
  audio Jamulus pour les participants actifs.

**Livrable : un live complet avec public 3D, tchat et interactions.**

### Phase 5 — Pièces et monétisation (4-5 semaines)

Objectif : l'économie de la plateforme.

- Portefeuille de pièces : soldes, journal des transactions (modèle
  append-only, jamais de mise à jour de solde sans écriture journalisée).
- Cadeaux : catalogue, envoi pendant les lives (animation 3D + entrée
  XMPP temps réel), partage créateur/plateforme paramétrable.
- Récompenses spectateur : pièces gagnées en envoyant des messages en
  **Morse** (mini-jeu encadré, plafonné, anti-triche), cadeaux reçus.
- Centrale de paiement : recharges (prestataire conforme DSP2), écriture
  du solde, reçus ; gestion des remboursements.
- Retraits créateurs (plus tard) : seuils, KYC légal.

**Livrable : économie fermée et journalisée : recharge, cadeau, gain, solde.**

### Phase 6 — Contenus : vidéo, audio, podcasts (4 semaines)

Objectif : le partage hors live.

- Upload et lecture de vidéos (transcodage, miniatures).
- Upload audio et enregistrements (sessions Jamulus exportées).
- Podcasts : flux, épisodes, lecture en arrière-plan dans le client.
- Partage social : commentaires (XMPP ou API), likes, abonnements.

**Livrable : publier et consommer tous types de contenus.**

### Phase 7 — Droits d'auteur et SACEM (3 semaines)

Objectif : diffuser de la musique en règle.

- Module déclaratif « œuvres diffusées » (titre, interprète, source).
- Détection des replay/contenus contenant de la musique protégée
  (empreinte audio — prestataire ou solution libre).
- Comptage des diffusions et **déclaration SACEM** (export au format
  attendu, rétribution juste), suivi des reversements.
- Blocage ou mise en sourdine automatique en live si œuvre non déclarée.

**Livrable : une diffusion musicale identifiée, déclarée et rétribuée.**

### Phase 8 — Sécurité, modération, mise en production (4 semaines)

Objectif : ouvrir au public.

- Modération : signalements, mute/ban (XMPP, lives, contenus), journal
  d'administration (réutiliser le modèle du back-office yAp y Ap).
- Sécurité : TLS partout, mots de passe Argon2, anti-fraude pièces
  (plafonds, détection d'anomalies), RGPD (export/suppression, mineurs).
- Charge : PostgreSQL, sauvegardes, supervision (sondes santé).
- Déploiement : conteneurs/orchestration, CDN vidéo, mise à l'échelle
  des serveurs Jamulus.

**Livrable : plateforme en production, modérée, conforme, sauvegardée.**

---

## Jalons synthétiques

| Jalon | Contenu | Semaine (cumul) |
|---|---|---|
| M1 | Prototypes validés (XMPP, Jamulus, vidéo) | ~3 |
| M2 | Comptes + présence temps réel | ~7 |
| M3 | Tchat et salons complets | ~10 |
| M4 | Scènes audio Jamulus réservables | ~14 |
| M5 | Live vidéo + monde 3D | ~20 |
| M6 | Économie de pièces + paiement | ~25 |
| M7 | Contenus et podcasts | ~29 |
| M8 | SACEM / droits | ~32 |
| M9 | Production | ~36 |

## Risques principaux et parades

| Risque | Parade |
|---|---|
| Latence vidéo > objectif | trancher en phase 0 entre SFU/P2P ; fallback audio-only |
| Bibliothèque XMPP immature en C#/Godot | prototype dès la phase 0 ; fallback : binaire/wrapper ou passerelle |
| Fraude sur les pièces | journal append-only, plafonds, plafond Morse anti-farm |
| Coûts serveurs (Jamulus, SFU) | scènes et lives démarrés à la demande, arrêt auto |
| SACEM / droits mal cadrés | module déclaratif dès la phase 7, pas de rétrofit tardif |
| Échelle des salons XMPP | MUC + archivage MAM, sharding de salons si besoin |
