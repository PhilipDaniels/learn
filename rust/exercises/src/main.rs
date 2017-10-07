#![allow(dead_code)]
#![allow(warnings)]

mod hashtables;
mod vectors;

fn main() {
    hashtables::intmap::run();
    hashtables::stringmap::run();
    hashtables::ofrefcellvalues::run();
    vectors::ofints::run();
}

/*
Slice methods - https://doc.rust-lang.org/std/primitive.slice.html
========================================================================================================================================================================================================
Slices are either mutable or shared. The shared slice type is &[T], while the mutable slice type is &mut [T]

Construction
============
fn clone_from_slice(&mut self, src: &[T])       Copies the elements from src into self. The length of src must be the same as self.
fn copy_from_slice(&mut self, src: &[T])        Copies all elements from src into self, using a memcpy. The length of src must be the same as self.
fn to_vec(&self) -> Vec<T>                      Copies self into a new Vec.
fn into_vec(self: Box<[T]>) -> Vec<T>           Converts self into a vector without clones or allocation.
fn to_owned(&self) -> Vec<T>                    Creates owned data from borrowed data, usually by cloning.

Queries
=======
fn len(&self) -> usize                          Returns the number of elements in the slice.
fn is_empty(&self) -> bool                      Returns true if the slice contains no elements.
fn contains(&self, x: &T) -> bool               Returns true if the slice contains an element with the given value.
fn starts_with(&self, needle: &[T]) -> bool     Returns true if needle is a prefix of the slice.
fn ends_with(&self, needle: &[T]) -> bool       Returns true if needle is a suffix of the slice.
fn binary_search(&self, x: &T) -> Result<usize, usize>                              Binary searches this sorted slice for a given element.
fn binary_search_by<'a, F>(&'a self, f: F) -> Result<usize, usize>                  Binary searches this sorted slice with a comparator function.
fn binary_search_by_key<'a, B, F>(&'a self, b: &B, f: F) -> Result<usize, usize>    Binary searches this sorted slice with a key extraction function.

Element Access & Change
=======================
fn first(&self) -> Option<&T>                   Returns the first element of the slice, or None if it is empty.
fn first_mut(&mut self) -> Option<&mut T>       Returns a mutable pointer to the first element of the slice, or None if it is empty.
fn last(&self) -> Option<&T>                    Returns the last element of the slice, or None if it is empty.
fn last_mut(&mut self) -> Option<&mut T>        Returns a mutable pointer to the last item in the slice.
fn get<I>(&self, index: I) -> Option<&<I as SliceIndex<[T]>>::Output>                   Returns a reference to an element or subslice depending on the type of index.
fn get_mut<I>(&mut self, index: I) -> Option<&mut <I as SliceIndex<[T]>>::Output>       Mutable version of get().
fn swap(&mut self, a: usize, b: usize)          Swaps two elements in the slice.
fn reverse(&mut self)                           Reverses the order of elements in the slice, in place.
slice[i]                                        Indexation.

Iterating
=========
fn iter(&self) -> Iter<T>                       Returns an iterator over the slice. "for n in numbers"
fn iter_mut(&mut self) -> IterMut<T>            Returns an iterator that allows modifying each value. "for n in &mut numbers"
fn windows(&self, size: usize) -> Windows<T>    Returns an iterator over all contiguous windows of length size. The windows overlap.
fn chunks(&self, size: usize) -> Chunks<T>      Returns an iterator over size elements of the slice at a time. The chunks are slices and do not overlap.
fn chunks_mut(&mut self, chunk_size: usize) -> ChunksMut<T>     Mutable version of chunks().
fn into_iter(self) -> Iter<'a, T>               Creates an iterator from a value.
fn into_iter(self) -> IterMut<'a, T>            Creates an iterator from a value.

Aggregating
===========
fn concat(&self) -> Self::Output                Flattens a slice of T into a single value Self::Output.
fn join(&self, sep: &T) -> Self::Output         Flattens a slice of T into a single value Self::Output, placing a given separator between each.

Sorting
=======
fn sort(&mut self)                              Sorts the slice.
fn sort_by<F>(&mut self, compare: F)            Sorts the slice with a comparator function.
fn sort_by_key<B, F>(&mut self, f: F)           Sorts the slice with a key extraction function.
fn sort_unstable(&mut self)                     Sorts the slice, but may not preserve the order of equal elements.
fn sort_unstable_by<F>(&mut self, compare: F)   Sorts the slice with a comparator function, but may not preserve the order of equal elements.
fn sort_unstable_by_key<B, F>(&mut self, f: F)  Sorts the slice with a key extraction function, but may not preserve the order of equal elements.

Splitting
=========
fn split_first(&self) -> Option<(&T, &[T])>                             Returns the first and all the rest of the elements of the slice, or None if it is empty.
fn split_first_mut(&mut self) -> Option<(&mut T, &mut [T])>             Returns the first and all the rest of the elements of the slice, or None if it is empty.
fn split_last(&self) -> Option<(&T, &[T])>                              Returns the last and all the rest of the elements of the slice, or None if it is empty.
fn split_last_mut(&mut self) -> Option<(&mut T, &mut [T])>              Returns the last and all the rest of the elements of the slice, or None if it is empty.
fn split_at(&self, mid: usize) -> (&[T], &[T])                          Divides one slice into two at an index.
fn split_at_mut(&mut self, mid: usize) -> (&mut [T], &mut [T])          Divides one &mut into two at an index.
fn split<F>(&self, pred: F) -> Split<T, F>                              Returns an iterator over subslices separated by elements that match pred. The matched element is not contained in the subslices.
fn split_mut<F>(&mut self, pred: F) -> SplitMut<T, F>                   Mutable version of split().
fn rsplit<F>(&self, pred: F) -> RSplit<T, F>                            Like split<F>, but scan backwards from the end.
fn rsplit_mut<F>(&mut self, pred: F) -> RSplitMut<T, F>                 Mutable version of rsplit<F>().
fn splitn<F>(&self, n: usize, pred: F) -> SplitN<T, F>                  Returns an iterator over subslices separated by elements that match pred, limited to returning at most n items. The matched element is not contained in the subslices.
fn splitn_mut<F>(&mut self, n: usize, pred: F) -> SplitNMut<T, F>       Mutable version of splitn<F>.
fn rsplitn<F>(&self, n: usize, pred: F) -> RSplitN<T, F>                Like splitn<F>, but scan backwards from the end.
fn rsplitn_mut<F>(&mut self, n: usize, pred: F) -> RSplitNMut<T, F>     Mutable version of rsplitn<F>.
*/


