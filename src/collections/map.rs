use crate::{Shared, Object};
use crate::collections::{Collection, Mapping};
use std::iter::FromIterator;

type Pair = (Shared<Object>, Shared<Object>);

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Map {
	data: Vec<Pair>
}

impl Map {
	#[inline]
	pub fn new(data: Vec<Pair>) -> Map {
		Map { data }
	}

	#[inline]
	pub fn empty() -> Map {
		Map::default()
	}

	pub fn iter(&self) -> impl Iterator<Item=&Pair> {
		self.data.iter()
	}

	pub fn keys(&self) -> impl Iterator<Item=&Shared<Object>> {
		struct KeyIter<'a>(std::slice::Iter<'a, Pair>);
		impl<'a> Iterator for KeyIter<'a> {
			type Item = &'a Shared<Object>;
			fn next(&mut self) -> Option<&'a Shared<Object>> {
				self.0.next().map(|(k, _)| k)
			}
		}
		KeyIter(self.data.iter())
	}
}

impl FromIterator<Pair> for Map {
	fn from_iter<T: IntoIterator<Item=Pair>>(iter: T) -> Map {
		Map::new(Vec::from_iter(iter))
	}
}

impl Collection for Map {
	fn len(&self) -> usize {
		self.data.len()
	}

	fn is_empty(&self) -> bool {
		self.data.is_empty()
	}
}

impl Mapping for Map {
	fn get(&self, key: &Shared<Object>) -> Option<&Shared<Object>> {
		self.iter().find_map(|(k, v)| if k == key { Some(v) } else { None })
	}

	fn set(&mut self, key: Shared<Object>, val: Shared<Object>) -> Option<Shared<Object>> {
		for i in 0..self.data.len() {
			if self.data[i].0 == key {
				let v = self.data[i].1.clone();
				self.data[i] = (key, val);
				return Some(v)
			}
		}
		self.data.push((key, val));
		None
	}

	fn del(&mut self, key: &Shared<Object>) -> Option<Shared<Object>> {
		for i in 0..self.data.len() {
			if &self.data[i].0 == key {
				return Some(self.data.swap_remove(i).1)
			}
		}
		None
	}

	fn has(&self, key: &Shared<Object>) -> bool {
		self.keys().any(|k| k == key)
	}
}
