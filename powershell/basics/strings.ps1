# $$ Single quoting. Variables are not expanded.
'a `backquoted string'              # Backtick is not special in single-quoted strings.

#write-output some`                  # Backtick is also used for line continuation.
#multiline`
  #string


# $$ Double quoting. Variables ARE expanded.
$something = 'Jupiter'
"Expand $something"                 # Variables are expanded in double quotes.
"Expand `$something"                # Backtick suppresses expansion.
"Hello 'cruel' world"               # Quotes can be nested in both directions.
'Hello "cruel" world'
"Escapes: `n `t `a `b `' `" `0 ``"  # Set of escape sequences valid in double quoted strings.


# $$ Double quoted strings can span lines.
write-output "Multi
line
strings are easy"


# $$ Subexpression expansion.
"2 + 2 is $(2 + 2)"
"2 + 2 is `$(2 + 2)"                # Just the usual suppression technique.

# Statements lists are expanded by evaluating each and concat with a space
# separator. Here the simple statements are just the integers 1, 2 and 3.
"Expanding three statements in a string: $(1; 2; 3)"

"Numbers 1 thru 10: $(for ($i=1; $i -le 10; $i++) { $i })."


# $$ Timing of string expansion. (Bruce Payette, p80).
# Expansion occurs when an assignment is executed.
'Timing 1: $a is only assigned once, so $x gets the value 1.'
$x = 0
$a = "x is $($x++; $x)"
1..3 | foreach { $a }

'Timing 2: The string is evaluated once per iteration, so $x increases.'
$x = 0
1..3 | foreach { "x is $($x++; $x)" }

# You can force expansion to occur using:
# $expanded = $ExecutionContext.InvokeCommand.ExpandString('a is $a')


# $$ HERE strings. The @" and "@ must be on their own lines.
# Expansion occurs here because we used double quotes, but the single quote
# variant also exists.
$hs = @"
Line one
Line two
Line three
Hello "world"
The date is "$(get-date)"
"@

$hs


# $$ Format operator.
# This is a call to String.Format.
#   formatstring -f array-of-values
'{2} {1} {0}' -f 100,200,300
