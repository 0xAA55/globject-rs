
use crate::prelude::*;
use bitvec::vec::BitVec;
use std::{
	fmt::Debug,
	marker::PhantomData,
	mem::{size_of, size_of_val},
	ops::{Deref, DerefMut, Index, IndexMut, Range, RangeFrom, RangeTo, RangeFull, RangeInclusive, RangeToInclusive},
	rc::Rc,
};

/// The type that could be the item of the `BufferVec`
pub trait BufferVecItem: Copy + Sized + Default + Debug {}
impl<T> BufferVecItem for T where T: Copy + Sized + Default + Debug {}

/// The `BufferVec` trait
pub trait BufferVec<T: BufferVecItem>: Debug + Clone + From<Buffer> {
	/// Get the underlying `Buffer`
	fn get_buffer(&self) -> &Buffer;

	/// Get the underlying `Buffer` as mut
	fn get_buffer_mut(&mut self) -> &mut Buffer;

	/// Get the default binding target of the buffer
	fn get_target(&self) -> BufferTarget;

	/// Set the binding target of the buffer
	fn set_target(&mut self, target: BufferTarget);

	/// Get the size of the buffer
	fn len(&self) -> usize;

	/// Get the capacity of the buffer
	fn capacity(&self) -> usize;

	/// Resizes to the new size, reallocate the buffer if the new size is larger
	fn resize(&mut self, new_len: usize, value: T) -> Result<(), GLCoreError>;

	/// Shrink to the exact number of items
	fn shrink_to_fit(&mut self) -> Result<(), GLCoreError>;

	/// Retrieve a single item from the buffer in the GPU
	fn get(&self, index: usize) -> Result<T, GLCoreError>;

	/// Update a single item from the buffer in the GPU
	fn set(&mut self, index: usize, data: &T) -> Result<(), GLCoreError>;

	/// Retrieve a slice of items from the buffer in the GPU
	fn get_slice_of_data(&self, start_index: usize, len: usize) -> Result<Vec<T>, GLCoreError>;

	/// Update a slice of itrems to the buffer in the GPU
	fn set_slice_of_data(&mut self, start_index: usize, data: &[T]) -> Result<(), GLCoreError>;

	/// Flush the buffer to the GPU if it has a cache in the system memory
	fn flush(&mut self) -> Result<(), GLCoreError> {Ok(())}

	/// Check if the content of the buffer is empty
	fn is_empty(&self) -> bool {
		self.len() == 0
	}

	/// Set the binding target of the buffer
	fn set_target(&mut self, target: BufferTarget);

	/// Create a `BufferBind` to use the RAII system to manage the binding state.
	fn bind<'a>(&'a self) -> Result<BufferBind<'a>, GLCoreError> {
		self.get_buffer().bind()
	}

	/// Bind to a specific target. WILL NOT change the default target of the buffer. Create a `BufferBind` to use the RAII system to manage the binding state, while change the binding target.
	fn bind_to<'a>(&'a self, target: BufferTarget) -> Result<BufferBind<'a>, GLCoreError> {
		self.get_buffer().bind_to(target)
	}
}

/// The `BufferVecStatic` struct, although it doesn't supports
#[derive(Debug, Clone)]
pub struct BufferVecStatic<T: BufferVecItem> {
	pub glcore: Rc<GLCore>,
	buffer: Buffer,
	num_items: usize,
	capacity: usize,
	_item_type: PhantomData<T>,
}

/// A vectorized buffer that allows you to modify its content via providing your struct.
impl<T: BufferVecItem> BufferVecStatic<T> {
	/// Get the internal name
	pub fn get_name(&self) -> u32 {
		self.buffer.get_name()
	}

	/// Convert `Buffer` to an `BufferVecStatic`
	pub fn new(glcore: Rc<GLCore>, buffer: Buffer) -> Self {
		let capacity = buffer.size() / size_of::<T>();
		Self {
			glcore,
			buffer,
			num_items: 0,
			capacity,
			_item_type: PhantomData,
		}
	}
}

impl<T: BufferVecItem> BufferVec<T> for BufferVecStatic<T> {
	fn get_buffer(&self) -> &Buffer {
		&self.buffer
	}

	fn capacity(&self) -> usize {
		self.capacity
	}

