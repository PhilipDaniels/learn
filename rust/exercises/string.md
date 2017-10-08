# str methods - https://doc.rust-lang.org/std/primitive.str.html
The str type, also called a 'string slice', is the most primitive string type. It is usually seen in its borrowed form, &str. It is also the type of string literals, &'static str.

Strings slices are always valid UTF-8.


### Construction and Conversion
fn to_owned(&self) -> String                            Converts to a String.
fn to_string(&self) -> String                           Converts to a String.
fn repeat(&self, n: usize) -> String                    Create a String by repeating a string n times.
fn as_bytes(&self) -> &[u8]                             Converts a string slice to a byte slice.
fn parse<F>(&self) -> Result<F, <F as FromStr>::Err>    Parses this string slice into another type.
fn into_boxed_bytes(self: Box<str>) -> Box<[u8]>        Converts a Box<str> into a Box<[u8]> without copying or allocating.
fn into_string(self: Box<str>) -> String                Converts a Box<str> into a String without copying or allocating.

### Queries
fn len(&self) -> usize                                  Returns the length of self, in bytes, not chars.
fn is_empty(&self) -> bool                              Returns true if self has a length of zero bytes.
fn is_char_boundary(&self, index: usize) -> bool        Checks that index-th byte lies at the start and/or end of a UTF-8 code point sequence.
fn contains<'a, P>(&'a self, pat: P) -> bool            Returns true if the given pattern matches a sub-slice of this string slice.
fn starts_with<'a, P>(&'a self, pat: P) -> bool         Returns true if the given pattern matches a prefix of this string slice.
fn ends_with<'a, P>(&'a self, pat: P) -> bool           Returns true if the given pattern matches a suffix of this string slice.

### Searching
fn find<'a, P>(&'a self, pat: P) -> Option<usize>                           Returns the byte index of the first character of this string slice that matches the pattern.
fn rfind<'a, P>(&'a self, pat: P) -> Option<usize>                          Returns the byte index of the last character of this string slice that matches the pattern.
fn matches<'a, P>(&'a self, pat: P) -> Matches<'a, P>                       An iterator over the disjoint matches of a pattern within the given string slice.
fn rmatches<'a, P>(&'a self, pat: P) -> RMatches<'a, P>                     An iterator over the disjoint matches of a pattern within this string slice, yielded in reverse order.
fn match_indices<'a, P>(&'a self, pat: P) -> MatchIndices<'a, P>            An iterator over the disjoint matches of a pattern within this string slice as well as the index that the match starts at.
fn rmatch_indices<'a, P>(&'a self, pat: P) -> RMatchIndices<'a, P>          An iterator over the disjoint matches of a pattern within self, yielded in reverse order along with the index of the match.

### Replacing
fn replace<'a, P>(&'a self, from: P, to: &str) -> String                    Replaces all matches of a pattern with another string.
fn replacen<'a, P>(&'a self, pat: P, to: &str, count: usize) -> String      Replaces first N matches of a pattern with another string.

### Trimming
fn trim(&self) -> &str                                                      Returns a string slice with leading and trailing whitespace removed.
fn trim_left(&self) -> &str                                                 Returns a string slice with leading whitespace removed.
fn trim_right(&self) -> &str                                                Returns a string slice with trailing whitespace removed.
fn trim_matches<'a, P>(&'a self, pat: P) -> &'a str                         Returns a string slice with all prefixes and suffixes that match a pattern repeatedly removed.
fn trim_left_matches<'a, P>(&'a self, pat: P) -> &'a str                    Returns a string slice with all prefixes that match a pattern repeatedly removed.
fn trim_right_matches<'a, P>(&'a self, pat: P) -> &'a str                   Returns a string slice with all suffixes that match a pattern repeatedly removed.

### Case conversion
fn to_lowercase(&self) -> String                                            Returns the lowercase equivalent of this string slice, as a new String.
fn to_uppercase(&self) -> String                                            Returns the uppercase equivalent of this string slice, as a new String.

### Sub-slicing
fn get<I>(&self, i: I) -> Option<&<I as SliceIndex<str>>::Output>               Returns a subslice of str (non-panicking).
fn get_mut<I>(&mut self, i: I) -> Option<&mut <I as SliceIndex<str>>::Output>   Mutable version of get().

### Iterating
fn chars(&self) -> Chars                                                    Returns an iterator over the chars of a string slice.
fn char_indices(&self) -> CharIndices                                       Returns an iterator over the chars of a string slice, and their positions.
n bytes(&self) -> Bytes                                                     Returns an iterator over the bytes of a string slice.
fn lines(&self) -> Lines                                                    Returns an iterator over the lines of a string, as string slices.

