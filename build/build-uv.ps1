#Requires -Version 7
<#
.SYNOPSIS
    构建 release 优化的 uv 二进制（体积更小、性能更好）。

.DESCRIPTION
    在标准 release 配置（strip + fat LTO + panic=abort，见根 Cargo.toml）基础上，
    通过 --config 追加 codegen-units = 1 以获得更好的优化效果。
    不修改仓库任何文件。

    注意：
    - 首次构建或参数变化时耗时较长（全量 LTO + 单编译单元）。
    - 与普通 `cargo build --release` 交替使用会因 profile 指纹不同而互相触发重编。

.EXAMPLE
    .\build-uv.ps1                          # 默认特性: performance, self-update
    .\build-uv.ps1 -Features performance    # 自定义特性
#>
param(
    [string[]]$Features = @('performance', 'self-update')
)

Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'

Set-Location -LiteralPath $PSScriptRoot

cargo build `
    --release `
    --locked `
    -p uv `
    --no-default-features `
    --features "$($Features -join ',')" `
    --config 'profile.release.codegen-units=1'

if ($LASTEXITCODE -eq 0) {
    $exe = Join-Path $PSScriptRoot 'target\release\uv.exe'
    $size = '{0:N1} MiB' -f ((Get-Item -LiteralPath $exe).Length / 1MB)
    Write-Host "`n构建成功: $exe ($size)" -ForegroundColor Green
} else {
    Write-Host "`n构建失败 (exit $LASTEXITCODE)" -ForegroundColor Red
    exit $LASTEXITCODE
}
