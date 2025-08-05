
pub fn to_camel_case(snake_case: &str, first_letter_uppercase: bool) -> String {
	let mut ret = String::new();
	let mut last_is_underscore = first_letter_uppercase;
	for ch in snake_case.chars() {
		if ch == '_' {
			last_is_underscore = true;
		} else {
			if last_is_underscore {
				for ch in ch.to_uppercase() {
					ret.push(ch);
				}
			} else {
				ret.push(ch);
			}
			last_is_underscore = false;
		}
	}
	ret
}
