<#
The process of pipeline execution is:

0. PS parses the pipeline and runs the command binding algorithm (see below).
1. PS runs all the BeginProcessing clauses in all the commands.
2. PS runs the ProcessRecord of the first command in the pipeline.
   If that produces an object, it is passed to the ProcessRecord of the next
   command in the pipeline, and so on.
3. PS runs all the EndProcessing clauses in all the commands.
#>


<#
The process of parameter binding is as follows.
From Bruce Payette, page 61, para 2.5.2.

1. Bind all named parameters.

   Find all unquoted tokens on the command line that start with a dash. If the
   token ends with a colon, an argument is required. If there’s no colon, look
   at the type of the parameter and see if an argument is required. Convert the
   type of actual argument to the type required by the parameter, and bind the
   parameter.

2. Bind all positional parameters.

   If there are any arguments on the command line that haven’t been used, look
   for unbound parameters that take positional parameters and try to bind them.

3. Bind from the pipeline by value with exact match.

   If the command is not the first command in the pipeline and there are still
   unbound parameters that take pipeline input, try to bind to a parameter that
   matches the type exactly.

4. If not bound, then bind from the pipe by value with conversion.

   If the previous step failed, try to bind using a type conversion.

5. If not bound, then bind from the pipeline by name with exact match.

   If the previous step failed, look for a property on the input object that
   matches the name of the parameter. If the types exactly match, bind the
   parameter.

6. If not bound, then bind from the pipeline by name with conversion.

   If the input object has a property whose name matches the name of a
   parameter, and the type of the property is convertible to the type of the
   parameter, bind the parameter.

#>

# You can debug binding like this:
trace-command -Name ParameterBinding -Option All -Expression { 123 | Write-Output } -PSHost
