# Slice methods - https://doc.rust-lang.org/std/primitive.slice.html
Slices are either mutable or shared. The shared slice type is &[T], while the mutable slice type is &mut [T]

### Construction
fn clone_from_slice(&mut self, src: &[T])       Copies the elements from src into self. The length of src must be the same as self.
fn copy_from_slice(&mut self, src: &[T])        Copies all elements from src into self, using a memcpy. The length of src must be the same as self.
fn to_vec(&self) -> Vec<T>                      Copies self into a new Vec.
fn into_vec(self: Box<[T]>) -> Vec<T>           Converts self into a vector without clones or allocation.
fn to_owned(&self) -> Vec<T>                    Creates owned data from borrowed data, usually by cloning.

### Queries
fn len(&self) -> usize                          Returns the number of elements in the slice.
fn is_empty(&self) -> bool                      Returns true if the slice contains no elements.
fn contains(&self, x: &T) -> bool               Returns true if the slice contains an element with the given value.
fn starts_with(&self, needle: &[T]) -> bool     Returns true if needle is a prefix of the slice.
fn ends_with(&self, needle: &[T]) -> bool       Returns true if needle is a suffix of the slice.
fn binary_search(&self, x: &T) -> Result<usize, usize>                              Binary searches this sorted slice for a given element.
fn binary_search_by<'a, F>(&'a self, f: F) -> Result<usize, usize>                  Binary searches this sorted slice with a comparator function.
fn binary_search_by_key<'a, B, F>(&'a self, b: &B, f: F) -> Result<usize, usize>    Binary searches this sorted slice with a key extraction function.

### Element Access & Change
fn first(&self) -> Option<&T>                   Returns the first element of the slice, or None if it is empty.
fn first_mut(&mut self) -> Option<&mut T>       Returns a mutable pointer to the first element of the slice, or None if it is empty.
fn last(&self) -> Option<&T>                    Returns the last element of the slice, or None if it is empty.
fn last_mut(&mut self) -> Option<&mut T>        Returns a mutable pointer to the last item in the slice.
fn get<I>(&self, index: I) -> Option<&<I as SliceIndex<[T]>>::Output>                   Returns a reference to an element or subslice depending on the type of index.
fn get_mut<I>(&mut self, index: I) -> Option<&mut <I as SliceIndex<[T]>>::Output>       Mutable version of get().
fn swap(&mut self, a: usize, b: usize)          Swaps two elements in the slice.
fn reverse(&mut self)                           Reverses the order of elements in the slice, in place.
slice[i]                                        Indexation.

### Iterating
fn iter(&self) -> Iter<T>                       Returns an iterator over the slice. "for n in numbers"
fn iter_mut(&mut self) -> IterMut<T>            Returns an iterator that allows modifying each value. "for n in &mut numbers"
fn windows(&self, size: usize) -> Windows<T>    Returns an iterator over all contiguous windows of length size. The windows overlap.
fn chunks(&self, size: usize) -> Chunks<T>      Returns an iterator over size elements of the slice at a time. The chunks are slices and do not overlap.
fn chunks_mut(&mut self, chunk_size: usize) -> ChunksMut<T>     Mutable version of chunks().
fn into_iter(self) -> Iter<'a, T>               Creates an iterator from a value.
fn into_iter(self) -> IterMut<'a, T>            Creates an iterator from a value.

### Aggregating
fn concat(&self) -> Self::Output                Flattens a slice of T into a single value Self::Output.
fn join(&self, sep: &T) -> Self::Output         Flattens a slice of T into a single value Self::Output, placing a given separator between each.

### Sorting
fn sort(&mut self)                              Sorts the slice.
fn sort_by<F>(&mut self, compare: F)            Sorts the slice with a comparator function.
fn sort_by_key<B, F>(&mut self, f: F)           Sorts the slice with a key extraction function.
fn sort_unstable(&mut self)                     Sorts the slice, but may not preserve the order of equal elements.
fn sort_unstable_by<F>(&mut self, compare: F)   Sorts the slice with a comparator function, but may not preserve the order of equal elements.
fn sort_unstable_by_key<B, F>(&mut self, f: F)  Sorts the slice with a key extraction function, but may not preserve the order of equal elements.

### Splitting
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





# Vector methods (+Slice) - https://doc.rust-lang.org/std/vec/struct.Vec.html
A contiguous growable array type, written Vec<T> but pronounced 'vector'.
In Rust, it's more common to pass slices as arguments rather than vectors when you just want to provide a
read access. The same goes for String and &str.

### Construction & Conversion
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

### Queries
fn is_empty(&self) -> bool
fn len(&self) -> usize                          Returns the number of elements in the vector, also referred to as its 'length'.
fn capacity(&self) -> usize                     Returns the number of elements the vector can hold without reallocating.

### Adding items
fn push(&mut self, value: T)                    Appends an element to the end of the vector.
fn insert(&mut self, index: usize, element: T)  Inserts an element at position index within the vector, shifting all elements after it to the right.
fn resize(&mut self, new_len: usize, value: T)  Resizes the Vec in-place so that len is equal to new_len.
fn resize_default(&mut self, new_len: usize)    Resizes the Vec in-place so that len is equal to new_len. Fills with the default value of T.
fn extend_from_slice(&mut self, other: &[T])    Clones and appends all elements in a slice to the vector.
fn append(&mut self, other: &mut Vec<T>)        Moves all the elements of other into Self, leaving other empty.

### Removing items
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

### Splitting
fn split_off(&mut self, at: usize) -> Vec<T>    Splits the collection into two at the given index.
