#Requires -Version 7
<#
.SYNOPSIS
    将构建好的 uv 产物打包为 7z 分发包（Bandizip CLI）。

.DESCRIPTION
    汇集 target\release\uv.exe 与 uvx.exe，按内容总体积自动选择压缩级别：
      - 16 MB 以下        → 仅存储（-l:0）
      - 16 MB ~ 32 MB     → 正常压缩（-l:5）
      - 32 MB 以上        → 极限压缩（-l:9）
    输出到 dist\ 目录。需先运行 .\build-uv.ps1 完成构建。

.EXAMPLE
    .\build\pack-release.ps1 -Tag v0.12.5-unjx.2
#>
param(
    [Parameter(Mandatory)]
    [string]$Tag,                                   # 版本标签，如 v0.12.5-unjx.2

    [string]$BinDir = "",
    [string]$OutDir = ""
)

$ErrorActionPreference = 'Stop'

# 脚本位于 <仓库根>\build\ 下；默认产物/输出目录相对仓库根解析
$RepoRoot = (Resolve-Path -LiteralPath "$PSScriptRoot\..").Path
if (-not $BinDir) { $BinDir = Join-Path $RepoRoot 'target\release' }
if (-not $OutDir) { $OutDir = Join-Path $RepoRoot 'dist' }

# 定位 bz.exe（Bandizip CLI），优先 PATH，其次常见安装路径，找不到则报错
function Resolve-Bandizip {
    $cmd = Get-Command bz -ErrorAction SilentlyContinue
    if ($cmd) { return $cmd.Source }
    $candidates = @(
        "${env:ProgramFiles}\Bandizip\bz.exe",
        "${env:ProgramFiles(x86)}\Bandizip\bz.exe",
        "$env:LOCALAPPDATA\Bandizip\bz.exe"
    )
    foreach ($candidate in $candidates) {
        if (Test-Path $candidate) { return $candidate }
    }
    throw "未找到 bz.exe（Bandizip CLI）。请安装 Bandizip，或将 bz.exe 加入 PATH。"
}

# 校验待打包产物存在
$Files = @("uv.exe", "uvx.exe") | ForEach-Object { Join-Path $BinDir $_ }
foreach ($f in $Files) {
    if (-not (Test-Path $f)) {
        throw "缺少产物：$f 。请先运行 .\build-uv.ps1 完成构建。"
    }
}

$Bz = Resolve-Bandizip
$totalBytes = ($Files | Get-Item | Measure-Object Length -Sum).Sum
$mb = $totalBytes / 1MB

# 按总体积分档选择压缩级别：<16MB 存储；16~32MB 正常；>32MB 极限
$level = if ($totalBytes -lt 16MB) { 0 }
         elseif ($totalBytes -le 32MB) { 5 }
         else { 9 }
$levelName = @{ 0 = '仅存储'; 5 = '正常压缩'; 9 = '极限压缩' }[[int]$level]

New-Item -ItemType Directory -Path $OutDir -Force | Out-Null

$archive = Join-Path $OutDir "uv-$Tag-x86_64-pc-windows-msvc.7z"

Write-Host ">>> 打包 $Tag ..." -ForegroundColor Yellow
Write-Host "  内容: $(($Files | ForEach-Object { Split-Path $_ -Leaf }) -join ', ')" -ForegroundColor Gray
Write-Host ("  总体积: {0:N1} MiB → 压缩级别 {1}（{2}）" -f $mb, $level, $levelName) -ForegroundColor Gray

# 用 Bandizip CLI 打包为 7z：
#   c         = 创建压缩包
#   -fmt:7z   = 7z 格式（高压缩率；Windows 系统自带解压器不支持，需 Bandizip/7-Zip 解压）
#   -l:<N>    = 压缩级别（0=存储，5=正常，9=极限）
#   -y        = 覆盖已存在的同名压缩包
#   不加密、不分卷、不自解压（即不加 -p / -v / -sfx）
& $Bz c -y -fmt:7z "-l:$level" $archive @Files 2>&1 | Out-Null
if ($LASTEXITCODE -ne 0) {
    Write-Host "ERROR: bz.exe 打包失败" -ForegroundColor Red
    exit 1
}

# 显式校验产物存在并汇报体积对比
if (-not (Test-Path $archive)) {
    Write-Host "ERROR: 未找到打包产物: $archive" -ForegroundColor Red
    exit 1
}

$archiveMiB = (Get-Item $archive).Length / 1MB
Write-Host ""
Write-Host "打包完成:" -ForegroundColor Green
Write-Host ("  {0}  ({1:N1} MiB，原始 {2:N1} MiB)" -f $archive, $archiveMiB, $mb)
