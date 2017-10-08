# Iterator methods - https://doc.rust-lang.org/std/iter/trait.Iterator.html
Immutable slice iterator. This struct is created by the iter() method on slices.

See also: https://doc.rust-lang.org/std/iter/trait.IntoIterator.html
    "By implementing IntoIterator for a type, you define how it will be converted to an iterator. This is common
    for types which describe a collection of some kind. One benefit of implementing IntoIterator is that your type
    will work with Rust's for loop syntax."

### Construction & Conversion
Call iter() on a slice or other collection type.
fn clone(&self) -> Iter<'a, T>                      Returns a copy of the value.
fn clone_from(&mut self, source: &Self)             Performs copy-assignment from source. Read more
fn as_slice(&self) -> &'a [T]                       View the underlying data as a subslice of the original data.
fn cloned<'a, T>(self) -> Cloned<Self>              Creates an iterator which clones all of its elements.
fn by_ref(&mut self) -> &mut Self                   Borrows an iterator, rather than consuming it. This is useful to allow applying iterator adaptors while still retaining ownership of the original iterator.

### Queries
fn is_empty(&self) -> bool                          Returns whether the iterator is empty.
fn len(&self) -> usize                              Returns the exact number of times the iterator will iterate.
fn size_hint(&self) -> (usize, Option<usize>)       Returns the bounds on the remaining length of the iterator.
fn peekable(self) -> Peekable<Self>                 Creates an iterator which can use peek to look at the next element of the iterator without consuming it.
fn all<F>(&mut self, f: F) -> bool                  Tests if every element of the iterator matches a predicate.
fn any<F>(&mut self, f: F) -> bool                  Tests if any element of the iterator matches a predicate.

### Iterating
fn next(&mut self) -> Option<Self::Item>            Advances the iterator and returns the next value.
fn count(self) -> usize                             Consumes the iterator, counting the number of iterations and returning it.
fn nth(&mut self, n: usize) -> Option<Self::Item>   Returns the nth element of the iterator.
fn last(self) -> Option<Self::Item>                 Consumes the iterator, returning the last element.
fn step_by(self, step: usize) -> StepBy<Self>       Creates an iterator starting at the same point, but stepping by the given amount at each iteration.
fn enumerate(self) -> Enumerate<Self>               Creates an iterator which gives the current iteration count as well as the next value.
fn fuse(self) -> Fuse<Self>                         Creates an iterator which ends after the first None. fuse() adapts an iterator, ensuring that after a None is given, it will always return None forever.
fn inspect<F>(self, f: F) -> Inspect<Self, F>       [For debugging] Do something with each element of an iterator, passing the value on.
fn rev(self) -> Rev<Self>                           Reverses an iterator's direction.
fn cycle(self) -> Cycle<Self>                       Repeats an iterator endlessly.
fn chain<U>(self, other: U) -> Chain<Self, <U as IntoIterator>::IntoIter>   Takes two iterators and creates a new iterator over both in sequence.

### Mapping & Zipping
fn for_each<F>(self, f: F)                                                  Calls a closure on each element of an iterator.
fn map<B, F>(self, f: F) -> Map<Self, F>                                    Takes a closure and creates an iterator which calls that closure on each element.
fn flat_map<U, F>(self, f: F) -> FlatMap<Self, U, F>                        Creates an iterator that works like map, but flattens nested structure.
fn zip<U>(self, other: U) -> Zip<Self, <U as IntoIterator>::IntoIter>       'Zips up' two iterators into a single iterator of pairs.
fn unzip<A, B, FromA, FromB>(self) -> (FromA, FromB)                        Converts an iterator of pairs into a pair of containers. The opposite of zip().

### Filtering
fn filter<P>(self, predicate: P) -> Filter<Self, P>         Creates an iterator which uses a closure to determine if an element should be yielded.
fn filter_map<B, F>(self, f: F) -> FilterMap<Self, F>       Creates an iterator that both filters and maps. The closure must return an Option<T>.
fn skip(self, n: usize) -> Skip<Self>                       Creates an iterator that skips the first n elements.
fn take(self, n: usize) -> Take<Self>                       Creates an iterator that yields its first n elements.
fn skip_while<P>(self, predicate: P) -> SkipWhile<Self, P>  Creates an iterator that skips elements based on a predicate.
fn take_while<P>(self, predicate: P) -> TakeWhile<Self, P>  Creates an iterator that yields elements based on a predicate.
fn partition<B, F>(self, f: F) -> (B, B)                    Consumes an iterator, creating two collections from it which are disjoint by the predicate.

### Finding
fn find<P>(&mut self, predicate: P) -> Option<Self::Item>   Searches for an element of an iterator that satisfies a predicate.
fn position<P>(&mut self, predicate: P) -> Option<usize>    Searches for an element in an iterator, returning its index.
fn rposition<P>(&mut self, predicate: P) -> Option<usize>   Version of position<P> that scans backwards from the end.

### Reducing
fn collect<A>(self) -> A                                    Transforms an iterator into a collection.
fn fold<A, F>(self, init: A, f: F) -> A                     An iterator adaptor that applies a function, producing a single, final value.
fn min(self) -> Option<Self::Item>                          Returns the minimum element of an iterator.
fn min_by_key<A, F>(self, f: F) -> Option<Self::Item>       Returns the element that gives the minimum value from the specified function.
fn min_by<F>(self, compare: F) -> Option<Self::Item>        Returns the element that gives the minimum value with respect to the specified comparison function.
fn max(self) -> Option<Self::Item>                          Returns the maximum element of an iterator.
fn max_by_key<A, F>(self, f: F) -> Option<Self::Item>       Returns the element that gives the maximum value from the specified function.
fn max_by<F>(self, compare: F) -> Option<Self::Item>        Returns the element that gives the maximum value with respect to the specified comparison function.
fn sum<S>(self) -> S                                        Sums the elements of an iterator.
fn product<P>(self) -> P                                    Iterates over the entire iterator, multiplying all the elements
fn scan<St, A, F>(self, initial_state: St, f: F) -> Scan<Self, St, F>   An iterator adaptor similar to fold that holds internal state and produces a new iterator.

### Free Functions
pub fn once<T>(value: T) -> Once<T>         Creates an iterator that yields an element exactly once. This is commonly used to adapt a single value into a chain of other kinds of iteration.
pub fn repeat<T>(elt: T) -> Repeat<T>       Creates a new iterator that endlessly repeats a single element.
pub fn empty<T>() -> Empty<T>               Creates an iterator that yields nothing.
