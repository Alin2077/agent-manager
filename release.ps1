# PowerShell 发布脚本
# 用法: .\release.ps1

$ErrorActionPreference = "Stop"
Set-Location $PSScriptRoot

# 1. 获取上一个 tag
$lastTag = git describe --tags --abbrev=0 2>$null
if ($lastTag) {
    Write-Host "📌 上次发布版本: " -NoNewline
    Write-Host $lastTag -ForegroundColor Cyan
} else {
    Write-Host "📌 还没有发布过任何版本" -ForegroundColor Yellow
}

# 2. 提示输入新版本号
Write-Host ""
$newVersion = Read-Host "🚀 请输入本次发布的版本号 (如 v0.1.0)"

if (-not $newVersion) {
    Write-Host "❌ 版本号不能为空" -ForegroundColor Red
    exit 1
}

# 确保以 v 开头
if (-not $newVersion.StartsWith("v")) {
    $newVersion = "v$newVersion"
}

# 3. 确认
Write-Host ""
Write-Host "将要发布: " -NoNewline
Write-Host $newVersion -ForegroundColor Green
$confirm = Read-Host "确认? (y/n)"

if ($confirm -ne "y" -and $confirm -ne "Y") {
    Write-Host "已取消" -ForegroundColor Yellow
    exit 0
}

# 4. 确保工作区干净
$status = git status --porcelain
if ($status) {
    Write-Host "⚠️  工作区有未提交的更改：" -ForegroundColor Yellow
    Write-Host $status
    $force = Read-Host "是否仍然继续? (y/n)"
    if ($force -ne "y" -and $force -ne "Y") {
        Write-Host "已取消" -ForegroundColor Yellow
        exit 0
    }
}

# 5. 打 tag 并推送
Write-Host ""
Write-Host "🏷️  正在创建 tag: $newVersion ..." -ForegroundColor Cyan
git tag $newVersion

Write-Host "📤 正在推送到 GitHub ..." -ForegroundColor Cyan
git push origin $newVersion

Write-Host ""
Write-Host "✅ 发布成功！" -ForegroundColor Green
Write-Host "   GitHub Actions 正在构建安装包..." -ForegroundColor Gray
Write-Host "   查看进度: https://github.com/Alin2077/agent-manager/actions" -ForegroundColor Gray
