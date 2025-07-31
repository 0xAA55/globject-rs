#![allow(dead_code)]

use glcore::*;
use crate::glbuffer::*;
use bitvec::vec::BitVec;
use std::{
	fmt::Debug,
	mem::size_of,
	ops::{Index, IndexMut, Range, RangeFrom, RangeTo, RangeFull, RangeInclusive, RangeToInclusive},
};

#[derive(Debug, Clone)]
pub struct BufferVec<'a> {
	pub glcore: &'a GLCore,
	buffer: Buffer<'a>,
}

pub trait BufferVecItem: Copy + Sized + Default + Debug {}
impl<T> BufferVecItem for T where T: Copy + Sized + Default + Debug {}

impl<'a> BufferVec<'a> {
	/// Convert `Buffer` to an `BufferVec`
	pub fn new(glcore: &'a GLCore, buffer: Buffer<'a>) -> Self {
		Self {
			glcore,
			buffer,
		}
	}

	/// Get the size of the buffer
	pub fn size(&self) -> usize {
		self.buffer.size()
	}

	/// Resize (reallocate) the buffer
	pub fn resize<T: BufferVecItem>(&'a mut self, new_len: usize, value: T) {
		self.buffer.resize(new_len, value)
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
		let (map, addr) = bind.map_ranged(offset, size_of::<T>() * data.len(), MapAccess::WriteOnly);
		let addr = addr as *mut T;
		for i in 0..data.len() {
			unsafe { data[i] = *addr.wrapping_add(i); };
		}
		map.unmap();
	}

	/// Update multiple data to GPU
	pub fn set_multi_data<T: BufferVecItem>(&mut self, index: usize, data: &[T]) {
		let offset = index * size_of::<T>();
		let bind = self.buffer.bind();
		let (map, addr) = bind.map_ranged(offset, size_of::<T>() * data.len(), MapAccess::WriteOnly);
		let addr = addr as *mut T;
		for i in 0..data.len() {
			unsafe { *addr.wrapping_add(i) = data[i]; };
		}
		map.unmap();
	}
}

impl<'a> Into<Buffer<'a>> for BufferVec<'a> {
	fn into(self) -> Buffer<'a> {
		self.buffer
	}
}

impl<'a> Into<BufferVec<'a>> for Buffer<'a> {
	fn into(self) -> BufferVec<'a> {
		BufferVec {
			glcore: self.glcore,
			buffer: self,
		}
	}
}

#[derive(Debug, Clone)]
pub struct BufferVecDynamic<'a, T: BufferVecItem> {
	pub glcore: &'a GLCore,
	buffer: BufferVec<'a>,
	num_items: usize,
	capacity: usize,
	cache: Vec<T>,
	cache_modified_bitmap: BitVec,
	cache_modified: bool,
}

impl<'a, T: BufferVecItem> BufferVecDynamic<'a, T> {
	/// Convert an `BufferVec` to the `BufferVecDynamic`
	pub fn new(buffer: BufferVec<'a>, num_items: usize) -> Self {
		let capacity = buffer.size() / size_of::<T>();
		let mut cache_modified_bitmap = BitVec::new();
		let mut cache = Vec::new();
		cache_modified_bitmap.resize(capacity, false);
		cache.resize(capacity, T::default());
		buffer.get_multi_data(0, &mut cache);
		Self {
			glcore: buffer.glcore,
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

	/// Get the capacity of the current buffer
	pub fn capacity(&self) -> usize {
		self.capacity
	}

	/// Get the buffer
	pub fn get_buffer(&self) -> &Buffer {
		&self.buffer.get_buffer()
	}

	/// Resizes to the new size, reallocate the buffer if the new size is larger.
	pub fn resize(&'a mut self, new_len: usize, value: T) {
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
	pub fn shrink_to_fit(&'a mut self) {
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
		if self.cache_modified == false {
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
			} else {
				if is_in {
					if gap_length < MAXIMUM_GAP {
						gap_length += 1;
					} else {
						self.buffer.set_multi_data(0, &self.cache[start_index..=end_index]);
						is_in = false;
					}
				}
			}
		}
		if is_in {
			self.buffer.set_multi_data(0, &self.cache[start_index..=end_index]);
		}

		self.cache_modified = false;
	}
}

impl<'a, T: BufferVecItem> Into<BufferVecDynamic<'a, T>> for BufferVec<'a> {
	fn into(self) -> BufferVecDynamic<'a, T> {
		let num_items = self.buffer.size() / size_of::<T>();
		BufferVecDynamic::new(self, num_items)
	}
}

impl<'a, T: BufferVecItem> Into<BufferVec<'a>> for BufferVecDynamic<'a, T> {
	fn into(mut self) -> BufferVec<'a> {
		self.flush();
		self.buffer
	}
}

