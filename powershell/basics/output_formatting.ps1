# PS has a database of formatting information for various types of
# objects. This is part of the "extended type system".
dir $PSHOME/*format* | Format-Table name

# $$ Different types of formatting commands
get-command format-*

get-item c:\ | format-table                       # (ft) Show a table across the screen
get-item c:\ | format-list                        # (fl) Shows each property on a line.
get-process -Name s* | format-wide -Column 8 id   # (fw) Show a single property (id) in 8 columns.
get-item c:\ | format-custom -depth 1             # (fc) Shows the object graph.


# $$ Output destinations.
# By default, objects are sent to Out-Default which is tied to Out-Host.
get-command out-*                                 # Show all possible output destinations.


# $$ /dev/null. These are equivalent.
get-command out-* | out-null
get-command out-* > $null
