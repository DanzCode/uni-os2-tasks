#![allow(dead_code, unused_variables)]

/// A contiguous array type backed by a slice.
struct StackVec<'a, T:'a> {
	storage: &'a mut [T],
	len: usize
}

impl<'a,T> StackVec<'a,T> {
	/// Constructs a new, empty `StackVec<T>` using `storage` as the backing
	/// store. The returned `StackVec` will be able to hold `storage.len()`
	/// values.
	fn new(storage: &'a mut [T]) -> Self {
		unimplemented!()
	}
	
	/// Appends a new element to this vector. Fails with `Err` if capacity
	/// would be exceeded.
	fn push(&mut self, e: T) -> Result<(),()> {
		unimplemented!()
	}
	
	/// Returns the number of elements this vector can hold.
	fn capacity(&self) -> usize {
		unimplemented!()
	}
	
	/// Returns the number of elements in the vector.
	fn len(&self) -> usize {
		unimplemented!()
	}
	
	/// Returns true if the vector contains no elements.
	fn is_empty(&self) -> bool {
		unimplemented!()
	}
	
	/// Extracts a slice containing the entire vector.
	fn as_slice(&self) -> &[T] {
		unimplemented!()
	}
}

impl<'a,T:Clone> StackVec<'a,T> {
	/// If this vector is not empty, removes the last element from this vector
    /// by cloning it and returns it. Otherwise returns `None`.
	fn pop(&mut self) -> Option<T> {
		unimplemented!()
	}
}

fn main() {
	// test program from the exercise 
	let mut storage = [0u8; 1024];
	let mut vec = StackVec::new(&mut storage);
	
	for i in 0..10 {
		vec.push(i * i).unwrap();
	}
	
	for e in vec.as_slice() {
		println!("{}", *e);
	}
	
	let last_element = vec.pop().unwrap();
	assert_eq!(last_element, 9 * 9);
}
