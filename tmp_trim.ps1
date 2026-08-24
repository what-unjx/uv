param([string[]]$Files)
foreach ($f in $Files) {
  $text = [System.IO.File]::ReadAllText($f)
  $sb = New-Object System.Text.StringBuilder
  $i = 0
  $marker = "PythonInstallation::find("
  while ($i -lt $text.Length) {
    $start = $text.IndexOf($marker, $i)
    if ($start -lt 0) { [void]$sb.Append($text.Substring($i)); break }
    if ($start -gt $i) { [void]$sb.Append($text.Substring($i, $start - $i)) }
    $openParen = $start + $marker.Length - 1
    [void]$sb.Append("PythonInstallation::find(")
    $depth = 1; $j = $openParen + 1
    while ($depth -gt 0 -and $j -lt $text.Length) {
      $c = $text[$j]
      if ($c -eq '(') { $depth++ } elseif ($c -eq ')') { $depth-- }
      $j++
    }
    $inner = $text.Substring($openParen + 1, $j - $openParen - 2)
    $argsList = New-Object System.Collections.Generic.List[string]
    $d = 0; $cur = ""
    foreach ($ch in $inner.ToCharArray()) {
      if ($ch -eq '(' -or $ch -eq '<') { $d++ }
      elseif ($ch -eq ')' -or $ch -eq '>') { $d-- }
      if ($ch -eq ',' -and $d -eq 0) { $argsList.Add($cur.Trim()); $cur = "" } else { $cur += $ch }
    }
    if ($cur.Trim()) { $argsList.Add($cur.Trim()) }
    if ($argsList.Count -gt 4) {
      $req = $argsList[0]
      if ($req -eq "None") { $req = "&PythonRequest::Default" }
      elseif ($req -match '^Some\((.*)\)$') { $req = $Matches[1].Trim() }
      elseif ($req.EndsWith(".as_ref()")) { $req = $req.Substring(0, $req.Length - ".as_ref()".Length) + ".as_deref().unwrap_or(&PythonRequest::Default)" }
      elseif ($req.EndsWith(".as_deref()")) { $req = $req.Substring(0, $req.Length - ".as_deref()".Length) + ".as_deref().unwrap_or(&PythonRequest::Default)" }
      [void]$sb.Append("$req, $($argsList[1]), $($argsList[2]), $($argsList[5]))")
    } else {
      [void]$sb.Append($inner + ")")
    }
    $i = $j
    # consume trailing .await and ?
    while ($i -lt $text.Length -and [char]::IsWhiteSpace($text[$i])) { $i++ }
    if ($i + 5 -le $text.Length -and $text.Substring($i, 6) -eq ".await") { $i += 6 }
    while ($i -lt $text.Length -and [char]::IsWhiteSpace($text[$i])) { $i++ }
    if ($i -lt $text.Length -and $text[$i] -eq '?') { $i++ }
  }
  [System.IO.File]::WriteAllText($f, $sb.ToString(), [System.Text.UTF8Encoding]::new($false))
}
"done"
