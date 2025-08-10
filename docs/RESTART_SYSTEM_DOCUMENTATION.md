# 🔄 Système de Redémarrage - Documentation Complète

## 📋 Vue d'Ensemble

Le système de redémarrage de la Plateforme d'Agents IA Ultime est une solution complète pour gérer les redémarrages automatiques et manuels de l'application, incluant la récupération d'erreurs, la sauvegarde d'état, et le monitoring de santé.

## 🏗️ Architecture

### Composants Principaux

1. **RestartManager** - Interface utilisateur pour les redémarrages manuels
2. **ErrorBoundary** - Capture des erreurs et récupération automatique
3. **HealthMonitor** - Surveillance de la santé des services
4. **AppStateManager** - Sauvegarde et restauration d'état
5. **useAppRecovery** - Hook de récupération automatique

### Types de Redémarrage

- **Soft** : Réinitialise l'interface sans recharger la page
- **Hard** : Recharge complètement la page
- **Services** : Redémarre les services backend
- **Full** : Redémarrage complet du système

## 🚀 Utilisation

### Interface Utilisateur

Le gestionnaire de redémarrage est accessible via un bouton flottant en bas à droite de l'interface :

```tsx
// Intégré automatiquement dans App.tsx
<RestartManager />
```

### Redémarrage Programmatique

```typescript
import { useAppRecovery } from '../hooks/useAppRecovery';

const { triggerRecovery } = useAppRecovery();

// Déclencher une récupération manuelle
triggerRecovery('Raison du redémarrage');
```

### Scripts PowerShell

```powershell
# Redémarrer tous les services
.\restart-services.ps1

# Redémarrer des services spécifiques
.\restart-services.ps1 -Services @("consciousness-engine", "api-gateway")

# Redémarrage avec attente de santé
.\restart-services.ps1 -WaitForHealthy

# Test du système
.\test-restart-system.ps1 -Verbose
```

## ⚙️ Configuration

### Configuration par Défaut

```typescript
const DEFAULT_RESTART_CONFIG: RestartConfig = {
  autoRestart: true,
  maxRetries: 3,
  retryDelay: 5000,
  healthCheckInterval: 30000,
  saveStateOnRestart: true,
  notifyUser: true,
};
```

### Personnalisation

```typescript
const { triggerRecovery } = useAppRecovery({
  config: {
    maxRetries: 5,
    retryDelay: 3000,
    autoRestart: false,
  },
  onRestart: (event) => {
    console.log('Redémarrage:', event);
  },
});
```

## 🔍 Monitoring et Diagnostic

### Surveillance de Santé

Le composant `HealthMonitor` vérifie automatiquement :
- État des services backend
- Performance du frontend
- Erreurs JavaScript
- Renders excessifs

### Métriques Collectées

- Nombre de renders par composant
- Erreurs JavaScript récentes
- Statut des services (running/error/stopped)
- Temps de réponse des APIs

### Logs et Historique

Tous les redémarrages sont enregistrés avec :
- ID unique de l'événement
- Raison du redémarrage
- Durée de l'opération
- Succès/échec
- État sauvegardé/restauré

## 🛠️ Développement

### Ajout d'un Nouveau Service

1. Modifier `restart-services.ps1` :

```powershell
$ServiceConfig = @{
    "mon-service" = @{
        Name = "Mon Service"
        Path = "mon-service"
        Port = 8082
        HealthEndpoint = "/health"
        StartCommand = "cargo run --release"
        StopSignal = "SIGTERM"
    }
}
```

2. Ajouter la vérification de santé dans `HealthMonitor.tsx`

### Personnalisation de l'Error Boundary

```tsx
<ErrorBoundary
  autoRecover={true}
  maxRetries={5}
  recoveryDelay={3000}
  onError={(error, errorInfo) => {
    // Logique personnalisée
  }}
>
  <MonComposant />
</ErrorBoundary>
```

## 🧪 Tests

### Test Automatique

```powershell
# Test complet du système
.\test-restart-system.ps1

# Test avec détails
.\test-restart-system.ps1 -Verbose

# Test interactif
.\test-restart-system.ps1 -Interactive
```

### Tests Manuels

1. **Test de Récupération d'Erreur** :
   - Déclencher une erreur JavaScript
   - Vérifier la capture par ErrorBoundary
   - Confirmer la récupération automatique

2. **Test de Redémarrage Manuel** :
   - Ouvrir le gestionnaire de redémarrage
   - Tester chaque type de redémarrage
   - Vérifier la sauvegarde/restauration d'état

3. **Test de Monitoring** :
   - Accéder à `/health`
   - Vérifier les métriques affichées
   - Tester la détection de problèmes

## 🔧 Dépannage

### Problèmes Courants

1. **Redémarrage en Boucle** :
   - Vérifier les logs d'erreur
   - Réduire `maxRetries`
   - Désactiver `autoRestart` temporairement

2. **État Non Restauré** :
   - Vérifier le localStorage
   - Contrôler les permissions
   - Nettoyer les anciens états

3. **Services Non Redémarrés** :
   - Vérifier les ports utilisés
   - Contrôler les permissions d'exécution
   - Examiner les logs PowerShell

### Commandes de Diagnostic

```powershell
# Vérifier les processus actifs
Get-NetTCPConnection -LocalPort 3000,8080,8081

# Nettoyer les processus bloqués
.\restart-services.ps1 -Force

# Test de connectivité
Test-NetConnection -ComputerName localhost -Port 3000
```

## 📊 Métriques de Performance

### Indicateurs Clés

- **MTTR** (Mean Time To Recovery) : < 30 secondes
- **Taux de Succès** : > 95%
- **Détection d'Erreur** : < 5 secondes
- **Sauvegarde d'État** : < 1 seconde

### Monitoring Continu

Le système collecte automatiquement :
- Temps de redémarrage
- Fréquence des erreurs
- Utilisation des ressources
- Satisfaction utilisateur

## 🔒 Sécurité

### Mesures Implémentées

- Validation des entrées utilisateur
- Limitation du nombre de tentatives
- Logs sécurisés (pas de données sensibles)
- Nettoyage automatique des anciens états

### Bonnes Pratiques

- Ne jamais exposer les tokens dans les logs
- Chiffrer les données sensibles sauvegardées
- Limiter les permissions des scripts
- Auditer les redémarrages fréquents

## 🚀 Roadmap

### Version Actuelle (1.0)
- ✅ Redémarrage manuel et automatique
- ✅ Sauvegarde/restauration d'état
- ✅ Monitoring de santé
- ✅ Error Boundary intégré

### Version Future (2.0)
- 🔄 Redémarrage distribué
- 🔄 Prédiction de pannes
- 🔄 Métriques avancées
- 🔄 Interface d'administration

## 📞 Support

Pour obtenir de l'aide :

1. Consulter les logs : Console du navigateur + Terminal
2. Exécuter les tests : `.\test-restart-system.ps1 -Verbose`
3. Vérifier la configuration : Fichiers `.env.local`
4. Contacter l'équipe de développement

---

*Documentation générée automatiquement - Version 1.0*
