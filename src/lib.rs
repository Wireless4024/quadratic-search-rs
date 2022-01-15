#![feature(core_intrinsics)]
#![feature(test)]
extern crate test;

use std::cmp::Ordering;
use std::cmp::Ordering::{Greater, Less};
use std::intrinsics::assume;

#[inline]
pub fn quadratic_search_by<'a, T, F>(arr: &'a [T], mut f: F) -> Result<usize, usize>
	where F: FnMut(&'a T) -> Ordering {
	let mut left = 0usize;
	let mut right = arr.len() - 1;
	while right > left {
		let mid = (right + left) >> 1;
		let cmp = unsafe { f(arr.get_unchecked(mid)) };

		if cmp == Less {
			left = mid + 1;
		} else if cmp == Greater {
			right = mid;
		} else {
			unsafe { assume(mid < arr.len()); }
			return Ok(mid);
		}
		let mid = (right + left) >> 1;
		let cmp = unsafe { f(arr.get_unchecked(mid)) };

		if cmp == Less {
			left = mid + 1;
		} else if cmp == Greater {
			right = mid;
		} else {
			unsafe { assume(mid < arr.len()); }
			return Ok(mid);
		}
	}
	unsafe { assume(left < arr.len()); }
	Err(left)
}

#[inline]
pub fn quadratic_search<T: Ord>(arr: &[T], value: &T) -> Result<usize, usize> {
	quadratic_search_by(arr, |it| it.cmp(value))
}

pub trait QuadraticSearch<'a, T: 'a + Ord> {
	fn quadratic_search(&self, value: &T) -> Result<usize, usize>;

	fn quadratic_search_by<F>(&self, f: F) -> Result<usize, usize> where F: FnMut(&T) -> Ordering;
}

impl<'a, T: 'a + Ord> QuadraticSearch<'a, T> for [T] {
	fn quadratic_search(&self, value: &T) -> Result<usize, usize> {
		self.quadratic_search_by(|it| it.cmp(value))
	}
	fn quadratic_search_by<F>(&self, f: F) -> Result<usize, usize> where F: FnMut(&T) -> Ordering {
		quadratic_search_by(self, f)
	}
}


#[cfg(test)]
mod tests {
	use test::Bencher;

	use rand::{RngCore, thread_rng};

	use super::*;

	#[test]
	fn it_works() {
		let mut arr = Vec::<u128>::with_capacity(SEARCH_BENCH_LEN);
		for i in 0..SEARCH_BENCH_LEN {
			arr.push(i as u128);
		}
		for i in 0..SEARCH_BENCH_LEN {
			let value = i as u128;
			assert_eq!(arr.binary_search(&value), quadratic_search(arr.as_slice(), &value));
		}
	}

	const SEARCH_BENCH_LEN: usize = 1 << 23; // 8M

	#[inline(never)]
	fn get_number() -> u128 {
		(SEARCH_BENCH_LEN >> 1) as u128 - 1
	}

	#[bench]
	fn bin_search(bench: &mut Bencher) {
		let mut arr = Vec::<u128>::with_capacity(SEARCH_BENCH_LEN);
		for i in 0..SEARCH_BENCH_LEN {
			arr.push(i as u128);
		}
		//std::env::var("NUMBER").and_then(|it| Ok(it.parse::<u128>().unwrap_or(get_number()))).unwrap_or(get_number());
		bench.iter(|| {
			let value: u128 = thread_rng().next_u64() as u128;
			let loc = arr.binary_search(&value);
			match loc {
				Ok(idx) => {
					assert_eq!(arr[idx], value)
				}
				_ => {
					assert!(value >= SEARCH_BENCH_LEN as u128)
				}
			}
		});
	}

	#[bench]
	fn qua_search(bench: &mut Bencher) {
		let mut arr = Vec::<u128>::with_capacity(SEARCH_BENCH_LEN);
		for i in 0..SEARCH_BENCH_LEN {
			arr.push(i as u128);
		}

		//;std::env::var("NUMBER").and_then(|it| Ok(it.parse::<u128>().unwrap_or(get_number()))).unwrap_or(get_number());
		bench.iter(|| {
			let value: u128 = thread_rng().next_u64() as u128;
			let loc = arr.quadratic_search(&value);

			match loc {
				Ok(idx) => {
					assert_eq!(arr[idx], value)
				}
				_ => {
					assert!(value >= SEARCH_BENCH_LEN as u128)
				}
			}
		});
	}
}