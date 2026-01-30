#!/bin/bash

echo "========================================"
echo "Pushing SkillMaster to GitHub"
echo "========================================"
echo ""

# Check if git is initialized
if [ ! -d .git ]; then
    echo "Initializing Git repository..."
    git init
    echo ""
fi

# Add remote if not exists
if ! git remote get-url origin > /dev/null 2>&1; then
    echo "Adding remote origin..."
    git remote add origin https://github.com/LeeSeaside/skillmaster.git
    echo ""
else
    echo "Remote origin already exists, updating URL..."
    git remote set-url origin https://github.com/LeeSeaside/skillmaster.git
    echo ""
fi

# Add all files
echo "Adding files to git..."
git add .
echo ""

# Commit
echo "Committing changes..."
git commit -m "Initial commit: SkillMaster v0.1.0"
echo ""

# Set main branch
echo "Setting main branch..."
git branch -M main
echo ""

# Push to GitHub
echo "Pushing to GitHub..."
git push -u origin main
echo ""

echo "========================================"
echo "Done! Check your repository at:"
echo "https://github.com/LeeSeaside/skillmaster"
echo "========================================"
