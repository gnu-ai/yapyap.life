# Référence de l'API REST

Base : `http://127.0.0.1:8080` (dev), `https://yapyap.life/api` (prod).
Tout en JSON. Erreurs : `{"erreur":"message"}` avec code adapté
(400, 401, 403, 404, 413, 500).

## Santé

| Route | Méthode | Auth | Réponse |
|---|---|---|---|
| /api/sante | GET | non | `{"statut":"ok","service":"yapyap"}` |

Sonde volontairement minimale : fonctionne même si PostgreSQL est lent.

## Authentification

| Route | Méthode | Auth | Corps / réponse |
|---|---|---|---|
| /api/auth/inscription | POST | non | `{pseudonyme, courriel, mot_de_passe (>=8), latitude, longitude, biographie}` → 201 |
| /api/auth/connexion | POST | non | `{courriel, mot_de_passe}` → `{jeton, expire_le}` |
| /api/auth/deconnexion | POST | oui | supprime la session |

Requêtes authentifiées : en-tête `Authorization: Bearer <jeton>`.

## Utilisateurs

| Route | Méthode | Auth | Description |
|---|---|---|---|
| /api/utilisateurs/moi | GET | oui | profil courant |
| /api/utilisateurs/moi | PATCH | oui | maj (latitude, longitude, biographie) |
| /api/utilisateurs/proches?latitude=..&longitude=..&rayon=.. | GET | oui | personnes à moins de rayon km, triées par distance |

## Sorties

| Route | Méthode | Auth | Description |
|---|---|---|---|
| /api/sorties | POST | oui | `{titre, description, latitude, longitude, debut (ISO 8601), capacite}` |
| /api/sorties?latitude=..&longitude=..&rayon=.. | GET | oui | sorties à proximité |
| /api/sorties/{id}/participer | POST | oui | rejoindre (limite capacite) |
| /api/sorties/{id}/retirer | POST | oui | se désinscrire |

## Exemple

```sh
curl -s -X POST http://127.0.0.1:8080/api/auth/inscription \
  -H 'Content-Type: application/json' \
  -d '{"pseudonyme":"claire","courriel":"claire@test.fr","mot_de_passe":"motdepasse123","latitude":48.8566,"longitude":2.3522,"biographie":"Salut !"}'
```

## Limites

- Corps limité par `http.max_body_bytes` (413 au-delà), cohérent avec
  `client_max_body_size` de Nginx.
- Jeton expire selon `sessions.duree_heures` (config) ; une fois expiré,
  seule une nouvelle connexion délivre un jeton.
