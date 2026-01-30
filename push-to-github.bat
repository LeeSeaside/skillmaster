@echo off
echo ========================================
echo Pushing SkillMaster to GitHub
echo ========================================
echo.

REM Check if git is initialized
if not exist .git (
    echo Initializing Git repository...
    git init
    echo.
)

REM Add remote if not exists
git remote get-url origin >nul 2>&1
if errorlevel 1 (
    echo Adding remote origin...
    git remote add origin https://github.com/LeeSeaside/skillmaster.git
    echo.
) else (
    echo Remote origin already exists, updating URL...
    git remote set-url origin https://github.com/LeeSeaside/skillmaster.git
    echo.
)

REM Add all files
echo Adding files to git...
git add .
echo.

REM Commit
echo Committing changes...
git commit -m "Initial commit: SkillMaster v0.1.0"
echo.

REM Set main branch
echo Setting main branch...
git branch -M main
echo.

REM Push to GitHub
echo Pushing to GitHub...
git push -u origin main
echo.

echo ========================================
echo Done! Check your repository at:
echo https://github.com/LeeSeaside/skillmaster
echo ========================================
pause
