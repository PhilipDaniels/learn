# Arrays are standard .Net arrays, with elements of type System.Object.  Like in
# .Net they are reference types, however there is a magic concat operator that
# creates new arrays and copies elements over (see below).


# $$ Creation.
# You make an array using the comma operator. It gets created as an array
# of System.Object (the same as the objects in a pipeline).
$singleton = , 'Can use comma to create an array with 1 element'
$a = 1,2,3
$a
$a.GetType().FullName
"The length is $($a.length)"

"`n$singleton`n"


$a[1] = 'arrays are polymorphic because they store System.Object'
$a


# $$ Range checking.
# Indexing outside the bounds will fail.
# $a[4] = 'This will not work'

# But you can add new items via concat. (Which actually creates a new array and
# copies the objects over).
$a += 22, 33
$a


# $$ Reference type.
# However note that arrays are reference types, so this means
# thot both variables are pointing to the same object.
$b = $a

$b[0] = 'new value in b'
"`n`nPrinting a after mutation via another reference"
$a


# $$ Array wrapping.
# The @(....) special sub-expression form forces the enclosed expression to be
# returned as an array, even if it is empty or contains only 1 element. If the
# sub-expression is already an array it will NOT be wrapped in another array.
# n.b. The comma operator ALWAYS wraps the expression in an array.
$empty = @()
$singleton = @("foo")


# $$ Converting a scalar value into an array of a specific type.
''
$i = [int[]] "123"
$i


# $$ Range operator
Write-Output "1..4 is $(1..4)"
$arr = 1..10
$arr[0]
$arr[-1]      # Last element.
$arr[1,2,3]   # Elements at indexes 1, 2 and 3.
$arr[4..8]
