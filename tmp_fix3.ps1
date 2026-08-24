param([string[]]$Files)
foreach ($f in $Files) {
  $text = [System.IO.File]::ReadAllText($f)
  $orig = $text
  $text = $text.Replace(".as_deref().unwrap_or(&PythonRequest::Default)", ".as_ref().unwrap_or(&PythonRequest::Default)")
  $pattern = '(?m)^(\s*)(PythonInstallation::find\([^\r\n]*cache)\)\r?\n(\s*)((?:into_interpreter|interpreter|source|key|python_version|implementation)\b)'
  $text = [regex]::Replace($text, $pattern, ('$1' + '$2?$3' + '`n' + '$4'.Replace('`n','')) -replace '\$1', '$1')
  # .NET replacement: build explicitly
  $pattern2 = '(?m)^(\s*)(PythonInstallation::find\([^\r\n]*cache)\)\r?\n(\s*)((?:into_interpreter|interpreter|source|key|python_version|implementation)\b)'
  $text = [regex]::Replace($text, $pattern2, { param($m) $m.Groups[1].Value + $m.Groups[2].Value + "?" + "`r`n" + $m.Groups[3].Value + $m.Groups[4].Value })
  if ($text -ne $orig) { [System.IO.File]::WriteAllText($f, $text, [System.Text.UTF8Encoding]::new($false)); "fixed: $f" }
}
"done"
