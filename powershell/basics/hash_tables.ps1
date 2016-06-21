$empty = @{ }
$user = @{ FirstName = "John"; LastName = "Smith"; PhoneNumber = "555-1212" }
$user

# We can use dot notation.
"The firstname property on our user hash is $($user.firstname)"

# Or array notation. Can use more than one key, returns more than one value.
$user["firstname", "lastname"]

# The underlying type of a hashtable:
$user.GetType().FullName

# Hence (with an example of sorting):
$user.keys | sort
$user.values


# $$ Powershell treats hashtables as scalar objects (Bruce Payette, p87).
# This means you can add a new key-value pair easily:
$user.Country = 'UK'
$user['County'] = 'Berkshire'
$user.remove('County')

"`nEnumerating a hash - this actually only loops once, because the hash is a
scalar. However, PS then dumps each kvp when it is printed!"
foreach ($v in $user)
{
    "In the loop"
    $v
}

"`nEnumerating a hash by kvps."
foreach ($v in $user.GetEnumerator())
{
    "In the loop"
    $v
}


"`nHashtables are reference types. Use .Clone() to make a copy."
$u2 = $user
$user.Country = 'France'
$u2