/*
Vector methods (+Slice) - https://doc.rust-lang.org/std/vec/struct.Vec.html
========================================================================================================================================================================================================
A contiguous growable array type, written Vec<T> but pronounced 'vector'.
In Rust, it's more common to pass slices as arguments rather than vectors when you just want to provide a
read access. The same goes for String and &str.

Construction & Conversion
=========================
let mut vec = Vec::new();                       Construct a new empty vector.
let mut vec = vec![1, 2, 3];                    Construct a new vector using a macro.
fn new() -> Vec<T>                              Constructs a new, empty Vec<T>.
fn with_capacity(capacity: usize) -> Vec<T>     Constructs a new, empty Vec<T> with the specified capacity.
fn reserve(&mut self, additional: usize)        Reserves capacity for at least additional more elements to be inserted.
fn reserve_exact(&mut self, additional: usize)  Reserves the minimum capacity for exactly additional more elements to be inserted.
fn shrink_to_fit(&mut self)                     Shrinks the capacity of the vector as much as possible.
fn into_boxed_slice(self) -> Box<[T]>           Converts the vector into Box<[T]>.
fn as_slice(&self) -> &[T]                      Extracts a slice containing the entire vector. Equivalent to &s[..].
fn as_mut_slice(&mut self) -> &mut [T]          Extracts a mutable slice of the entire vector. Equivalent to &mut s[..].

Queries
=======
fn is_empty(&self) -> bool
fn len(&self) -> usize                          Returns the number of elements in the vector, also referred to as its 'length'.
fn capacity(&self) -> usize                     Returns the number of elements the vector can hold without reallocating.

Adding items
============
fn push(&mut self, value: T)                    Appends an element to the end of the vector.
fn insert(&mut self, index: usize, element: T)  Inserts an element at position index within the vector, shifting all elements after it to the right.
fn resize(&mut self, new_len: usize, value: T)  Resizes the Vec in-place so that len is equal to new_len.
fn resize_default(&mut self, new_len: usize)    Resizes the Vec in-place so that len is equal to new_len. Fills with the default value of T.
fn extend_from_slice(&mut self, other: &[T])    Clones and appends all elements in a slice to the vector.
fn append(&mut self, other: &mut Vec<T>)        Moves all the elements of other into Self, leaving other empty.

Removing items
==============
fn clear(&mut self)                             Clears the vector, removing all values.
fn pop(&mut self) -> Option<T>                  Removes the last element from a vector and returns it, or None if it is empty.
fn remove(&mut self, index: usize) -> T         Removes and returns the element at position index within the vector, shifting all elements after it to the left.
fn remove_item(&mut self, item: &T) -> Option<T>    Removes the first instance of item from the vector if the item exists.
fn truncate(&mut self, len: usize)              Shortens the vector, keeping the first len elements and dropping the rest.
fn swap_remove(&mut self, index: usize) -> T    Removes an element from the vector and returns it. The removed element is replaced by the last element of the vector.
fn retain<F>(&mut self, f: F)                   Retains only the elements specified by the predicate.
fn dedup(&mut self)                             Removes consecutive repeated elements in the vector.
fn dedup_by_key<F, K>(&mut self, key: F)        Removes all but the first of consecutive elements in the vector that resolve to the same key.
fn dedup_by<F>(&mut self, same_bucket: F)       Removes all but the first of consecutive elements in the vector satisfying a given equality relation.
fn drain<R>(&mut self, range: R) -> Drain<T>    Creates a draining iterator that removes the specified range in the vector and yields the removed items.

Splitting
=========
fn split_off(&mut self, at: usize) -> Vec<T>    Splits the collection into two at the given index.
*/



