pub fn add_u8_with_carry(l: u8, r: u8, c: bool) -> (u8, bool) {
	let mut sum = (l as u16) + (r as u16);
	
	if c {
		sum += 1
	}

	let res = (sum & 0xFF) as u8;
	(res, sum > 255)
}

#[cfg(test)]
mod test {
	mod add_u8_with_carry {
		use util::add_u8_with_carry;

		#[test]
		pub fn adds_numbers_that_fit_in_u8() {
			let (a, c) = add_u8_with_carry(40, 2, false);
			assert_eq!(a, 42);
			assert!(!c);
		}

		#[test]
		pub fn applies_carry_value() {
			let (a, c) = add_u8_with_carry(40, 1, true);
			assert_eq!(a, 42);
			assert!(!c);
		}

		#[test]
		pub fn returns_carry_if_overflows() {
			let (a, c) = add_u8_with_carry(255, 10, true);
			assert_eq!(a, 10);
			assert!(c);
		}
	}
}