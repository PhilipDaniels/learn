<#
Bruce Payette, p97.

[int]             System.Int32
[long]            System.Int64
[string]          System.String
[char]            System.Char
[bool]            System.Boolean
[byte]            System.Byte
[double]          System.Double
[decimal]         System.Decimal
[float]           System.Single
[single]          System.Single
[regex]           System.Text.RegularExpressions.Regex
[array]           System.Array
[xml]             System.Xml.XmlDocument
[scriptblock]     System.Management.Automation.ScriptBlock
[switch]          System.Management.Automation.SwitchParameter
[hashtable]       System.Collections.Hashtable
[ref]             System.Management.Automation.PSReference
[type]            System.Type
[psobject]        System.Management.Automation.PSObject
[pscustomobject]  System.Management.Automation.PSObject
[psmoduleinfo]    System.Management.Automation.PSModuleInfo
[powershell]      System.Management.Automation.PowerShell
[runspacefactory] System.Management.Runspaces.RunspaceFactory
[runspace]        System.Management.Automation.Runspaces.Runspace
[ipaddress]       System.Net.IPAddress
[wmi]             System.Management.ManagementObject
[wmisearcher]     System.Management.ManagementClass
[wmiclass]        System.Management.ManagementClass
[adsi]            System.DirectoryServices.DirectoryEntry
[adsisearcher]    System.DirectoryServices.DirectorySearcher

They are not case-sensitive, and PowerShell automatically prepends "System."
so you can get away with [io.file]::ReadAllLines(...)
#>

# $$ Declare.
[long] $x = 2
$x.GetType().FullName


# $$ Cast.
$y = [decimal] 12;
$y.GetType().FullName


# $$ Generics.
# Best-practice to include the single quote, though the example for the list
# will work without it - the dictionary example will not.
$int_list = new-object 'system.collections.generic.list[int]'
$int_list.add(2)
$int_list
$int_list.GetType().FullName

$dict = new-object 'system.collections.generic.dictionary[string,int]'
$dict.add('hello', 42)
$dict['world'] = 84
$dict
$dict.remove('hello') > $null


# $$ Static members.
# This creates a System.Type object then passes it to get-member.
# (See start_here.ps1 for more examples of get-member).
[string] | get-member -static

[string]::IsNullOrWhitespace("  ")
[string]::Format("Hello World at {0}", (get-date))
