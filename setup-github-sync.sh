#!/bin/bash

# Vérifier si le dépôt distant GitHub existe déjà
if ! git remote | grep -q "github"; then
    echo "Configuration du dépôt distant GitHub..."
    read -p "Entrez l'URL de votre dépôt GitHub (ex: git@github.com:utilisateur/repo.git): " github_url
    git remote add github $github_url
fi

# Ajouter ce script comme hook post-push pour Gitea
HOOK_PATH=".git/hooks/post-push"

if [ ! -f "$HOOK_PATH" ]; then
    echo "Installation du hook post-push..."
    cat > "$HOOK_PATH" << 'EOF'
#!/bin/bash
echo "Synchronisation avec GitHub..."
git push github $(git rev-parse --abbrev-ref HEAD)
EOF
    chmod +x "$HOOK_PATH"
    echo "Hook post-push installé avec succès!"
fi

echo "Configuration terminée! Vos pushes seront maintenant automatiquement synchronisés avec GitHub."