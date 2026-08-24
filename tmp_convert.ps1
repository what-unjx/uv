param([string[]]$Files)
foreach ($f in $Files) {
  $text = [System.IO.File]::ReadAllText($f)
  $sb = New-Object System.Text.StringBuilder
  $i = 0
  while ($i -lt $text.Length) {
    $idx = $text.IndexOf("PythonInstallation::find_or_download(", $i)
    $idxBest = if ($idx -ge 0) { $idx } else { $text.IndexOf("PythonInstallation::find_best(", $i) }
    if ($idx -ge 0 -and ($idxBest -lt 0 -or $idx -le $idxBest)) { $name = "find"; $start = $idx } else { $name = "find_existing"; $start = $idxBest }
    if ($start -lt 0) { [void]$sb.Append($text.Substring($i)); break }
    # copy prefix up to and including fn name + open paren
    $openParen = $start + $text.Substring($start).IndexOf("(")
    [void]$sb.Append($text.Substring($i, $openParen + 1 - $i))
    # scan balanced parens
    $depth = 1; $j = $openParen + 1
    while ($depth -gt 0 -and $j -lt $text.Length) {
      $c = $text[$j]
      if ($c -eq '(') { $depth++ } elseif ($c -eq ')') { $depth-- }
      $j++
    }
    $inner = $text.Substring($openParen + 1, $j - $openParen - 2)
    # split top-level commas
    $argsList = New-Object System.Collections.Generic.List[string]
    $d = 0; $cur = ""
    foreach ($ch in $inner.ToCharArray()) {
      if ($ch -eq '(' -or $ch -eq '<') { $d++ }
      elseif ($ch -eq ')' -or $ch -eq '>') { $d-- }
      if ($ch -eq ',' -and $d -eq 0) { $argsList.Add($cur.Trim()); $cur = "" } else { $cur += $ch }
    }
    if ($cur.Trim()) { $argsList.Add($cur.Trim()) }
    $req = $argsList[0]
    if ($req -eq "None") { $req = "&PythonRequest::Default" }
    elseif ($req -match '^Some\((.*)\)$') { $req = $Matches[1] }
    elseif ($req.EndsWith(".as_ref()")) { $req = $req.Substring(0, $req.Length - ".as_ref()".Length) + ".as_deref().unwrap_or(&PythonRequest::Default)" }
    $cacheArg = $argsList[5]
    [void]$sb.Append("$req, $($argsList[1]), $($argsList[2]), $cacheArg)")
    # skip trailing .await and ? 
    $k = $j
    while ($k -lt $text.Length -and $text[$k] -match '\s') { $k++ }
    if ($text.Substring($k).StartsWith(".await")) { $k += 6; while ($k -lt $text.Length -and $text[$k] -match '\s') { $k++ } }
    if ($k -lt $text.Length -and $text[$k] -eq '?') { $k++ }
    $i = $k
  }
  [System.IO.File]::WriteAllText($f, $sb.ToString(), [System.Text.UTF8Encoding]::new($false))
}
"converted"
