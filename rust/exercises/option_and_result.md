# Option - https://doc.rust-lang.org/std/option/enum.Option.html

### Conversions
fn as_ref(&self) -> Option<&T>	                        Converts from Option<T> to Option<&T>.
fn as_mut(&mut self) -> Option<&mut T>	                Converts from Option<T> to Option<&mut T>.
fn ok_or<E>(self, err: E) -> Result<T, E>	            Transforms the Option<T> into a Result<T, E>, mapping Some(v) to Ok(v) and None to Err(err).
fn ok_or_else<E, F>(self, err: F) -> Result<T, E>	    Transforms the Option<T> into a Result<T, E>, mapping Some(v) to Ok(v) and None to Err(err()).

### Queries
fn is_some(&self) -> bool               	            Returns true if the option is a Some value.
fn is_none(&self) -> bool	                            Returns true if the option is a None value.

### Functions for getting at the value
fn expect(self, msg: &str) -> T	                        Unwraps an option, yielding the content of a Some. Panics if the value is a None with a custom panic message provided by msg.
fn unwrap(self) -> T	                                Moves the value v out of the Option<T> if it is Some(v). Panics if the self value equals None.
fn unwrap_or(self, def: T) -> T	                        Returns the contained value or a default.
fn unwrap_or_default(self) -> T	                        Returns the contained value or a default
fn unwrap_or_else<F>(self, f: F) -> T	                Returns the contained value or computes it from a closure.
fn map_or<U, F>(self, default: U, f: F) -> U	        Applies a function to the contained value (if any), or returns a default (if not).
fn map_or_else<U, D, F>(self, default: D, f: F) -> U	Applies a function to the contained value (if any), or computes a default (if not).

### Functions that return another Option<T>, good for chaining operations
fn map<U, F>(self, f: F) -> Option<U>	                Maps an Option<T> to Option<U> by applying a function to a contained value. Consumes the original value (see as_ref for a way around this) and always applies a new wrapping.
fn and<U>(self, optb: Option<U>) -> Option<U>	        Returns None if the option is None, otherwise returns optb.
fn and_then<U, F>(self, f: F) -> Option<U>	            Returns None if the option is None, otherwise calls f with the wrapped value and returns the result.
fn or(self, optb: Option<T>) -> Option<T>	            Returns the option if it contains a value, otherwise returns optb.
fn or_else<F>(self, f: F) -> Option<T>	                Returns the option if it contains a value, otherwise calls f and returns the result.
fn take(&mut self) -> Option<T>	                        Takes the value out of the option, leaving a None in its place.

### Iteration
fn iter(&self) -> Iter<T>	                            Returns an iterator over the possibly contained value.
fn iter_mut(&mut self) -> IterMut<T>	                Returns a mutable iterator over the possibly contained value.



# Result - https://doc.rust-lang.org/std/result/enum.Result.html

### Conversions
fn as_ref(&self) -> Result<&T, &E>	                    Converts from Result<T, E> to Result<&T, &E>.
fn as_mut(&mut self) -> Result<&mut T, &mut E>	        Converts from Result<T, E> to Result<&mut T, &mut E>.
fn ok(self) -> Option<T>	                            Converts self into an Option<T>, consuming self, and discarding the error, if any.
fn err(self) -> Option<E>	                            Converts self into an Option<E>, consuming self, and discarding the success value, if any.
fn clone(&self) -> Result<T, E>                         Clones the Result<T>.

### Queries
fn is_ok(&self) -> bool	                                Returns true if the result is Ok.
fn is_err(&self) -> bool	                            Returns true if the result is Err.

### Functions for getting at the value
fn expect(self, msg: &str) -> T	                        Unwraps a result, yielding the content of an Ok. Panics if the value is an Err, with a panic message including the passed message, and the content of the Err.
fn expect_err(self, msg: &str) -> E	                    Unwraps a result, yielding the content of an Err. Panics if the value is an Ok, with a panic message including the passed message, and the content of the Ok.
fn unwrap(self) -> T	                                Unwraps a result, yielding the content of an Ok. Panics if the value is an Err, with a panic message provided by the Err's value.
fn unwrap_err(self) -> E	                            Unwraps a result, yielding the content of an Err. Panics if the value is an Ok, with a custom panic message provided by the Ok's value.
fn unwrap_or(self, optb: T) -> T	                    Unwraps a result, yielding the content of an Ok. Else, it returns optb.
fn unwrap_or_default(self) -> T	                        Returns the contained value or a default.
fn unwrap_or_else<F>(self, op: F) -> T	                Unwraps a result, yielding the content of an Ok. If the value is an Err then it calls op with its value.

### Functions that return another Result<T>, good for chaining operations
fn map<U, F>(self, op: F) -> Result<U, E>	            Maps a Result<T, E> to Result<U, E> by applying a function to a contained Ok value, leaving an Err value untouched.
fn map_err<F, O>(self, op: O) -> Result<T, F>	        Maps a Result<T, E> to Result<T, F> by applying a function to a contained Err value, leaving an Ok value untouched.
fn and<U>(self, res: Result<U, E>) -> Result<U, E>	    Returns res if the result is Ok, otherwise returns the Err value of self.
fn and_then<U, F>(self, op: F) -> Result<U, E>	        Calls op if the result is Ok, otherwise returns the Err value of self.
fn or<F>(self, res: Result<T, F>) -> Result<T, F>	    Returns res if the result is Err, otherwise returns the Ok value of self.
fn or_else<F, O>(self, op: O) -> Result<T, F>	        Calls op if the result is Err, otherwise returns the Ok value of self.

### Iteration
fn iter(&self) -> Iter<T>	                            Returns an iterator over the possibly contained value.
fn iter_mut(&mut self) -> IterMut<T>	                Returns a mutable iterator over the possibly contained value.
fn sum<I>(iter: I) -> Result<T, E>                      Takes each element in the Iterator: if it is an Err, no further elements are taken, and the Err is returned. Should no Err occur, the sum of all elements is returned.