	fn len(&self) -> usize {
		self.num_items
	}

	fn resize(&mut self, new_len: usize, value: T) -> Result<(), GLCoreError> {
		let new_size = new_len * size_of::<T>();
		if new_size > self.capacity {
			self.buffer.resize(new_len * size_of::<T>(), value)?;
		}
		self.num_items = new_len;
		Ok(())
	}

	fn shrink_to_fit(&mut self) -> Result<(), GLCoreError> {
		self.capacity = self.num_items;
		self.buffer.resize(self.capacity * size_of::<T>(), T::default())?;
		Ok(())
	}

	fn get(&self, index: usize) -> Result<T, GLCoreError> {
		let offset = index * size_of::<T>();
		let bind = self.buffer.bind()?;
		let (map, addr) = bind.map_ranged(offset, size_of::<T>(), MapAccess::WriteOnly)?;
		let addr = addr as *mut T;
		let ret = unsafe { *addr };
		map.unmap();
		Ok(ret)
	}

	fn set(&mut self, index: usize, data: &T) -> Result<(), GLCoreError> {
		let offset = index * size_of::<T>();
		let bind = self.buffer.bind()?;
		let (map, addr) = bind.map_ranged(offset, size_of::<T>(), MapAccess::WriteOnly)?;
		let addr = addr as *mut T;
		unsafe {
			*addr = *data;
		}
		map.unmap();
		Ok(())
	}

	fn get_slice_of_data(&self, start_index: usize, len: usize) -> Result<Vec<T>, GLCoreError> {
		let offset = start_index * size_of::<T>();
		let end_index = start_index + len;
		let bind = self.buffer.bind()?;
		let (map, addr) = bind.map_ranged(offset, len * size_of::<T>(), MapAccess::WriteOnly)?;
		let addr = addr as *mut T;
		let mut ret: Vec<T> = Vec::with_capacity(len);
		for i in start_index..end_index {
			ret.push(unsafe {*addr.wrapping_add(i)});
		}
		map.unmap();
		Ok(ret)
	}

	fn set_slice_of_data(&mut self, index: usize, data: &[T]) -> Result<(), GLCoreError> {
		let offset = index * size_of::<T>();
		let bind = self.buffer.bind()?;
		let (map, addr) = bind.map_ranged(offset, size_of_val(data), MapAccess::WriteOnly)?;
		let addr = addr as *mut T;
		for (i, item) in data.iter().enumerate() {
			unsafe { *addr.wrapping_add(i) = *item; };
		}
		map.unmap();
		Ok(())
	}

	fn set_target(&mut self, target: BufferTarget) {
		self.buffer.set_target(target)
	}
}

impl<T: BufferVecItem> From<BufferVecStatic<T>> for Buffer {
	fn from(val: BufferVecStatic<T>) -> Self {
		val.buffer
	}
}

impl<T: BufferVecItem> From<Buffer> for BufferVecStatic<T> {
	fn from(val: Buffer) -> Self {
		let capacity = val.size() / size_of::<T>();
		BufferVecStatic {
			glcore: val.glcore.clone(),
			buffer: val,
			num_items: 0,
			capacity,
			_item_type: PhantomData,
		}
	}
}

/// A high-level vectorized buffer that allows you to modify its content via index accessing/slicing
#[derive(Debug, Clone)]
pub struct BufferVecDynamic<T: BufferVecItem> {
	pub glcore: Rc<GLCore>,
	buffer: BufferVecStatic<T>,
	num_items: usize,
	capacity: usize,
	cache: Vec<T>,
	cache_modified_bitmap: BitVec,
	cache_modified: bool,
}

impl<T: BufferVecItem> BufferVecDynamic<T> {
	/// Get the internal name
	pub fn get_name(&self) -> u32 {
		self.buffer.get_name()
	}

	/// Convert an `BufferVecStatic` to the `BufferVecDynamic`
	pub fn new(buffer: BufferVecStatic<T>) -> Result<Self, GLCoreError> {
		let capacity = buffer.capacity();
		let mut cache_modified_bitmap = BitVec::new();
		let cache = buffer.get_slice_of_data(0, capacity)?;
		cache_modified_bitmap.resize(capacity, false);
		let num_items = buffer.len();
		Ok(Self {
			glcore: buffer.glcore.clone(),
			buffer,
			cache,
			cache_modified_bitmap,
			cache_modified: false,
			num_items,
			capacity
		})
	}
}

