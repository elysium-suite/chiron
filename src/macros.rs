/// Used to score checks, `each_cond_contains` will iterate over a (possible)
/// sequence of checks to ensure that their scored condition(s) are met
#[macro_export]
macro_rules! each_cond_contains {
	($impl_iter:expr, $is:literal) => {{
		let mut ret = true;
		if let Some(ref iter) = $impl_iter {
			ret = iter.iter().all(|cond| matches!(cond.score(), Ok($is)))
		}
		ret
	}};
}
