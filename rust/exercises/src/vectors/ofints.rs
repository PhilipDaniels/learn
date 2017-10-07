// print_vec takes an immutable_slice. The elements in the slice are not changeable.
fn print_vec(slice: &[i32]) {
    // Won't compile.
    // slice[0] = 20;

    for (idx, elem) in slice.iter().enumerate() {
        println!("vec[{}] = {}", idx, elem);
    }
}

// We can also get a mutable slice, where we can change the elements.
fn pass_mutable_slice(slice: &mut [i32]) {
    slice[0] *= 100;
}

pub fn run() {
    println!("********* vec<i32> examples *********");

    // Create using ctor and type inference.
    let mut vec = Vec::new();
    vec.push(1);

    // Create using a macro.
    let mut vec = vec![10, 20, 30, 40, 50];

    // If a function does not need to add, remove or change elements, then pass an immutable slice.
    println!("Slicing the entire vector...");
    print_vec(&vec);
    println!("Slicing the first four elements...");
    print_vec(&vec[0..4]);

    vec.truncate(2);
    println!("Truncating to length of 2...");
    print_vec(&vec);

    // These two calls are equivalent.
    pass_mutable_slice(&mut vec);
    pass_mutable_slice(vec.as_mut_slice());

    println!("After mutating a slice...");
    print_vec(&vec);

    // Summary
    //     &vec is equivalent to vec.as_slice() and yields an immutable slice
    //     &mut vec is equivalent to vec.as_mut_slice() and yields a mutable slice.

    vec.insert(0, 50);              // Insert an element at index = 0.
    let elem = vec.remove(1);       // Remove the element at index = 1.
    println!("After insert and remove...");
    print_vec(&vec);

    // Reinitialize.
    let mut vec = vec![10, 20, 30, 40, 50];

    println!("After retaining only elements matching a predicate...");
    vec.retain(|&elem| elem == 20 || elem == 40);
    print_vec(&vec);

    // Try that again, using a lambda for the predicate.
    // The two predicates are the same. Not sure why.
    let mut vec = vec![10, 20, 30, 40, 50];
    let pred = |elem: &i32| *elem == 20 || *elem == 40;
    let pred = |&elem: &i32| elem == 20 || elem == 40;
    vec.retain(&pred);
    println!("After retaining only elements matching a predicate (lambda)...");
    print_vec(&vec);

    // Negating the predicate, not sure if this is the best way.
    vec.retain(|&elem| !pred(&elem));

    // Add an element to the end.
    vec.push(555);

    // Remove the last element. Returns Option<T>: None if the vec is empty.
    let elem = vec.pop().unwrap();

    // We can append a vec, but it does a 'move' of all the elements from the vec2 we are appending.
    let mut vec = vec![10, 20, 30, 40, 50];
    println!("vec before append of vec2...");
    print_vec(&vec);

    let mut vec2 = vec![-10, -20, -30];
    vec.append(&mut vec2);

    println!("vec after append of vec2...");
    print_vec(&vec);

    println!("vec2 after append to vec (it's empty)...");
    print_vec(&vec2);
    println!("See!");

    // We have a lot of methods from come from slices, because vectors implement "Deref<Target = [T]>"
    vec.reverse();
    vec.reverse();

    // drain allows us to remove slices of vectors.
    let removed: Vec<_> = vec.drain(2..5).collect();
    println!("vec after draining 2..5 ...");
    print_vec(&vec);
    println!("The items drained...");
    print_vec(&removed);

    // Clear is easy enough.
    vec.clear();

    // split_off allows us to split a vector into two. Half-empty range, as usual.
    let mut vec = vec![10, 20, 30, 40, 50];
    let vec2 = vec.split_off(2);
    assert_eq!(vec, [10, 20]);
    assert_eq!(vec2, [30, 40, 50]);

    // Change length to 5, fill with value of 4.
    vec.resize(5, 4);
    println!("vec after resizing and filling with a value of 4 ...");
    print_vec(&vec);

    vec.extend_from_slice(&[100, 200]);
    println!("vec after extend_from_slice ...");
    print_vec(&vec);


    let mut vec = vec![1, 2, 2, 3, 2];
    println!("vec before dedup ...");
    print_vec(&vec);
    vec.dedup();
    println!("vec after dedup (note there is dedup_by_key as well)...");
    print_vec(&vec);
}