impl<T: BufferVecItem> BufferVec<T> for BufferVecDynamic<T> {
	fn get_buffer(&self) -> &Buffer {
		self.buffer.get_buffer()
	}

	fn len(&self) -> usize {
		self.num_items
	}

	fn capacity(&self) -> usize {
		self.capacity
	}

	fn resize(&mut self, new_len: usize, value: T) -> Result<(), GLCoreError> {
		self.cache.resize(new_len, value);
		self.num_items = new_len;
		if new_len > self.capacity {
			self.cache_modified_bitmap.clear(); // set all false
			self.cache_modified_bitmap.resize(new_len, false);
			self.buffer.resize(new_len, value)?;
			self.capacity = new_len;
			self.cache_modified = false;
		} else {
			self.cache_modified_bitmap.resize(new_len, false);
		}
		Ok(())
	}

	fn shrink_to_fit(&mut self) -> Result<(), GLCoreError> {
		if self.capacity > self.num_items {
			self.cache.shrink_to_fit();
			self.cache_modified_bitmap.clear(); // set all false
			self.cache_modified_bitmap.resize(self.num_items, false);
			self.buffer.resize(self.num_items, T::default())?;
			self.capacity = self.num_items;
			self.cache_modified = false;
		}
		Ok(())
	}

	fn get(&self, index: usize) -> Result<T, GLCoreError> {
		Ok(self.cache[index])
	}

	fn set(&mut self, index: usize, data: &T) -> Result<(), GLCoreError> {
		self.cache[index] = *data;
		self.cache_modified = true;
		self.cache_modified_bitmap.set(index, true);
		Ok(())
	}

	fn get_slice_of_data(&self, start_index: usize, len: usize) -> Result<Vec<T>, GLCoreError> {
		let end_index = start_index + len;
		Ok(self.cache[start_index..end_index].to_vec())
	}

	fn set_slice_of_data(&mut self, start_index: usize, data: &[T]) -> Result<(), GLCoreError> {
		let end_index = start_index + data.len();
		self.cache_modified = true;
		for i in start_index..end_index {
			self.cache[i] = data[i - start_index];
			self.cache_modified_bitmap.set(i, true);
		}
		Ok(())
	}

	fn set_target(&mut self, target: BufferTarget) {
		self.buffer.set_target(target)
	}

	fn flush(&mut self) -> Result<(), GLCoreError> {
		if !self.cache_modified {
			return Ok(());
		}

		const MAXIMUM_GAP: usize = 16;

		let mut is_in: bool = false;
		let mut start_index: usize = 0;
		let mut end_index: usize = 0;
		let mut gap_length: usize = 0;
		for i in 0..self.num_items {
			if self.cache_modified_bitmap[i] {
				if !is_in {
					is_in = true;
					start_index = i;
				}
				gap_length = 0;
				end_index = i;
				self.cache_modified_bitmap.set(i, false);
			} else if is_in {
   					if gap_length < MAXIMUM_GAP {
						gap_length += 1;
					} else {
						self.buffer.set_slice_of_data(0, &self.cache[start_index..=end_index])?;
						is_in = false;
					}
				}
		}
		if is_in {
			self.buffer.set_slice_of_data(0, &self.cache[start_index..=end_index])?;
		}

		self.cache_modified = false;
		Ok(())
	}
}

impl<T: BufferVecItem> From<BufferVecStatic<T>> for BufferVecDynamic<T> {
	fn from(val: BufferVecStatic<T>) -> Self {
		BufferVecDynamic::new(val).unwrap()
	}
}

impl<T: BufferVecItem> From<BufferVecDynamic<T>> for BufferVecStatic<T> {
	fn from(mut val: BufferVecDynamic<T>) -> Self {
		val.flush().unwrap();
		val.buffer
	}
}

impl<T: BufferVecItem> From<BufferVecDynamic<T>> for Buffer {
	fn from(val: BufferVecDynamic<T>) -> Self {
		val.buffer.into()
	}
}

