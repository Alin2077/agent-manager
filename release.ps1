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
$newVersion = Read-Host "Enter new version (e.g. v0.2.0)"

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

# 5. Inject version into files
$plainVersion = $newVersion.TrimStart('v')
Write-Host ""
Write-Host "Setting version to $plainVersion in config files..." -ForegroundColor Cyan

# Update cli/Cargo.toml
$cliToml = Get-Content "cli/Cargo.toml" -Raw
$cliToml = $cliToml -replace '^version = ".*"', "version = `"$plainVersion`""
[System.IO.File]::WriteAllText("$PSScriptRoot/cli/Cargo.toml", $cliToml, [System.Text.UTF8Encoding]::new($false))

# Update gui/Cargo.toml
$guiToml = Get-Content "gui/Cargo.toml" -Raw
$guiToml = $guiToml -replace '^version = ".*"', "version = `"$plainVersion`""
[System.IO.File]::WriteAllText("$PSScriptRoot/gui/Cargo.toml", $guiToml, [System.Text.UTF8Encoding]::new($false))

# Update gui/tauri.conf.json
$tauriConf = Get-Content "gui/tauri.conf.json" -Raw | ConvertFrom-Json
$tauriConf.version = $plainVersion
$json = $tauriConf | ConvertTo-Json -Depth 10
[System.IO.File]::WriteAllText("$PSScriptRoot\gui\tauri.conf.json", $json, [System.Text.UTF8Encoding]::new($false))

Write-Host "  cli/Cargo.toml       -> $plainVersion"
Write-Host "  gui/Cargo.toml       -> $plainVersion"
Write-Host "  gui/tauri.conf.json  -> $plainVersion"

# 6. Commit version bump
git add cli/Cargo.toml gui/Cargo.toml gui/tauri.conf.json
git commit -m "chore: bump version to $newVersion" 2>$null

# 7. Tag and push
Write-Host ""
Write-Host "Creating tag: $newVersion ..." -ForegroundColor Cyan
git tag $newVersion

Write-Host "Pushing to GitHub ..." -ForegroundColor Cyan
git push origin master
git push origin $newVersion

Write-Host ""
Write-Host "Done! Release $newVersion pushed." -ForegroundColor Green
Write-Host ""
Write-Host "GitHub Actions is building:" -ForegroundColor Gray
Write-Host "  CLI binaries:    agent-manager-v${plainVersion}-*" -ForegroundColor Gray
Write-Host "  GUI installers:  Agent Manager_${plainVersion}_*" -ForegroundColor Gray
Write-Host ""
Write-Host "Track: https://github.com/Alin2077/agent-manager/actions" -ForegroundColor Gray
