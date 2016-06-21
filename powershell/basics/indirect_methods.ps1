# This gives a method description object, a PSMethod
$m = "abc".substring
$m

# Invoke the method. Automatically picks the right overload.
$m.Invoke(1)
$m.Invoke(0, 2)

# Works for static too.
$m = "sin"
[math]::$m
