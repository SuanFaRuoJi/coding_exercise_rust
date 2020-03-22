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
pub mod find_leaves_of_binary_tree_366;
pub mod evaluate_reverse_polish_notation_150;
pub mod tree_node;
pub mod valid_triangle_number_611;
pub mod shortest_word_distance_3;
pub mod graph_valid_tree_261;
pub mod longest_palindrome_subsequence_516;

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

	#[test]
	fn test_150() {
		use crate::evaluate_reverse_polish_notation_150::eval_rpn as test_func;
		assert_eq!(test_func(vec![
			"1".to_string(), "2".to_string(), "*".to_string()
		]), 2);
	}

	#[test]
	fn test_611() {
		use crate::valid_triangle_number_611::triangle_number as test_func;
		assert_eq!(test_func(vec![1,2,3,4,5]), 3);
	}

	#[test]
	fn test_245() {
		use crate::shortest_word_distance_3::shortest_word_distance as test_func;
		assert_eq!(test_func(vec![
			String::from("practice"),
			String::from("makes"),
			String::from("perfect"),
			String::from("coding"),
			String::from("makes"),
		], String::from("makes"), String::from("coding")), 1);
	}

    #[test]
    fn test_261() {
        use crate::graph_valid_tree_261::valid_tree as test_func;
        assert_eq!(test_func(6, vec![
            vec![0,1],
            vec![0,2],
            vec![0,3],
            vec![0,5],
            vec![2,4]
        ]), true);
    }

    #[test]
    fn test_516() {
        use crate::longest_palindrome_subsequence_516::longest_palindrome_subseq as test_func;
        assert_eq!(test_func(String::from("bbbab")), 4);
        assert_eq!(test_func(String::from("babcbac")), 5);
    }
}
