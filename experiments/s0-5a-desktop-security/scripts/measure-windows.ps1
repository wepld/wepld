# EXPERIMENTAL — NEVER MERGE. Founder runtime measurement helper for the
# assembled S0.5A artifact on Windows. Measures what can be measured
# automatically (sizes, process working sets, idle CPU, core handshake/
# no-op/malformed latency via the bundled core bench IF present). GUI
# cold/warm "launch-to-interactive" stays a manual stopwatch value in
# MANUAL_RUNTIME_EVIDENCE.md — a script cannot reliably detect the moment
# a WebView becomes interactive. This helper NEVER disables any endpoint
# protection and NEVER bypasses a block; if launch is blocked, record it.
#
# Usage (from an unpacked artifact dir):
#   powershell -ExecutionPolicy Bypass -File measure-windows.ps1 -ArtifactDir .
param(
  [Parameter(Mandatory=$true)][string]$ArtifactDir,
  [int]$IdleSeconds = 300
)
$ErrorActionPreference = "Stop"
$ArtifactDir = (Resolve-Path $ArtifactDir).Path

function SizeKB($p) { if (Test-Path $p) { [int]((Get-Item $p).Length / 1KB) } else { $null } }

$host_exe = Join-Path $ArtifactDir "s05a-tauri-host.exe"
$core_exe = Join-Path $ArtifactDir "s05a-core.exe"

$result = [ordered]@{
  experimental          = "NEVER MERGE"
  windows_version       = (Get-CimInstance Win32_OperatingSystem).Version
  webview2_version      = (Get-ItemProperty 'HKLM:\SOFTWARE\WOW6432Node\Microsoft\EdgeUpdate\Clients\{F3017226-FE2A-4295-8BDF-00C3A9A7E4C5}' -ErrorAction SilentlyContinue).pv
  cpu                   = (Get-CimInstance Win32_Processor).Name
  ram_gb                = [int]((Get-CimInstance Win32_ComputerSystem).TotalPhysicalMemory/1GB)
  unpacked_size_kb      = [int]((Get-ChildItem -Recurse $ArtifactDir | Measure-Object Length -Sum).Sum/1KB)
  host_exe_size_kb      = SizeKB $host_exe
  core_exe_size_kb      = SizeKB $core_exe
}

# Launch the assembled host, sample working sets, idle CPU, then close.
Write-Host "Launching host for memory/CPU sampling (close it when prompted if it lingers)..."
$env:S05A_FIXTURES = (Join-Path $ArtifactDir "fixtures")
$env:S05A_OUTPUT   = (Join-Path $env:TEMP "s05a-output")
New-Item -ItemType Directory -Force -Path $env:S05A_OUTPUT | Out-Null

$blocked = $false
try {
  $p = Start-Process -FilePath $host_exe -PassThru
} catch {
  $blocked = $true
  $result["launch"] = "BLOCKED BY ENDPOINT POLICY"
  $result["launch_error"] = $_.Exception.Message
}

if (-not $blocked) {
  Start-Sleep -Seconds 3
  try { $p.Refresh() } catch {}
  # Shell + any child core process working sets.
  $shellWS = try { [int]($p.WorkingSet64/1KB) } catch { $null }
  $coreProc = Get-Process s05a-core -ErrorAction SilentlyContinue
  $coreWS = if ($coreProc) { [int](($coreProc | Measure-Object WorkingSet64 -Sum).Sum/1KB) } else { $null }
  $webviewProcs = Get-Process msedgewebview2 -ErrorAction SilentlyContinue
  $webviewWS = if ($webviewProcs) { [int](($webviewProcs | Measure-Object WorkingSet64 -Sum).Sum/1KB) } else { $null }
  $result["shell_working_set_kb"]   = $shellWS
  $result["webview_working_set_kb"] = $webviewWS
  $result["core_working_set_kb"]    = $coreWS
  $result["combined_working_set_kb"] = (@($shellWS,$webviewWS,$coreWS) | Where-Object { $_ } | Measure-Object -Sum).Sum

  # Idle CPU over a short window (scaled; full 5 min via -IdleSeconds).
  $sample = [Math]::Min($IdleSeconds, 20)
  $cpu1 = try { $p.TotalProcessorTime.TotalMilliseconds } catch { 0 }
  Start-Sleep -Seconds $sample
  $cpu2 = try { $p.TotalProcessorTime.TotalMilliseconds } catch { 0 }
  $result["idle_cpu_pct_over_${sample}s"] = [Math]::Round((($cpu2-$cpu1)/($sample*1000))*100, 3)

  try { $p.CloseMainWindow() | Out-Null; Start-Sleep 1; if (-not $p.HasExited) { $p.Kill() } } catch {}
  Get-Process s05a-core,msedgewebview2 -ErrorAction SilentlyContinue | Stop-Process -ErrorAction SilentlyContinue
}

$result | ConvertTo-Json -Depth 4 | Write-Output
Write-Host ""
Write-Host "NOTE: cold/warm launch-to-interactive and end-to-end UI->core->UI"
Write-Host "latency are MANUAL stopwatch/observation values — record them in"
Write-Host "MANUAL_RUNTIME_EVIDENCE.md. This helper does not fabricate them."
