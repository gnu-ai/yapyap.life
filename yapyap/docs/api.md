# Référence de l'API REST yAp y Ap

Toutes les routes renvoient du JSON. Les routes marquées 🔒 exigent
l'en-tête `Authorization: Bearer <jeton>` obtenu à la connexion.

## Santé

| Méthode | Chemin        | Réponse                     |
|---------|---------------|-----------------------------|
| GET     | /api/sante    | `{"statut":"ok"}`           |

## Authentification

| Méthode | Chemin                  | Corps                              | Réponse |
|---------|-------------------------|------------------------------------|---------|
| POST    | /api/auth/inscription   | `{pseudonyme, courriel, mot_de_passe, latitude, longitude, biographie?}` | 201 + profil |
| POST    | /api/auth/connexion     | `{courriel, mot_de_passe}`         | 200 + `{jeton, expire_le}` |
| POST    | /api/auth/deconnexion   | — (jeton requis)                   | 204 |

## Utilisateurs

| Méthode | Chemin                        | Description |
|---------|-------------------------------|-------------|
| GET 🔒   | /api/utilisateurs/moi         | profil courant |
| PATCH 🔒 | /api/utilisateurs/moi         | mise à jour `{latitude?, longitude?, biographie?}` |
| GET 🔒   | /api/utilisateurs/proches?latitude=..&longitude=..&rayon=.. | personnes à moins de `rayon` km |

Réponse de « proches » :
```json
{ "rayon_km": 5.0,
  "personnes": [ {"id":1,"pseudonyme":"claire","distance_km":1.7, ...} ] }
```

## Sorties

| Méthode | Chemin                                | Description |
|---------|---------------------------------------|-------------|
| POST 🔒  | /api/sorties                         | créer `{titre, description?, latitude, longitude, debut, capacite?}` |
| GET 🔒   | /api/sorties?latitude=..&longitude=.. | sorties à venir à proximité |
| POST 🔒  | /api/sorties/{id}/participer         | inscription (409 si pleine) |
| POST 🔒  | /api/sorties/{id}/retirer            | désinscription |

## Codes d'erreur

Corps uniforme : `{"erreur":"message"}`.

| Statut | Signification |
|--------|---------------|
| 400    | données invalides |
| 401    | jeton absent/expiré |
| 404    | ressource introuvable |
| 409    | conflit (pseudonyme pris, sortie pleine) |
| 500    | erreur interne (détails uniquement en journal serveur) |
