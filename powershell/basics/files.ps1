
# $$ Renaming
# This uses a script block to change the extension.
dir pd*.xml | Rename-Item -Path {$_.name} -NewName {$_.name -replace '\.xml$', '.txt'} -whatif


# $$ Parsing lines
# Simulate a file using a string.
$data = "quiet 0 25
normal 26 50
loud 51 75
noisy 75 100"

($data -split "\n") | foreach {
    $e = @{ }
    $e.level, [int] $e.lower, [int] $e.upper = -split $_
    $e
}

Write-Output "Now using the file sound_levels.txt:"
Get-Content sound_levels.txt | foreach {
    $e = @{ }
    $e.level, [int] $e.lower, [int] $e.upper = -split $_
    # You can throw some fields away like this.
    # $e.level, $null, [int] $e.upper = -split $_
    $e
}


# $$ DotNet File class static methods
$data = [IO.File]::ReadAllLines('sound_levels.txt')
Write-Output "`$data has type $($data.GetType().FullName) but the next stmt prints all on one line"
Write-Output "Using ReadAllLines: $data"
