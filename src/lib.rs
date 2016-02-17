#![feature(core_intrinsics)]

pub fn get_type_of<T>(_: &T) -> String {
	unsafe {
		std::intrinsics::type_name::<T>()
	}.to_owned()
}

#[macro_export]
macro_rules! dump(
	($($a:expr),*) => {
		println!(concat!("[", file!(), ":", line!(), "] ", $(stringify!($a), ": {} = {:?}; "),*), $($crate::get_type_of(&$a), $a),*);
	}
);

#[cfg(test)]
mod test {
	#[test]
	fn it_works() {
		let s = String::new();
		let n = 3;
		dump!(s);
		// The output looks kind of nonsensical when passing in
		// literals instead of variable names, but it still works.
		dump!(&&n, s, 9, "test", 2 + 2);
	}
}
