pub mod main_module {
	pub fn show_list() {
		let solved_problems: [&str;1] = [
			"131. Remove Duplicate Letters"
		];

		println!("Below is a list of solved leetcode problems");
		
        for prob_name in &solved_problems {
			println!("{}", prob_name);
		}
    }

}

pub mod remove_duplicate_letters_131;
pub mod max_points_on_a_line_149;
pub mod nested_list_weight_sum_2_364;
pub mod factor_combinations_254;

#[cfg(test)]
mod tests {
	#[test]
	fn test_131 () {
		use crate::remove_duplicate_letters_131::remove_duplicate_letters as test_func;
		assert_eq!(test_func(String::from("abcdeeasc")), String::from("abcdes"));
		assert_eq!(test_func(String::from("")), String::from(""));
		assert_eq!(test_func(String::from("bcabc")), String::from("abc"));
		assert_eq!(test_func(String::from("cbacdcbc")), String::from("acdb"));
		assert_eq!(test_func(String::from("bbcaac")), String::from("bac"));
	}

	#[test]
	fn test_149() {
		use crate::max_points_on_a_line_149::max_points as test_func;
		assert_eq!(test_func(vec![vec![1,1], vec![3,2], vec![5,3], vec![4,1], vec![2,3], vec![1,4]]), 4);
		assert_eq!(test_func(vec![vec![0,0]]), 1);
	}

	#[test]
	fn test_364() {
		use crate::nested_list_weight_sum_2_364::depth_sum_inverse as test_func;
		use crate::nested_list_weight_sum_2_364::NestedInteger;
		assert_eq!(test_func(vec![
			NestedInteger::List(vec![NestedInteger::Int(1), NestedInteger::Int(1)]),
			NestedInteger::Int(2),
			NestedInteger::List(vec![NestedInteger::Int(1), NestedInteger::Int(1)])
		]), 8);
		assert_eq!(test_func(vec![
			NestedInteger::Int(1),
			NestedInteger::List(vec![NestedInteger::Int(4), NestedInteger::List(vec![NestedInteger::Int(6)])])
		]), 17);
	}

	#[test]
	fn test_254() {
		use crate::factor_combinations_254::get_factors as test_func;
		assert_eq!(test_func(12), vec![
			vec![2,2,3],
			vec![2,6],
			vec![3,4]
		]);
		assert_eq!(test_func(16), vec![
			vec![2,2,2,2],
			vec![2,2,4],
			vec![2,8],
			vec![4,4]
		]);
	}
}