### Splitting
fn split_at(&self, mid: usize) -> (&str, &str)                              Divide one string slice into two at an index.
fn split_at_mut(&mut self, mid: usize) -> (&mut str, &mut str)              Mutable version of split_at().
fn split_whitespace(&self) -> SplitWhitespace                               Split a string slice by whitespace.
fn split<'a, P>(&'a self, pat: P) -> Split<'a, P>                           An iterator over substrings of this string slice, separated by characters matched by a pattern.
fn rsplit<'a, P>(&'a self, pat: P) -> RSplit<'a, P>                         An iterator over substrings of the given string slice, separated by characters matched by a pattern and yielded in reverse order.
fn split_terminator<'a, P>(&'a self, pat: P) -> SplitTerminator<'a, P>      An iterator over substrings of the given string slice, separated by characters matched by a pattern.
fn rsplit_terminator<'a, P>(&'a self, pat: P) -> RSplitTerminator<'a, P>    An iterator over substrings of self, separated by characters matched by a pattern and yielded in reverse order.
fn splitn<'a, P>(&'a self, n: usize, pat: P) -> SplitN<'a, P>               An iterator over substrings of the given string slice, separated by a pattern, restricted to returning at most n items.
fn rsplitn<'a, P>(&'a self, n: usize, pat: P) -> RSplitN<'a, P>             An iterator over substrings of this string slice, separated by a pattern, starting from the end of the string, restricted to returning at most n items.



# String methods - https://doc.rust-lang.org/std/string/struct.String.html
A UTF-8 encoded, growable string.

Strings implement Deref<Target=str>, and so inherit all of str's methods. In addition, this means
that you can pass a String to any function which takes a &str by using an ampersand (&).

### Construction and Conversion
fn new() -> String                                              Creates a new empty String.
fn with_capacity(capacity: usize) -> String                     Creates a new empty String with a particular capacity.
fn as_str(&self) -> &str                                        Extracts a string slice containing the entire string.
fn as_mut_str(&mut self) -> &mut str                            Mutable version of as_str().
fn from_utf8(vec: Vec<u8>) -> Result<String, FromUtf8Error>     Converts a vector of bytes to a String.
fn from_utf8_lossy(v: &'a [u8]) -> Cow<'a, str>                 Converts a slice of bytes to a string, including invalid characters.
fn from_utf16(v: &[u16]) -> Result<String, FromUtf16Error>      Decode a UTF-16 encoded vector v into a String, returning Err if v contains any invalid data.
fn from_utf16_lossy(v: &[u16]) -> String                        Decode a UTF-16 encoded vector v into a string, replacing invalid data with the replacement character (U+FFFD).
fn into_bytes(self) -> Vec<u8>                                  Converts a String into a byte vector.
fn as_bytes(&self) -> &[u8]                                     Returns a byte slice of this String's contents.
fn into_boxed_str(self) -> Box<str>                             Converts this String into a Box<str>.
fn borrow(&self) -> &str                                        Immutably borrows from an owned value.
fn clone(&self) -> String                                       Clones self.
fn deref(&self) -> &str                                         The method called to dereference a value

### Queries
fn is_empty(&self) -> bool                                      Returns true if this String has a length of zero.
fn len(&self) -> usize                                          Returns the length of this String, in bytes.
fn capacity(&self) -> usize                                     Returns this String's capacity, in bytes.

### Adding things
fn push_str(&mut self, string: &str)                            Appends a given string slice onto the end of this String.
fn push(&mut self, ch: char)                                    Appends the given char to the end of this String.
fn insert(&mut self, idx: usize, ch: char)                      Inserts a character into this String at a byte position.
fn insert_str(&mut self, idx: usize, string: &str)              Inserts a string slice into this String at a byte position.
fn extend<I>(&mut self, iter: I)                                Extends the string with the contents of an iterator.

### Removing things
fn truncate(&mut self, new_len: usize)                          Shortens this String to the specified length.
fn pop(&mut self) -> Option<char>                               Removes the last character from the string buffer and returns it.
fn remove(&mut self, idx: usize) -> char                        Removes a char from this String at a byte position and returns it.
fn clear(&mut self)                                             Truncates this String, removing all contents.
fn drain<R>(&mut self, range: R) -> Drain                       Creates a draining iterator that removes the specified range in the string and yields the removed chars.
fn splice<R>(&'a mut self, range: R, replace_with: &'b str) -> Splice<'a, 'b>   Creates a splicing iterator that removes the specified range in the string, replaces with the given string, and yields the removed chars.

### Memory Management
fn reserve(&mut self, additional: usize)                        Ensures that this String's capacity is at least additional bytes larger than its length.
fn reserve_exact(&mut self, additional: usize)                  Ensures that this String's capacity is additional bytes larger than its length.
fn shrink_to_fit(&mut self)                                     Shrinks the capacity of this String to match its length.

### Splitting
fn split_off(&mut self, at: usize) -> String                    Splits the string into two at the given index.

### Concatentation
impl<'a> Add<&'a str> for String
    let c = a + &b;                                             The rhs must be borrowed. a is mutated.
    let c = a.clone() + &b;                                     This form allows you to continue to use a.