impl<T: BufferVecItem> From<Buffer> for BufferVecDynamic<T> {
	fn from(val: Buffer) -> Self {
		let ab: BufferVecStatic<T> = val.into();
		ab.into()
	}
}

impl<T: BufferVecItem> Index<usize> for BufferVecDynamic<T> {
	type Output = T;
	fn index(&self, i: usize) -> &T {
		&self.cache[i]
	}
}

impl<T: BufferVecItem> IndexMut<usize> for BufferVecDynamic<T> {
	fn index_mut(&mut self, i: usize) -> &mut T {
		self.cache_modified = true;
		self.cache_modified_bitmap.set(i, true);
		&mut self.cache[i]
	}
}

impl<T: BufferVecItem> Index<Range<usize>> for BufferVecDynamic<T> {
	type Output = [T];
	fn index(&self, r: Range<usize>) -> &[T] {
		&self.cache[r]
	}
}

impl<T: BufferVecItem> IndexMut<Range<usize>> for BufferVecDynamic<T> {
	fn index_mut(&mut self, r: Range<usize>) -> &mut [T] {
		self.cache_modified = true;
		for i in r.clone() {
			self.cache_modified_bitmap.set(i, true);
		}
		&mut self.cache[r]
	}
}

impl<T: BufferVecItem> Index<RangeFrom<usize>> for BufferVecDynamic<T> {
	type Output = [T];
	fn index(&self, r: RangeFrom<usize>) -> &[T] {
		&self.cache[r]
	}
}

impl<T: BufferVecItem> IndexMut<RangeFrom<usize>> for BufferVecDynamic<T> {
	fn index_mut(&mut self, r: RangeFrom<usize>) -> &mut [T] {
		self.cache_modified = true;
		for i in r.clone() {
			self.cache_modified_bitmap.set(i, true);
		}
		&mut self.cache[r]
	}
}

impl<T: BufferVecItem> Index<RangeTo<usize>> for BufferVecDynamic<T> {
	type Output = [T];
	fn index(&self, r: RangeTo<usize>) -> &[T] {
		&self.cache[r]
	}
}

impl<T: BufferVecItem> IndexMut<RangeTo<usize>> for BufferVecDynamic<T> {
	fn index_mut(&mut self, r: RangeTo<usize>) -> &mut [T] {
		self.cache_modified = true;
		for i in 0..r.end {
			self.cache_modified_bitmap.set(i, true);
		}
		&mut self.cache[r]
	}
}

impl<T: BufferVecItem> Index<RangeFull> for BufferVecDynamic<T> {
	type Output = [T];
	fn index(&self, _: RangeFull) -> &[T] {
		&self.cache[..]
	}
}

impl<T: BufferVecItem> IndexMut<RangeFull> for BufferVecDynamic<T> {
	fn index_mut(&mut self, _: RangeFull) -> &mut [T] {
		self.cache_modified = true;
		for i in 0..self.num_items {
			self.cache_modified_bitmap.set(i, true);
		}
		&mut self.cache[..]
	}
}

impl<T: BufferVecItem> Index<RangeInclusive<usize>> for BufferVecDynamic<T> {
	type Output = [T];
	fn index(&self, r: RangeInclusive<usize>) -> &[T] {
		&self.cache[r]
	}
}

impl<T: BufferVecItem> IndexMut<RangeInclusive<usize>> for BufferVecDynamic<T> {
	fn index_mut(&mut self, r: RangeInclusive<usize>) -> &mut [T] {
		self.cache_modified = true;
		for i in r.clone() {
			self.cache_modified_bitmap.set(i, true);
		}
		&mut self.cache[r]
	}
}

impl<T: BufferVecItem> Index<RangeToInclusive<usize>> for BufferVecDynamic<T> {
	type Output = [T];
	fn index(&self, r: RangeToInclusive<usize>) -> &[T] {
		&self.cache[r]
	}
}

impl<T: BufferVecItem> IndexMut<RangeToInclusive<usize>> for BufferVecDynamic<T> {
	fn index_mut(&mut self, r: RangeToInclusive<usize>) -> &mut [T] {
		self.cache_modified = true;
		for i in 0..=r.end {
			self.cache_modified_bitmap.set(i, true);
		}
		&mut self.cache[r]
	}
}