# $$ Compound assignment
$a, $bc, $c = 1,2,3,4
Write-Output "`$a = $a, `$b = $b, `$c = $c"

# $$ Arithmetic operators
#  + *
#  - / %    (only on numbers, unless the .Net type support it, e.g. DateTime)
2 + 5
"hello " + "world"

# Works for arrays or collections too.
$arr1 = 5,6,7
$arr2 = 8,9,10
$arr1 + $arr2

$innerPlanets = @{ Mercury = 0; Mars = 2 }
$outerPlanets = @{ Jupiter = 14; Saturn = 21 }
$innerPlanets + $outerPlanets

"abc" * 0 # Empty string
"abc" * 3
$arr1 * 3

# Automatic type conversion to the type of the left hand operand.
2 + "123"


# $$ Case sensitivity
# For an operator "op"
#    op is case-insensitive
#   iop is also case-sensitive and rarely used
#   cop is case-sensitive
# There are c- and i- versions of virtually everything, excepting join.


# $$ Comparison operators
#  eq ne gt ge lt le
# These operators are PowerShell built-ins for strings and numbers.
# For other types, IComparable is used.
# They work on scalars and collections. Scalar examples first:
Write-Output "01 -eq 001 is $(01 -eq 001) because the lhs is a number"
Write-Output "01 -eq `"001`" is $(01 -eq `"001`") because the rhs is converted to the type of the lhs"
Write-Output "`"01`" -eq 001 is $(`"01`" -eq 001) because the rhs is converted to a string"
Write-Output "[int] `"01`" -eq 001 is $([int] `"01`" -eq 001) because we casted the lhs to int"

$d1 = [DateTime] "2001-01-01"
$d2 = [DateTime] "2015-01-01"
Write-Output "$d1 -lt $d2 is $($d1 -lt $d2)"

# Now some collection examples.
# If the lhs is a collection, the operator returns the matching elements.
$arr1 = 1,2,3,4,1,2,2,1,2
Write-Output "Elements equal to 2:"
$arr1 -eq 2
Write-Output "Elements greater than 3:"
$arr1 -gt 3
Write-Output "It works with strings too:"
"alpha", "beta", "gamma", "delta" -gt "c"


# $$ Containment operators
#  [ci]contains [ci]notcontains
Write-Output "Containment operators return a boolean:"
Write-Output "1,2,3 -contains 4 is $(1,2,3 -contains 4)"


# $$ Pattern matching
#  like notlike      - for use with wildcard patterns
#  match notmatch    - for use with regular expressions
#
# Wildcard patterns are used with filenames. They support
# *, ?, [abc] (any one of) and [a-z] (any char in range)
Write-Output "'hello' -like 'h*' is $('hello' -like 'h*')"
Write-Output "'hello' -like 'H*' is $('hello' -like 'H*')"
Write-Output "'hello' -clike 'H*' is $('hello' -clike 'H*')"

# Regexes are the standard .Net implementation.
# match returns $true or $false and sets the special $matches variable
# which is a hash table.
Write-Output "'Hello' -match '[h]' is $('Hello' -match '[h]')"
Write-Output "'Hello' -cmatch '[h]' is $('Hello' -cmatch '[h]')"

Write-Output 'A parenthetical expression creates a match group which is then
available via the $matches variable. $matches = '
'abc' -match '(a)(b)(c)' > $null
$matches

Write-Output "You can use named captures:"
'abc' -match '(?<first>a)(b)(?<last>c)' > $null
$matches
Write-Output "Because `$matches is a hash table, we can do `$matches.first = $($matches.first)"


# $$ Text operators
#  [ci]replace [ci]split join
# replace does regex replacements. It does not change the string, it
# returns a new one (standard .Net string immutability).
'hello cruel world' -replace 'cruel', 'Cruel'
'hello cruel world' -replace 'cruel '            # Remove a string by omitting the second arg

# The $1 is not expanded as a PowerShell variable because it is in single quotes.
# Instead, it is a regex operator meaning "the matched text", i.e exactly the
# same as the $matches variable works HOWEVER replace does NOT set $matches.
# See https://msdn.microsoft.com/en-us/library/ewy2t5e0%28v=vs.110%29.aspx
'The car is red, the bus is blue, and the truck is green' -replace 'is (red|blue)', 'was $1'

# Unary join creates a string from a collection. The brackets are needed
# because comma binds tighter than -join.
Write-Output "-join (1,2,3) is $(-join (1,2,3))"

# Binary join allows you to specify a join string.
Write-Output "1,2,3,4 -join ',' is $(@(1,2,3,4) -join ',')"


# Unary split splits on whitespace.
Write-Output "-split 'a b c' is $(-split 'a b c')"

# Binary split syntax is:
#   string-to-split -split split-pattern,num-to-return,Options
# Typical options: SimpleMatch, IgnoreCase, MultiLine
# split-pattern is a regex unless SimpleMatch is used.
Write-Output "'a:b:c:d:e' -split ':' is $('a:b:c:d:e' -split ':')"
Write-Output "'a:b:c:d:e' -split ':',3 is $('a:b:c:d:e' -split ':',3)"
Write-Output "'a*b*c' -split '*',0,'SimpleMatch' is $('a*b*c' -split '*',0,'simplematch')"

# You can pass a scriptblock which functions as a predicate.
# This example from Payette, p148.
$colors = "Black,Brown,Red,Orange,Yellow,Green,Blue,Violet,Gray,White"
$count=@(0)
$colors -split {$_ -eq "," -and ++$count[0] % 2 -eq 0 }


# $$ Logical and bitwise operators
#  and or not xor
#  band bor bnot bxor
Write-Output "$true -and $false is $($true -and $false)"
Write-Output "0x55 -band 0x32 is $(0x55 -band 0x23)"


# $$ Type operators.
# We have -is, -as and -isnot, they work the same way as in C#.


# $$ Dot operator.
# The rhs can be an expression.
("*" * 5).("len" + "gth")
