#![allow(dead_code)]

use glcore::*;
use crate::glbuffer::*;
use bitvec::vec::BitVec;
use std::{
	mem::size_of,
	ops::{Index, IndexMut, Range, RangeFrom, RangeTo, RangeFull, RangeInclusive, RangeToInclusive},
};

#[derive(Debug, Clone)]
pub struct ArrayBuffer<'a> {
	glcore: &'a GLCore,
	buffer: Buffer<'a>,
}

pub trait ArrayBufferItem: Copy + Sized + Default {}
impl<T> ArrayBufferItem for T where T: Copy + Sized + Default {}

impl<'a> ArrayBuffer<'a> {
	pub fn new(glcore: &'a GLCore, mut buffer: Buffer<'a>) -> Self {
		buffer.set_target(BufferTarget::ArrayBuffer);
		Self {
			glcore,
			buffer,
		}
	}

	pub fn size(&self) -> usize {
		self.buffer.size()
	}

	pub fn resize<T: ArrayBufferItem>(&'a mut self, new_len: usize, value: T) {
		self.buffer.resize(new_len, value)
	}

	pub fn get_data<T: ArrayBufferItem>(&self, index: usize) -> T {
		let offset = index * size_of::<T>();
		let bind = self.buffer.bind();
		let (map, addr) = bind.map_ranged(offset, size_of::<T>(), MapAccess::WriteOnly);
		let addr = addr as *mut T;
		let ret = unsafe { *addr };
		map.unmap();
		ret
	}

	pub fn set_data<T: ArrayBufferItem>(&mut self, index: usize, data: &T) {
		let offset = index * size_of::<T>();
		let bind = self.buffer.bind();
		let (map, addr) = bind.map_ranged(offset, size_of::<T>(), MapAccess::WriteOnly);
		let addr = addr as *mut T;
		unsafe {
			*addr = *data;
		}
		map.unmap();
	}

	pub fn get_multi_data<T: ArrayBufferItem>(&self, index: usize, data: &mut [T]) {
		let offset = index * size_of::<T>();
		let bind = self.buffer.bind();
		let (map, addr) = bind.map_ranged(offset, size_of::<T>() * data.len(), MapAccess::WriteOnly);
		let addr = addr as *mut T;
		for i in 0..data.len() {
			unsafe { data[i] = *addr.wrapping_add(i); };
		}
		map.unmap();
	}

	pub fn set_multi_data<T: ArrayBufferItem>(&mut self, index: usize, data: &[T]) {
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

impl<'a> Into<Buffer<'a>> for ArrayBuffer<'a> {
	fn into(self) -> Buffer<'a> {
		self.buffer
	}
}

#[derive(Debug, Clone)]
pub struct ArrayBufferDynamic<'a, T: ArrayBufferItem> {
	glcore: &'a GLCore,
	buffer: ArrayBuffer<'a>,
	num_items: usize,
	capacity: usize,
	cache: Vec<T>,
	cache_modified_bitmap: BitVec,
	cache_modified: bool,
}

impl<'a, T: ArrayBufferItem> ArrayBufferDynamic<'a, T> {
	pub fn new(buffer: ArrayBuffer<'a>, num_items: usize) -> Self {
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

	pub fn len(&self) -> usize {
		self.num_items
	}

	pub fn capacity(&self) -> usize {
		self.capacity
	}

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

impl<'a, T: ArrayBufferItem> Into<ArrayBufferDynamic<'a, T>> for ArrayBuffer<'a> {
	fn into(self) -> ArrayBufferDynamic<'a, T> {
		let num_items = self.buffer.size() / size_of::<T>();
		ArrayBufferDynamic::new(self, num_items)
	}
}

impl<'a, T: ArrayBufferItem> Into<ArrayBuffer<'a>> for ArrayBufferDynamic<'a, T> {
	fn into(mut self) -> ArrayBuffer<'a> {
		self.flush();
		self.buffer
	}
}

impl<'a, T: ArrayBufferItem> Index<usize> for ArrayBufferDynamic<'a, T> {
	type Output = T;
	fn index(&self, i: usize) -> &T {
		&self.cache[i]
	}
}

