# Mazerion GitHub Upload Script
# Save as: upload-to-github.ps1

Write-Host "=== Mazerion GitHub Upload ===" -ForegroundColor Cyan

# Check if we're in the right directory
if (-not (Test-Path "Cargo.toml")) {
    Write-Host "ERROR: Not in Mazerion project directory!" -ForegroundColor Red
    Write-Host "Please run this script from: C:\Users\DrTom\RustroverProjects\Mazerion" -ForegroundColor Yellow
    exit 1
}

# Check if git is installed
try {
    git --version | Out-Null
} catch {
    Write-Host "ERROR: Git is not installed!" -ForegroundColor Red
    Write-Host "Install from: https://git-scm.com/download/win" -ForegroundColor Yellow
    exit 1
}

# Initialize git if needed
if (-not (Test-Path ".git")) {
    Write-Host "`nInitializing Git repository..." -ForegroundColor Yellow
    git init
    Write-Host "✓ Git initialized" -ForegroundColor Green
} else {
    Write-Host "`n✓ Git repository exists" -ForegroundColor Green
}

# Create .gitignore if it doesn't exist
if (-not (Test-Path ".gitignore")) {
    Write-Host "`nCreating .gitignore..." -ForegroundColor Yellow
    @"
/target
Cargo.lock
**/*.rs.bk
*.pdb
.DS_Store
*.swp
*.log
.idea/
.vscode/
*.db
"@ | Out-File -FilePath ".gitignore" -Encoding UTF8
    Write-Host "✓ .gitignore created" -ForegroundColor Green
}

# Check if we have a remote
$hasRemote = git remote | Select-String "origin"

if (-not $hasRemote) {
    Write-Host "`nNo GitHub remote found!" -ForegroundColor Yellow
    Write-Host "Enter your GitHub repository URL:" -ForegroundColor Cyan
    Write-Host "Example: https://github.com/YourUsername/Mazerion.git" -ForegroundColor Gray
    $repoUrl = Read-Host "Repository URL"

    if ($repoUrl) {
        git remote add origin $repoUrl
        Write-Host "✓ Remote 'origin' added: $repoUrl" -ForegroundColor Green
    } else {
        Write-Host "ERROR: No repository URL provided!" -ForegroundColor Red
        exit 1
    }
} else {
    $currentRemote = git remote get-url origin
    Write-Host "`n✓ Remote exists: $currentRemote" -ForegroundColor Green
}

# Stage all files
Write-Host "`nStaging files..." -ForegroundColor Yellow
git add .
Write-Host "✓ Files staged" -ForegroundColor Green

# Show status
Write-Host "`nCurrent status:" -ForegroundColor Cyan
git status --short

# Get commit message
Write-Host "`nEnter commit message (or press Enter for default):" -ForegroundColor Cyan
$commitMsg = Read-Host "Message"
if (-not $commitMsg) {
    $commitMsg = "Update Mazerion - $(Get-Date -Format 'yyyy-MM-dd HH:mm')"
}

# Commit
Write-Host "`nCommitting changes..." -ForegroundColor Yellow
git commit -m "$commitMsg"
Write-Host "✓ Changes committed" -ForegroundColor Green

# Check current branch
$currentBranch = git branch --show-current
if (-not $currentBranch) {
    $currentBranch = "main"
    git branch -M main
    Write-Host "✓ Set default branch to 'main'" -ForegroundColor Green
}

# Push to GitHub
Write-Host "`nPushing to GitHub (branch: $currentBranch)..." -ForegroundColor Yellow
try {
    git push -u origin $currentBranch
    Write-Host "`n✓ Successfully pushed to GitHub!" -ForegroundColor Green
    Write-Host "`nYour repository: $(git remote get-url origin)" -ForegroundColor Cyan
} catch {
    Write-Host "`nFirst push - setting upstream..." -ForegroundColor Yellow
    git push --set-upstream origin $currentBranch
    Write-Host "`n✓ Successfully pushed to GitHub!" -ForegroundColor Green
}

Write-Host "`n=== Upload Complete ===" -ForegroundColor Cyan
Write-Host "Your Mazerion project is now on GitHub!" -ForegroundColor Green