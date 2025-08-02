
#![allow(dead_code)]

use glcore::*;
use crate::glbuffer::*;
use bitvec::vec::BitVec;
use std::{
	fmt::Debug,
	mem::{size_of, size_of_val},
	ops::{Index, IndexMut, Range, RangeFrom, RangeTo, RangeFull, RangeInclusive, RangeToInclusive},
	rc::Rc,
};

#[derive(Debug, Clone)]
pub struct BufferVec {
	pub glcore: Rc<GLCore>,
	buffer: Buffer,
}

pub trait BufferVecItem: Copy + Sized + Default + Debug {}
impl<T> BufferVecItem for T where T: Copy + Sized + Default + Debug {}

impl BufferVec {
	/// Get the internal name
	pub fn get_name(&self) -> u32 {
		self.buffer.get_name()
	}

	/// Convert `Buffer` to an `BufferVec`
	pub fn new(glcore: Rc<GLCore>, buffer: Buffer) -> Self {
		Self {
			glcore,
			buffer,
		}
	}

	/// Get the size of the buffer
	pub fn size_in_bytes(&self) -> usize {
		self.buffer.size()
	}

	/// Resize (reallocate) the buffer
	pub fn resize<T: BufferVecItem>(&mut self, new_len: usize, value: T) {
		self.buffer.resize(new_len * size_of::<T>(), value)
	}

	/// Get the buffer
	pub fn get_buffer(&self) -> &Buffer {
		&self.buffer
	}

	/// Retrieve data from GPU
	pub fn get_data<T: BufferVecItem>(&self, index: usize) -> T {
		let offset = index * size_of::<T>();
		let bind = self.buffer.bind();
		let (map, addr) = bind.map_ranged(offset, size_of::<T>(), MapAccess::WriteOnly);
		let addr = addr as *mut T;
		let ret = unsafe { *addr };
		map.unmap();
		ret
	}

	/// Update data to GPU
	pub fn set_data<T: BufferVecItem>(&mut self, index: usize, data: &T) {
		let offset = index * size_of::<T>();
		let bind = self.buffer.bind();
		let (map, addr) = bind.map_ranged(offset, size_of::<T>(), MapAccess::WriteOnly);
		let addr = addr as *mut T;
		unsafe {
			*addr = *data;
		}
		map.unmap();
	}

	/// Retrieve multiple data from GPU
	pub fn get_multi_data<T: BufferVecItem>(&self, index: usize, data: &mut [T]) {
		let offset = index * size_of::<T>();
		let bind = self.buffer.bind();
		let (map, addr) = bind.map_ranged(offset, size_of_val(data), MapAccess::WriteOnly);
		let addr = addr as *mut T;
		for (i, item) in data.iter_mut().enumerate() {
			unsafe { *item = *addr.wrapping_add(i); };
		}
		map.unmap();
	}

	/// Update multiple data to GPU
	pub fn set_multi_data<T: BufferVecItem>(&mut self, index: usize, data: &[T]) {
		let offset = index * size_of::<T>();
		let bind = self.buffer.bind();
		let (map, addr) = bind.map_ranged(offset, size_of_val(data), MapAccess::WriteOnly);
		let addr = addr as *mut T;
		for (i, item) in data.iter().enumerate() {
			unsafe { *addr.wrapping_add(i) = *item; };
		}
		map.unmap();
	}

	/// Set the default binding target
	pub fn set_target(&mut self, target: BufferTarget) {
		self.buffer.set_target(target)
	}

	/// Create a `BufferBind` to use the RAII system to manage the binding state.
	pub fn bind<'a>(&'a self) -> BufferBind<'a>{
		self.buffer.bind()
	}

	/// Create a `BufferBind` to use the RAII system to manage the binding state, while change the binding target.
	pub fn bind_to<'a>(&'a mut self, target: BufferTarget) -> BufferBind<'a> {
		self.buffer.bind_to(target)
	}
}

impl<'a> From<BufferVec> for Buffer {
	fn from(val: BufferVec) -> Self {
		val.buffer
	}
}

impl<'a> From<Buffer> for BufferVec {
	fn from(val: Buffer) -> Self {
		BufferVec {
			glcore: val.glcore.clone(),
			buffer: val,
		}
	}
}

