# 🔄 Système de Redémarrage - Guide de Démarrage Rapide

## 🚀 Démarrage Immédiat

```powershell
# Démarrage complet avec tests
.\quick-start-with-restart.ps1 -Mode full

# Démarrage développement uniquement
.\quick-start-with-restart.ps1 -Mode dev

# Démarrage statique (production)
.\quick-start-with-restart.ps1 -Mode static
```

## 🎯 Fonctionnalités Principales

### ✅ **Redémarrage Automatique**
- Détection automatique des erreurs
- Récupération sans intervention utilisateur
- Sauvegarde automatique de l'état

### ✅ **Redémarrage Manuel**
- Interface utilisateur intuitive
- 4 types de redémarrage disponibles
- Contrôle total sur le processus

### ✅ **Monitoring de Santé**
- Surveillance en temps réel
- Détection proactive des problèmes
- Métriques de performance

### ✅ **Sauvegarde d'État**
- Préservation de la session utilisateur
- Restauration automatique après redémarrage
- Nettoyage intelligent des anciens états

## 🔧 Interface Utilisateur

### Bouton de Redémarrage
Situé en bas à droite de l'interface, le bouton de redémarrage offre :
- Accès rapide aux options de redémarrage
- Indicateur visuel des erreurs détectées
- Historique des redémarrages récents

### Types de Redémarrage

| Type | Description | Utilisation |
|------|-------------|-------------|
| **Soft** | Réinitialise l'interface | Problèmes UI mineurs |
| **Hard** | Recharge la page complète | Erreurs JavaScript |
| **Services** | Redémarre les services backend | Problèmes API |
| **Full** | Redémarrage complet | Problèmes systémiques |

## 🛠️ Scripts Disponibles

### Scripts Principaux

```powershell
# Démarrage avec redémarrage intégré
.\quick-start-with-restart.ps1 -Mode dev -Port 3001

# Test du système complet
.\test-restart-system.ps1 -Verbose

# Redémarrage des services
.\restart-services.ps1 -Services @("all") -WaitForHealthy

# Interface stable (sans hot reload)
.\start-ui-stable.ps1 -Dev
```

### Options Avancées

```powershell
# Redémarrage forcé
.\restart-services.ps1 -Force

# Test sans compilation
.\test-restart-system.ps1 -SkipBuild

# Démarrage sans tests
.\quick-start-with-restart.ps1 -SkipTests
```

## 📊 Monitoring et Diagnostic

### Page de Santé
Accédez à `http://localhost:3001/#/health` pour :
- État en temps réel des services
- Métriques de performance frontend
- Historique des vérifications de santé

### Diagnostic des Erreurs
Le système capture automatiquement :
- Erreurs JavaScript non gérées
- Promesses rejetées
- Composants React en erreur
- Problèmes de connectivité réseau

### Logs et Historique
Tous les événements sont enregistrés avec :
- Horodatage précis
- Raison du redémarrage
- Durée de l'opération
- Statut de réussite/échec

## 🔍 Dépannage Rapide

### Problèmes Courants

**Interface qui ne répond plus :**
```powershell
.\restart-services.ps1 -Services @("web-ui") -Force
```

**Services backend inaccessibles :**
```powershell
.\restart-services.ps1 -Services @("consciousness-engine", "api-gateway")
```

**Erreurs de compilation :**
```powershell
cd web-ui
npm install
npm run build
```

**Ports occupés :**
```powershell
# Vérifier les ports utilisés
Get-NetTCPConnection -LocalPort 3000,3001,8080,8081

# Forcer l'arrêt
.\restart-services.ps1 -Force
```

### Commandes de Diagnostic

```powershell
# État des processus
Get-Process | Where-Object {$_.ProcessName -like "*node*" -or $_.ProcessName -like "*cargo*"}

# Test de connectivité
Test-NetConnection -ComputerName localhost -Port 3001

# Nettoyage complet
Remove-Item web-ui/node_modules -Recurse -Force
Remove-Item web-ui/build -Recurse -Force
```

## ⚙️ Configuration

### Variables d'Environnement

Le système utilise automatiquement ces variables pour la stabilité :

```bash
FAST_REFRESH=false
WDS_HOT=false
WDS_LIVE_RELOAD=false
CHOKIDAR_USEPOLLING=true
WATCHPACK_POLLING=true
BROWSER=none
```

### Personnalisation

Modifiez `web-ui/src/types/restart.ts` pour ajuster :
- Nombre maximum de tentatives
- Délai entre les tentatives
- Intervalle de vérification de santé
- Comportement de sauvegarde d'état

## 🧪 Tests et Validation

### Test Automatique Complet

```powershell
.\test-restart-system.ps1 -Verbose
```

**Ce test vérifie :**
- ✅ Présence de tous les composants
- ✅ Validité des types TypeScript
- ✅ Compilation sans erreur
- ✅ Syntaxe des scripts PowerShell
- ✅ Démarrage et accessibilité du serveur

### Tests Manuels Recommandés

1. **Test de Récupération d'Erreur :**
   - Ouvrir la console développeur
   - Exécuter : `throw new Error("Test error")`
   - Vérifier la capture et récupération

2. **Test de Redémarrage Manuel :**
   - Cliquer sur le bouton de redémarrage
   - Tester chaque type de redémarrage
   - Vérifier la préservation de l'état

3. **Test de Monitoring :**
   - Accéder à `/health`
   - Vérifier les métriques affichées
   - Tester la détection de problèmes

## 📈 Métriques de Performance

### Objectifs de Performance
- **Temps de redémarrage soft :** < 2 secondes
- **Temps de redémarrage hard :** < 10 secondes
- **Détection d'erreur :** < 1 seconde
- **Sauvegarde d'état :** < 500ms

### Indicateurs de Santé
- **Taux de succès :** > 95%
- **Fréquence d'erreurs :** < 1 par heure
- **Temps de récupération :** < 30 secondes

## 🔒 Sécurité et Bonnes Pratiques

### Mesures de Sécurité
- Validation des entrées utilisateur
- Limitation du nombre de tentatives
- Nettoyage automatique des données sensibles
- Logs sécurisés sans exposition de tokens

### Recommandations
- Surveiller les redémarrages fréquents
- Analyser les patterns d'erreurs
- Maintenir les dépendances à jour
- Tester régulièrement le système

## 📞 Support et Ressources

### Documentation Complète
- `RESTART_SYSTEM_DOCUMENTATION.md` - Documentation technique détaillée
- `WEB_UI_FIXES_README.md` - Solutions aux problèmes de rafraîchissement
- `INFINITE_REFRESH_SOLUTION.md` - Guide de résolution des boucles

### Scripts Utiles
- `test-ui-stability.ps1` - Test de stabilité de l'interface
- `serve-static-ui.ps1` - Serveur statique simple
- `build-static-ui.ps1` - Construction optimisée

### Contact
Pour obtenir de l'aide ou signaler des problèmes :
1. Consulter les logs dans la console du navigateur
2. Exécuter `.\test-restart-system.ps1 -Verbose`
3. Vérifier les fichiers de configuration `.env.local`
4. Contacter l'équipe de développement

---

**🎉 Le système de redémarrage est maintenant opérationnel !**

*Utilisez `.\quick-start-with-restart.ps1 -Mode dev` pour commencer immédiatement.*
