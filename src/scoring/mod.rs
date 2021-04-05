const SCORING_REPORT_FILE: &str =
	const_format::concatcp!(super::CHIRON_DIR, "report.html");

/// Scoring report generation
pub mod report;
