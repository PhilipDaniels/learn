# dir > out.txt                       Redirect stdout
# dir >> out.txt                      Append stdout
# dir nosuchfile.txt 2> err.txt       Redirect stderr
# dir nosuchfile.txt 2>> err.txt      Append stderr
# dir nosuchfile.txt 2>&1 > out.txt   Tie stderr to stdout then redirect
# dir nosuchfile.txt > $null          Equivalent of /dev/null

# Redirection is performed using the Out-File cmdlet.
# get-help out-file for some interesting options.