#[derive(Debug, Clone)]
pub struct BufferVecDynamic<T: BufferVecItem> {
	pub glcore: Rc<GLCore>,
	buffer: BufferVec,
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

	/// Convert an `BufferVec` to the `BufferVecDynamic`
	pub fn new(buffer: BufferVec, num_items: usize) -> Self {
		let capacity = buffer.size_in_bytes() / size_of::<T>();
		let mut cache_modified_bitmap = BitVec::new();
		let mut cache = Vec::new();
		cache_modified_bitmap.resize(capacity, false);
		cache.resize(capacity, T::default());
		buffer.get_multi_data(0, &mut cache);
		Self {
			glcore: buffer.glcore.clone(),
			buffer,
			cache,
			cache_modified_bitmap,
			cache_modified: false,
			num_items,
			capacity
		}
	}

	/// Get num items of the buffer
	pub fn len(&self) -> usize {
		self.num_items
	}

	/// Check if the buffer is empty
	pub fn is_empty(&self) -> bool {
		self.num_items == 0
	}

	/// Get the capacity of the current buffer
	pub fn capacity(&self) -> usize {
		self.capacity
	}

	/// Get the buffer
	pub fn get_buffer(&self) -> &Buffer {
		self.buffer.get_buffer()
	}

	/// Resizes to the new size, reallocate the buffer if the new size is larger.
	pub fn resize(&mut self, new_len: usize, value: T) {
		self.cache.resize(new_len, value);
		self.num_items = new_len;
		if new_len > self.capacity {
			self.cache_modified_bitmap.clear(); // set all false
			self.cache_modified_bitmap.resize(new_len, false);
			self.buffer.resize(new_len, value);
			self.capacity = new_len;
			self.cache_modified = false;
		} else {
			self.cache_modified_bitmap.resize(new_len, false);
		}
	}

	/// Reallocate the buffer to fit
	pub fn shrink_to_fit(&mut self) {
		if self.capacity > self.num_items {
			self.cache.shrink_to_fit();
			self.cache_modified_bitmap.clear(); // set all false
			self.cache_modified_bitmap.resize(self.num_items, false);
			self.buffer.resize(self.num_items, T::default());
			self.capacity = self.num_items;
			self.cache_modified = false;
		}
	}

	/// Commit all changes to the buffer to OpenGL Buffer Object
	pub fn flush(&mut self) {
		if !self.cache_modified {
			return;
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
						self.buffer.set_multi_data(0, &self.cache[start_index..=end_index]);
						is_in = false;
					}
				}
		}
		if is_in {
			self.buffer.set_multi_data(0, &self.cache[start_index..=end_index]);
		}

		self.cache_modified = false;
	}

	/// Set the default binding target
	pub fn set_target(&mut self, target: BufferTarget) {
		self.buffer.set_target(target)
	}

	/// Create a `BufferBind` to use the RAII system to manage the binding state.
	pub fn bind<'a>(&'a self) -> BufferBind<'a>{
		self.buffer.bind()
	}

	/// Create a `BufferBind` to use the RAII system to manage the binding state, while change the binding target.
	pub fn bind_to<'a>(&'a mut self, target: BufferTarget) -> BufferBind<'a> {
		self.buffer.bind_to(target)
	}
}

impl<T: BufferVecItem> From<BufferVec> for BufferVecDynamic<T> {
	fn from(val: BufferVec) -> Self {
		let num_items = val.buffer.size() / size_of::<T>();
		BufferVecDynamic::new(val, num_items)
	}
}

impl<T: BufferVecItem> From<BufferVecDynamic<T>> for BufferVec {
	fn from(mut val: BufferVecDynamic<T>) -> Self {
		val.flush();
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
		let ab: BufferVec = val.into();
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
		for i in r.start..r.end {
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
		for i in r.start..self.num_items {
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
	fn index(&self, r: RangeFull) -> &[T] {
		&self.cache[r]
	}
}

impl<T: BufferVecItem> IndexMut<RangeFull> for BufferVecDynamic<T> {
	fn index_mut(&mut self, r: RangeFull) -> &mut [T] {
		self.cache_modified = true;
		for i in 0..self.num_items {
			self.cache_modified_bitmap.set(i, true);
		}
		&mut self.cache[r]
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
		for i in *r.start()..=*r.end() {
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





