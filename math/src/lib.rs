


pub fn add_two_numbers<T>(a: T, b: T) -> T
where
	T: std::ops::Add<Output = T>
{
	a + b
}


#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_add_two_numbers() {
		assert_eq!(add_two_numbers(1_u8, 2_u8), 3_u8);
		assert_eq!(add_two_numbers(2_i16, -3_i16), -1_i16);
		assert_eq!(add_two_numbers(2_f64, -3_f64), -1_f64);
	}
}


