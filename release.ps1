# PowerShell release script
# Usage: .\release.ps1

$ErrorActionPreference = "Stop"
Set-Location $PSScriptRoot

# 1. Get last tag
$lastTag = git tag --sort=-version:refname | Select-Object -First 1
if ($lastTag) {
    Write-Host "Last release: " -NoNewline
    Write-Host $lastTag -ForegroundColor Cyan
} else {
    Write-Host "No previous release found" -ForegroundColor Yellow
}

# 2. Prompt for new version
Write-Host ""
$newVersion = Read-Host "Enter new version (e.g. v0.1.0)"

if (-not $newVersion) {
    Write-Host "Error: version cannot be empty" -ForegroundColor Red
    exit 1
}

if (-not $newVersion.StartsWith("v")) {
    $newVersion = "v$newVersion"
}

# 3. Confirm
Write-Host ""
Write-Host "About to release: " -NoNewline
Write-Host $newVersion -ForegroundColor Green
$confirm = Read-Host "Confirm? (y/n)"

if ($confirm -ne "y" -and $confirm -ne "Y") {
    Write-Host "Cancelled" -ForegroundColor Yellow
    exit 0
}

# 4. Handle existing tag (local + remote)
$tagExists = git tag -l $newVersion
if ($tagExists) {
    Write-Host ""
    Write-Host "Tag $newVersion already exists, removing..." -ForegroundColor Yellow
    git tag -d $newVersion
    git push origin :refs/tags/$newVersion 2>$null
    Write-Host "Old tag removed." -ForegroundColor Gray
}

# 5. Check working tree is clean
$status = git status --porcelain
if ($status) {
    Write-Host "Warning: uncommitted changes:" -ForegroundColor Yellow
    Write-Host $status
    $force = Read-Host "Continue anyway? (y/n)"
    if ($force -ne "y" -and $force -ne "Y") {
        Write-Host "Cancelled" -ForegroundColor Yellow
        exit 0
    }
}

# 6. Tag and push
Write-Host ""
Write-Host "Creating tag: $newVersion ..." -ForegroundColor Cyan
git tag $newVersion

Write-Host "Pushing to GitHub ..." -ForegroundColor Cyan
git push origin $newVersion

Write-Host ""
Write-Host "Done! Release $newVersion pushed." -ForegroundColor Green
Write-Host "GitHub Actions is building the binaries..." -ForegroundColor Gray
Write-Host "Track progress: https://github.com/Alin2077/agent-manager/actions" -ForegroundColor Gray
