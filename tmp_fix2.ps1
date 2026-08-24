param([string[]]$Files)
foreach ($f in $Files) {
  $text = [System.IO.File]::ReadAllText($f)
  $orig = $text

  # 1) as_deref -> as_ref
  $text = $text.Replace(".as_deref().unwrap_or(&PythonRequest::Default)", ".as_ref().unwrap_or(&PythonRequest::Default)")

  # 2) add missing ? after find(...) when next line chains a method
  #    matches: "cache)" EOL, then indentation + "."method
  $pattern = '(?m)^(PythonInstallation::find\([^\n]*cache)\)\r?(\n)(\s*)(\.(?:into_interpreter|interpreter|source|key|python_version|implementation)\b)'
  $text = [regex]::Replace($text, $pattern, '$1)?$2$3$4')

  if ($text -ne $orig) { [System.IO.File]::WriteAllText($f, $text, [System.Text.UTF8Encoding]::new($false)); "fixed: $f" }
}
"done"
