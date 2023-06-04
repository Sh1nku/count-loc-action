use std::error;
use std::path::Path;
use tokei::{Config, Languages};

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct TokeiResult {
    pub language: String,
    pub code: usize,
    pub blanks: usize,
    pub comments: usize,
}

pub fn get_loc<T: AsRef<Path>>(
    paths: &[T],
    excluded: &[&str],
) -> Result<Vec<TokeiResult>, Box<dyn error::Error>> {
    let config = Config::default();
    let mut languages = Languages::new();
    languages.get_statistics(paths, excluded, &config);
    let mut results = Vec::with_capacity(languages.len());
    results.push(TokeiResult {
        language: "Total".to_string(),
        code: languages.total().code,
        blanks: languages.total().blanks,
        comments: languages.total().comments,
    });
    results.extend(languages.into_iter().map(|(language, stats)| TokeiResult {
        language: language.to_string(),
        code: stats.code,
        blanks: stats.blanks,
        comments: stats.comments,
    }));
    results.sort();
    Ok(results)
}
