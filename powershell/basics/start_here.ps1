<#
You can do multiline
comments like this.
#>

# $$ Environment.
#   $PSVersionTable.PSVersion       # To find the version of Powershell that is running
#   $PSHOME                         # PS home directory.


# $$ Basic structure of a command:
#   command parameter1 parameter2-with-arg1 positional-arg
# Parameter names do not have to be fully specified, just a unique
# case-insensitive prefix. You can use -- to mark the end of parameters.


# $$ There are four types of commands:
#   cmdlets, functions, script commands, and native Windows commands.


# $$ To get help on a command or alias:
#   get-help dir                    # Show help in console
#   get-help -online dir            # Show help online (more up to date)
#   get-command dir                 # Show aliases for a command
#   get-command get-childitem | fl  # Show a command (but you might as well use get-help)
#   get-alias (or gal)              # Show all aliases
#   get-alias dir                   # Get a specific alias
#   get-alias -name *w*             # Get alias where the alias name matches wildcards.
#   get-alias -definition *web*     # Get alias where the aliased command matches the wildcards.


# $$ Object help.
#   [string] | get-member -static   # Show static members on a type.
#   (23).GetType() | gm -static     # Show static members from a literal.
#   $a.GetType() | gm -static       # Show static members from a variable.
#   2 | gm                          # Show instance methods.
#   $a | format-custom              # Show object graph.


# $$ Single quoting.
# 'single quotes'                   # Quote without substitution (WYSIWYG).
# 'more `backquoted'                # Backtick is not special in single-quoted strings.
# cd c:\program` files              # Quote a single character using a backtick.
# write-output some`                # Backtick is also used for line continuation.
# multiline`
# string


# $$ Double quoting.
# "Expand $something"               # Variables are expanded in double quotes.
# "Expand `$something"              # Bactick suppresses expansion.
# "Hello 'cruel' world"             # Quotes can be nested in both directions.
# 'Hello "cruel" world'
# "`n `t `a `b `' `" `0 ``"         # Set of escape sequences valid in double quoted strings.
# write-output "Multi               # Double quoted strings can span lines.
# line
# strings are easy"


# $$ Parsing.
# TODO: expression mode vs command mode.
# cd $HOME/Documents                # When a variable is part of a larger string it is expanded
                                    # (i.e. treated as if it were in double quotes).
