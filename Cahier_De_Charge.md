# Cahier des Charges - Clone de Maze Wars en Rust/Bevy

## 1. Vue d'ensemble du projet

### 1.1 Objectif
Développer une version moderne du jeu Maze Wars en utilisant Rust et le moteur de jeu Bevy, en implémentant une architecture client-serveur multijoueur.

### 1.2 Technologies principales
- Langage : Rust
- Moteur de jeu : Bevy
- Protocol réseau : UDP
- Architecture : Client-serveur

## 2. Spécifications techniques

### 2.1 Architecture système
- **Serveur central**
  - Gestion des connexions clients (minimum 10 joueurs simultanés)
  - Synchronisation de l'état du jeu
  - Gestion des collisions et de la logique de jeu
  - Système de rooms/sessions de jeu

- **Client**
  - Interface graphique avec Bevy
  - Gestion des inputs joueur
  - Rendu 3D temps réel
  - Interface utilisateur
  - Communication réseau avec le serveur

### 2.2 Composants Bevy essentiels
- **Systems**
  - NetworkingSystem
  - InputSystem
  - PhysicsSystem
  - RenderSystem
  - UISystem
  - GameStateSystem

- **Resources**
  - GameState
  - NetworkState
  - PlayerState
  - MapState

- **Components**
  - Player
  - Wall
  - Projectile
  - Camera
  - Transform
  - Collider

### 2.3 Interface utilisateur
- **Écran de connexion**
  - Champ pour l'adresse IP du serveur
  - Champ pour le nom d'utilisateur
  - Bouton de connexion

- **Interface en jeu**
  - Minimap avec positions des joueurs
  - Compteur FPS
  - Score
  - Vie restante
  - Liste des joueurs connectés

### 2.4 Niveaux de jeu
- 3 niveaux minimum avec difficulté croissante
- Système de génération de labyrinthe
- Système de chargement de niveau

## 3. Contraintes techniques

### 3.1 Performance
- Maintenir 50+ FPS constant
- Latence réseau maximale acceptable : 100ms
- Optimisation des assets et des calculs physiques

### 3.2 Réseau
- Implémentation UDP fiable
- Gestion de la désynchronisation
- Prédiction côté client
- Réconciliation côté serveur

### 3.3 Code
- Tests unitaires pour chaque module
- Documentation complète (rustdoc)
- Gestion des erreurs robuste
- Logging complet

## 4. Planification et structure du projet

### 4.1 Structure des dossiers
```
src/
├── bin/
│   ├── client.rs
│   └── server.rs
├── common/
│   ├── network/
│   ├── game_state/
│   └── types/
├── client/
│   ├── graphics/
│   ├── input/
│   └── ui/
└── server/
    ├── game_logic/
    ├── physics/
    └── session/
```

### 4.2 Dépendances principales
```toml
[dependencies]
bevy = "0.12"
bevy_networking = "0.12"
serde = { version = "1.0", features = ["derive"] }
tokio = { version = "1.0", features = ["full"] }
rand = "0.8"
rapier3d = "0.17"  # Pour la physique
```

## 5. Fonctionnalités bonus

### 5.1 Éditeur de niveau
- Interface graphique pour la création de niveaux
- Sauvegarde/chargement de niveaux personnalisés
- Outils d'édition intuitifs
- Validation de niveau

### 5.2 Génération procédurale
- Algorithme de génération de labyrinthe
- Paramètres configurables
- Validation de jouabilité
- Seeds pour reproduction

### 5.3 IA des adversaires
- Pathfinding intelligent (A*)
- Différents niveaux de difficulté
- Comportements variés
- Adaptation au style de jeu du joueur

### 5.4 Interface de connexion améliorée
- Historique des serveurs
- Système d'alias pour les serveurs
- Interface graphique pour la configuration
- Sauvegarde des préférences

## 6. Bonnes pratiques à respecter

### 6.1 Code
- Utilisation de clippy avec configuration stricte
- Format de code rustfmt
- Documentation exhaustive
- Tests de couverture >80%
- Revue de code systématique

### 6.2 Architecture
- SOLID principles
- Clean Architecture
- Error handling robuste
- Logging complet
- Modularité et réutilisabilité

### 6.3 Performance
- Profilage régulier
- Optimisation des allocations
- Minimisation des copies
- Cache-friendly data structures
- Parallel computing quand possible

### 6.4 Sécurité
- Validation des entrées
- Sanitization des données réseau
- Protection contre les attaques DoS
- Rate limiting
- Gestion sécurisée des sessions

<!-- _______________________________________________________________________________ -->

### **7. Phases du projet**

#### **7.1 Phase 1 : Préparation (1 semaine)**
- Étudier la bibliothèque **Bevy** (tutoriels et documentation).
- Définir la structure du projet :
  - Séparer les modules pour le client, le serveur et les systèmes communs.
- Créer des wireframes pour l’interface utilisateur.

#### **7.2 Phase 2 : Développement (4 semaines)**
1. **Semaine 1 :**
   - Implémentation de la logique de connexion client-serveur avec UDP.
   - Création de l’environnement de jeu de base (mur, joueur, déplacements).
2. **Semaine 2 :**
   - Ajout des fonctionnalités de la mini-map.
   - Affichage des performances (FPS, ping).
   - Gestion des collisions dans le labyrinthe.
3. **Semaine 3 :**
   - Développement des niveaux (création manuelle et difficulté croissante).
   - Ajout de la configuration client (adresse IP, pseudo).
4. **Semaine 4 :**
   - Amélioration des graphismes et de l’interface.
   - Tests de charge pour garantir la stabilité du serveur.

#### **7.3 Phase 3 : Bonus et Finalisation (2 semaines)**
- Implémenter un ou plusieurs bonus :
  - Génération procédurale de labyrinthe.
  - IA pour les joueurs.
  - Éditeur de niveaux.
- Documentation du code et des fonctionnalités.

#### **7.4 Phase 4 : Validation et déploiement (1 semaine)**
- Test de bout en bout avec plusieurs joueurs.
- Optimisation des performances.
- Déploiement sur une machine ou un serveur public pour démonstration.

---