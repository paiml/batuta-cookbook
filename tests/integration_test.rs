//! Integration tests for the cookbook

use batuta_cookbook::{Analyzer, Result};

#[test]
fn test_analyzer_integration() -> Result<()> {
    let analyzer = Analyzer::new(".");
    let report = analyzer.analyze()?;

    assert!(report.file_count > 0);
    assert!(report.total_lines > 0);

    Ok(())
}

#[test]
fn test_analyzer_with_tdg() -> Result<()> {
    let analyzer = Analyzer::new(".");
    let report = analyzer.analyze_with_tdg()?;

    assert!(report.tdg_score.is_some());

    let tdg = report.tdg();
    assert!(tdg.score >= 0.0);
    assert!(tdg.score <= 100.0);

    Ok(())
}

#[test]
fn test_all_examples_compile() {
    // This test ensures our example recipes exist and can be compiled
    // The actual compilation is done by CI, this is a sanity check

    let example_dir = std::path::Path::new("examples");
    if example_dir.exists() {
        let count = std::fs::read_dir(example_dir)
            .unwrap()
            .filter(|e| {
                e.as_ref()
                    .unwrap()
                    .path()
                    .extension()
                    .and_then(|s| s.to_str())
                    == Some("rs")
            })
            .count();

        assert!(count >= 2, "Should have at least 2 recipe examples");
    }
}