impl<'a, T: ArrayBufferItem> IndexMut<usize> for ArrayBufferDynamic<'a, T> {
	fn index_mut(&mut self, i: usize) -> &mut T {
		self.cache_modified = true;
		self.cache_modified_bitmap.set(i, true);
		&mut self.cache[i]
	}
}

impl<'a, T: ArrayBufferItem> Index<Range<usize>> for ArrayBufferDynamic<'a, T> {
	type Output = [T];
	fn index(&self, r: Range<usize>) -> &[T] {
		&self.cache[r]
	}
}

impl<'a, T: ArrayBufferItem> IndexMut<Range<usize>> for ArrayBufferDynamic<'a, T> {
	fn index_mut(&mut self, r: Range<usize>) -> &mut [T] {
		self.cache_modified = true;
		for i in r.start..r.end {
			self.cache_modified_bitmap.set(i, true);
		}
		&mut self.cache[r]
	}
}

impl<'a, T: ArrayBufferItem> Index<RangeFrom<usize>> for ArrayBufferDynamic<'a, T> {
	type Output = [T];
	fn index(&self, r: RangeFrom<usize>) -> &[T] {
		&self.cache[r]
	}
}

impl<'a, T: ArrayBufferItem> IndexMut<RangeFrom<usize>> for ArrayBufferDynamic<'a, T> {
	fn index_mut(&mut self, r: RangeFrom<usize>) -> &mut [T] {
		self.cache_modified = true;
		for i in r.start..self.num_items {
			self.cache_modified_bitmap.set(i, true);
		}
		&mut self.cache[r]
	}
}

impl<'a, T: ArrayBufferItem> Index<RangeTo<usize>> for ArrayBufferDynamic<'a, T> {
	type Output = [T];
	fn index(&self, r: RangeTo<usize>) -> &[T] {
		&self.cache[r]
	}
}

impl<'a, T: ArrayBufferItem> IndexMut<RangeTo<usize>> for ArrayBufferDynamic<'a, T> {
	fn index_mut(&mut self, r: RangeTo<usize>) -> &mut [T] {
		self.cache_modified = true;
		for i in 0..r.end {
			self.cache_modified_bitmap.set(i, true);
		}
		&mut self.cache[r]
	}
}

impl<'a, T: ArrayBufferItem> Index<RangeFull> for ArrayBufferDynamic<'a, T> {
	type Output = [T];
	fn index(&self, r: RangeFull) -> &[T] {
		&self.cache[r]
	}
}

impl<'a, T: ArrayBufferItem> IndexMut<RangeFull> for ArrayBufferDynamic<'a, T> {
	fn index_mut(&mut self, r: RangeFull) -> &mut [T] {
		self.cache_modified = true;
		for i in 0..self.num_items {
			self.cache_modified_bitmap.set(i, true);
		}
		&mut self.cache[r]
	}
}

impl<'a, T: ArrayBufferItem> Index<RangeInclusive<usize>> for ArrayBufferDynamic<'a, T> {
	type Output = [T];
	fn index(&self, r: RangeInclusive<usize>) -> &[T] {
		&self.cache[r]
	}
}

impl<'a, T: ArrayBufferItem> IndexMut<RangeInclusive<usize>> for ArrayBufferDynamic<'a, T> {
	fn index_mut(&mut self, r: RangeInclusive<usize>) -> &mut [T] {
		self.cache_modified = true;
		for i in *r.start()..=*r.end() {
			self.cache_modified_bitmap.set(i, true);
		}
		&mut self.cache[r]
	}
}

impl<'a, T: ArrayBufferItem> Index<RangeToInclusive<usize>> for ArrayBufferDynamic<'a, T> {
	type Output = [T];
	fn index(&self, r: RangeToInclusive<usize>) -> &[T] {
		&self.cache[r]
	}
}

impl<'a, T: ArrayBufferItem> IndexMut<RangeToInclusive<usize>> for ArrayBufferDynamic<'a, T> {
	fn index_mut(&mut self, r: RangeToInclusive<usize>) -> &mut [T] {
		self.cache_modified = true;
		for i in 0..=r.end {
			self.cache_modified_bitmap.set(i, true);
		}
		&mut self.cache[r]
	}
}