/*
Iterator methods - https://doc.rust-lang.org/std/iter/trait.Iterator.html
========================================================================================================================================================================================================
Immutable slice iterator. This struct is created by the iter() method on slices.

See also: https://doc.rust-lang.org/std/iter/trait.IntoIterator.html
    "By implementing IntoIterator for a type, you define how it will be converted to an iterator. This is common
    for types which describe a collection of some kind. One benefit of implementing IntoIterator is that your type
    will work with Rust's for loop syntax."

Construction & Conversion
=========================
Call iter() on a slice or other collection type.
fn clone(&self) -> Iter<'a, T>                      Returns a copy of the value.
fn clone_from(&mut self, source: &Self)             Performs copy-assignment from source. Read more
fn as_slice(&self) -> &'a [T]                       View the underlying data as a subslice of the original data.
fn cloned<'a, T>(self) -> Cloned<Self>              Creates an iterator which clones all of its elements.
fn by_ref(&mut self) -> &mut Self                   Borrows an iterator, rather than consuming it. This is useful to allow applying iterator adaptors while still retaining ownership of the original iterator.

Queries
=======
fn is_empty(&self) -> bool                          Returns whether the iterator is empty.
fn len(&self) -> usize                              Returns the exact number of times the iterator will iterate.
fn size_hint(&self) -> (usize, Option<usize>)       Returns the bounds on the remaining length of the iterator.
fn peekable(self) -> Peekable<Self>                 Creates an iterator which can use peek to look at the next element of the iterator without consuming it.
fn all<F>(&mut self, f: F) -> bool                  Tests if every element of the iterator matches a predicate.
fn any<F>(&mut self, f: F) -> bool                  Tests if any element of the iterator matches a predicate.

Iterating
=========
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

Mapping & Zipping
=================
fn for_each<F>(self, f: F)                                                  Calls a closure on each element of an iterator.
fn map<B, F>(self, f: F) -> Map<Self, F>                                    Takes a closure and creates an iterator which calls that closure on each element.
fn flat_map<U, F>(self, f: F) -> FlatMap<Self, U, F>                        Creates an iterator that works like map, but flattens nested structure.
fn zip<U>(self, other: U) -> Zip<Self, <U as IntoIterator>::IntoIter>       'Zips up' two iterators into a single iterator of pairs.
fn unzip<A, B, FromA, FromB>(self) -> (FromA, FromB)                        Converts an iterator of pairs into a pair of containers. The opposite of zip().

Filtering
=========
fn filter<P>(self, predicate: P) -> Filter<Self, P>         Creates an iterator which uses a closure to determine if an element should be yielded.
fn filter_map<B, F>(self, f: F) -> FilterMap<Self, F>       Creates an iterator that both filters and maps. The closure must return an Option<T>.
fn skip(self, n: usize) -> Skip<Self>                       Creates an iterator that skips the first n elements.
fn take(self, n: usize) -> Take<Self>                       Creates an iterator that yields its first n elements.
fn skip_while<P>(self, predicate: P) -> SkipWhile<Self, P>  Creates an iterator that skips elements based on a predicate.
fn take_while<P>(self, predicate: P) -> TakeWhile<Self, P>  Creates an iterator that yields elements based on a predicate.
fn partition<B, F>(self, f: F) -> (B, B)                    Consumes an iterator, creating two collections from it which are disjoint by the predicate.

Finding
=======
fn find<P>(&mut self, predicate: P) -> Option<Self::Item>   Searches for an element of an iterator that satisfies a predicate.
fn position<P>(&mut self, predicate: P) -> Option<usize>    Searches for an element in an iterator, returning its index.
fn rposition<P>(&mut self, predicate: P) -> Option<usize>   Version of position<P> that scans backwards from the end.

Reducing
========
fn collect<B>(self) -> B                                    Transforms an iterator into a collection.
fn fold<B, F>(self, init: B, f: F) -> B                     An iterator adaptor that applies a function, producing a single, final value.
fn max(self) -> Option<Self::Item>                          Returns the maximum element of an iterator.
fn min(self) -> Option<Self::Item>                          Returns the minimum element of an iterator.
fn max_by_key<B, F>(self, f: F) -> Option<Self::Item>       Returns the element that gives the maximum value from the specified function.
fn max_by<F>(self, compare: F) -> Option<Self::Item>        Returns the element that gives the maximum value with respect to the specified comparison function.
fn min_by_key<B, F>(self, f: F) -> Option<Self::Item>       Returns the element that gives the minimum value from the specified function.
fn min_by<F>(self, compare: F) -> Option<Self::Item>        Returns the element that gives the minimum value with respect to the specified comparison function.
fn sum<S>(self) -> S                                        Sums the elements of an iterator.
fn product<P>(self) -> P                                    Iterates over the entire iterator, multiplying all the elements
fn scan<St, B, F>(self, initial_state: St, f: F) -> Scan<Self, St, F>   An iterator adaptor similar to fold that holds internal state and produces a new iterator.

Free Functions
==============
pub fn once<T>(value: T) -> Once<T>         Creates an iterator that yields an element exactly once. This is commonly used to adapt a single value into a chain of other kinds of iteration.
pub fn repeat<T>(elt: T) -> Repeat<T>       Creates a new iterator that endlessly repeats a single element.
pub fn empty<T>() -> Empty<T>               Creates an iterator that yields nothing.
*/





/*
String methods
====================================================================================================
*/



/*
Hashtable methods
====================================================================================================
*/

/*
Option methods
====================================================================================================
*/

/*
Result methods
====================================================================================================
*/