impl<'a, T: BufferVecItem> Into<Buffer<'a>> for BufferVecDynamic<'a, T> {
	fn into(self) -> Buffer<'a> {
		self.buffer.into()
	}
}

impl<'a, T: BufferVecItem> Into<BufferVecDynamic<'a, T>> for Buffer<'a> {
	fn into(self) -> BufferVecDynamic<'a, T> {
		let ab: BufferVec = self.into();
		ab.into()
	}
}

impl<'a, T: BufferVecItem> Index<usize> for BufferVecDynamic<'a, T> {
	type Output = T;
	fn index(&self, i: usize) -> &T {
		&self.cache[i]
	}
}

impl<'a, T: BufferVecItem> IndexMut<usize> for BufferVecDynamic<'a, T> {
	fn index_mut(&mut self, i: usize) -> &mut T {
		self.cache_modified = true;
		self.cache_modified_bitmap.set(i, true);
		&mut self.cache[i]
	}
}

impl<'a, T: BufferVecItem> Index<Range<usize>> for BufferVecDynamic<'a, T> {
	type Output = [T];
	fn index(&self, r: Range<usize>) -> &[T] {
		&self.cache[r]
	}
}

impl<'a, T: BufferVecItem> IndexMut<Range<usize>> for BufferVecDynamic<'a, T> {
	fn index_mut(&mut self, r: Range<usize>) -> &mut [T] {
		self.cache_modified = true;
		for i in r.start..r.end {
			self.cache_modified_bitmap.set(i, true);
		}
		&mut self.cache[r]
	}
}

impl<'a, T: BufferVecItem> Index<RangeFrom<usize>> for BufferVecDynamic<'a, T> {
	type Output = [T];
	fn index(&self, r: RangeFrom<usize>) -> &[T] {
		&self.cache[r]
	}
}

impl<'a, T: BufferVecItem> IndexMut<RangeFrom<usize>> for BufferVecDynamic<'a, T> {
	fn index_mut(&mut self, r: RangeFrom<usize>) -> &mut [T] {
		self.cache_modified = true;
		for i in r.start..self.num_items {
			self.cache_modified_bitmap.set(i, true);
		}
		&mut self.cache[r]
	}
}

impl<'a, T: BufferVecItem> Index<RangeTo<usize>> for BufferVecDynamic<'a, T> {
	type Output = [T];
	fn index(&self, r: RangeTo<usize>) -> &[T] {
		&self.cache[r]
	}
}

impl<'a, T: BufferVecItem> IndexMut<RangeTo<usize>> for BufferVecDynamic<'a, T> {
	fn index_mut(&mut self, r: RangeTo<usize>) -> &mut [T] {
		self.cache_modified = true;
		for i in 0..r.end {
			self.cache_modified_bitmap.set(i, true);
		}
		&mut self.cache[r]
	}
}

impl<'a, T: BufferVecItem> Index<RangeFull> for BufferVecDynamic<'a, T> {
	type Output = [T];
	fn index(&self, r: RangeFull) -> &[T] {
		&self.cache[r]
	}
}

impl<'a, T: BufferVecItem> IndexMut<RangeFull> for BufferVecDynamic<'a, T> {
	fn index_mut(&mut self, r: RangeFull) -> &mut [T] {
		self.cache_modified = true;
		for i in 0..self.num_items {
			self.cache_modified_bitmap.set(i, true);
		}
		&mut self.cache[r]
	}
}

impl<'a, T: BufferVecItem> Index<RangeInclusive<usize>> for BufferVecDynamic<'a, T> {
	type Output = [T];
	fn index(&self, r: RangeInclusive<usize>) -> &[T] {
		&self.cache[r]
	}
}

impl<'a, T: BufferVecItem> IndexMut<RangeInclusive<usize>> for BufferVecDynamic<'a, T> {
	fn index_mut(&mut self, r: RangeInclusive<usize>) -> &mut [T] {
		self.cache_modified = true;
		for i in *r.start()..=*r.end() {
			self.cache_modified_bitmap.set(i, true);
		}
		&mut self.cache[r]
	}
}

impl<'a, T: BufferVecItem> Index<RangeToInclusive<usize>> for BufferVecDynamic<'a, T> {
	type Output = [T];
	fn index(&self, r: RangeToInclusive<usize>) -> &[T] {
		&self.cache[r]
	}
}

impl<'a, T: BufferVecItem> IndexMut<RangeToInclusive<usize>> for BufferVecDynamic<'a, T> {
	fn index_mut(&mut self, r: RangeToInclusive<usize>) -> &mut [T] {
		self.cache_modified = true;
		for i in 0..=r.end {
			self.cache_modified_bitmap.set(i, true);
		}
		&mut self.cache[r]
	}
}





