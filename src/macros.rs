macro_rules! score_checks {
	($check:ident, $scored:ident) => {
		// probably should look into rayon for parallel iters here? might be an
		// issue for file locking tho. probably should put async on a todo list
		if let Some(passes) = $check.pass {
			for condition in passes {
				if !matches!(condition.score(), Ok(true)) {
					$scored = $scored && false;
					}
				}
			}

		if let Some(passes) = $check.fail {
			for condition in passes {
				if matches!(condition.score(), Ok(true)) {
					$scored = $scored && false;
					}
				}
			}
	};
}