/* Method summary

Construction
============
let mut vec = Vec::new();
let mut vec = vec![1, 2, 3];

fn new() -> Vec<T>                              Constructs a new, empty Vec<T>.
fn with_capacity(capacity: usize) -> Vec<T>     Constructs a new, empty Vec<T> with the specified capacity.
fn reserve(&mut self, additional: usize)        Reserves capacity for at least additional more elements to be inserted.
fn reserve_exact(&mut self, additional: usize)  Reserves the minimum capacity for exactly additional more elements to be inserted.
fn shrink_to_fit(&mut self)                     Shrinks the capacity of the vector as much as possible.
fn clone_from_slice(&mut self, src: &[T])       Copies the elements from src into self. The length of src must be the same as self.
fn copy_from_slice(&mut self, src: &[T])        Copies all elements from src into self, using a memcpy. The length of src must be the same as self.
fn to_vec(&self) -> Vec<T>                      Copies self into a new Vec.

Queries
=======
fn len(&self) -> usize                          Returns the number of elements in the slice.
fn is_empty(&self) -> bool                      Returns true if the slice contains no elements.
fn capacity(&self) -> usize                     Returns the number of elements the vector can hold without reallocating.
fn contains(&self, x: &T) -> bool               Returns true if the slice contains an element with the given value.
fn starts_with(&self, needle: &[T]) -> bool     Returns true if needle is a prefix of the slice.
fn ends_with(&self, needle: &[T]) -> bool       Returns true if needle is a suffix of the slice.
fn binary_search(&self, x: &T) -> Result<usize, usize>                              Binary searches this sorted slice for a given element.
fn binary_search_by<'a, F>(&'a self, f: F) -> Result<usize, usize>                  Binary searches this sorted slice with a comparator function.
fn binary_search_by_key<'a, B, F>(&'a self, b: &B, f: F) -> Result<usize, usize>    Binary searches this sorted slice with a key extraction function.

Conversions
===========
fn as_slice(&self) -> &[T]                      Extracts a slice containing the entire vector. Equivalent to &s[..].
fn as_mut_slice(&mut self) -> &mut [T]          Extracts a mutable slice of the entire vector. Equivalent to &mut s[..].

Adding items
============
fn push(&mut self, value: T)                    Appends an element to the end of the vector.
fn insert(&mut self, index: usize, element: T)  Inserts an element at position index within the vector, shifting all elements after it to the right.
fn append(&mut self, other: &mut Vec<T>)        Moves all the elements of other into Self, leaving other empty.
fn resize(&mut self, new_len: usize, value: T)  Resizes the Vec in-place so that len is equal to new_len.
fn resize_default(&mut self, new_len: usize)    Resizes the Vec in-place so that len is equal to new_len. Fills with the default value of T.
fn extend_from_slice(&mut self, other: &[T])    Clones and appends all elements in a slice to the vector.

Iterating
=========
fn iter(&self) -> Iter<T>                       Returns an iterator over the slice.
fn iter_mut(&mut self) -> IterMut<T>            Returns an iterator that allows modifying each value.
fn windows(&self, size: usize) -> Windows<T>    Returns an iterator over all contiguous windows of length size. The windows overlap.
fn chunks(&self, size: usize) -> Chunks<T>      Returns an iterator over size elements of the slice at a time. The chunks are slices and do not overlap.
fn chunks_mut(&mut self, chunk_size: usize) -> ChunksMut<T>     Mutable version of chunks().

Removing items
==============
fn clear(&mut self)                             Clears the vector, removing all values.
fn pop(&mut self) -> Option<T>                  Removes the last element from a vector and returns it, or None if it is empty.
fn truncate(&mut self, len: usize)              Shortens the vector, keeping the first len elements and dropping the rest.
fn remove(&mut self, index: usize) -> T         Removes and returns the element at position index within the vector, shifting all elements after it to the left.
fn remove_item(&mut self, item: &T) -> Option<T>    Removes the first instance of item from the vector if the item exists.
fn retain<F>(&mut self, f: F)                   Retains only the elements specified by the predicate.
fn drain<R>(&mut self, range: R) -> Drain<T>    Creates a draining iterator that removes the specified range in the vector and yields the removed items.
fn dedup(&mut self)                             Removes consecutive repeated elements in the vector.
fn dedup_by_key<F, K>(&mut self, key: F)        Removes all but the first of consecutive elements in the vector that resolve to the same key.
fn dedup_by<F>(&mut self, same_bucket: F)       Removes all but the first of consecutive elements in the vector satisfying a given equality relation.

Sorting
=======
fn sort(&mut self)                              Sorts the slice.
fn sort_by<F>(&mut self, compare: F)            Sorts the slice with a comparator function.
fn sort_by_key<B, F>(&mut self, f: F)           Sorts the slice with a key extraction function.
fn sort_unstable(&mut self)                     Sorts the slice, but may not preserve the order of equal elements.
fn sort_unstable_by<F>(&mut self, compare: F)   Sorts the slice with a comparator function, but may not preserve the order of equal elements.
fn sort_unstable_by_key<B, F>(&mut self, f: F)  Sorts the slice with a key extraction function, but may not preserve the order of equal elements.

Getting/Changing
================
fn get<I>(&self, index: I) -> Option<&<I as SliceIndex<[T]>>::Output>                   Returns a reference to an element or subslice depending on the type of index.
fn get_mut<I>(&mut self, index: I) -> Option<&mut <I as SliceIndex<[T]>>::Output>       mutable version of get().
fn reverse(&mut self)                           Reverses the order of elements in the slice, in place.
fn swap(&mut self, a: usize, b: usize)          Swaps two elements in the slice.
fn split_off(&mut self, at: usize) -> Vec<T>    Splits the collection into two at the given index.
fn swap_remove(&mut self, index: usize) -> T    Removes an element from the vector and returns it. The removed element is replaced by the last element of the vector.

Head/Tail/Split
===============
fn first(&self) -> Option<&T>                   Returns the first element of the slice, or None if it is empty.
fn first_mut(&mut self) -> Option<&mut T>       Returns a mutable pointer to the first element of the slice, or None if it is empty.
fn last(&self) -> Option<&T>                    Returns the last element of the slice, or None if it is empty.
fn last_mut(&mut self) -> Option<&mut T>        Returns a mutable pointer to the last item in the slice.

fn split<F>(&self, pred: F) -> Split<T, F>                      Returns an iterator over subslices separated by elements that match pred. The matched element is not contained in the subslices.
fn split_mut<F>(&mut self, pred: F) -> SplitMut<T, F>           Mutable version of split().
fn rsplit<F>(&self, pred: F) -> RSplit<T, F>                    Like split, but scan backwards from the end.
fn rsplit_mut<F>(&mut self, pred: F) -> RSplitMut<T, F>         Mutable version of rsplit().
fn split_first(&self) -> Option<(&T, &[T])>                     Returns the first and all the rest of the elements of the slice, or None if it is empty.
fn split_first_mut(&mut self) -> Option<(&mut T, &mut [T])>     Returns the first and all the rest of the elements of the slice, or None if it is empty.
fn split_last(&self) -> Option<(&T, &[T])>                      Returns the last and all the rest of the elements of the slice, or None if it is empty.
fn split_last_mut(&mut self) -> Option<(&mut T, &mut [T])>      Returns the last and all the rest of the elements of the slice, or None if it is empty.

fn splitn<F>(&self, n: usize, pred: F) -> SplitN<T, F>
fn splitn_mut<F>(&mut self, n: usize, pred: F) -> SplitNMut<T, F>
fn rsplitn<F>(&self, n: usize, pred: F) -> RSplitN<T, F>
fn rsplitn_mut<F>(&mut self, n: usize, pred: F) -> RSplitNMut<T, F>

*/
