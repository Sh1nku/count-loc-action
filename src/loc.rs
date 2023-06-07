use std::collections::BTreeMap;
use std::error;
use std::path::Path;
use tokei::{Config, LanguageType, Languages, Report};

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct TokeiResult {
    pub language: String,
    pub code: usize,
    pub blanks: usize,
    pub comments: usize,
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct ChildResult {
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
    let mut results = Vec::with_capacity(languages.len() + 1);
    results.push(TokeiResult {
        language: "Total".to_string(),
        code: languages.total().code,
        blanks: languages.total().blanks,
        comments: languages.total().comments,
    });
    results.extend(languages.into_iter().map(|(language, stats)| {
        let child_result = count_child_loc(&stats.children);
        TokeiResult {
            language: language.to_string(),
            code: stats.code + child_result.code,
            blanks: stats.blanks + child_result.blanks,
            comments: stats.comments + child_result.comments,
        }
    }));
    results.sort();
    Ok(results)
}

pub fn count_child_loc(children: &BTreeMap<LanguageType, Vec<Report>>) -> ChildResult {
    let mut code = 0;
    let mut blanks = 0;
    let mut comments = 0;
    for (_, reports) in children {
        for report in reports {
            code += report.stats.code;
            blanks += report.stats.blanks;
            comments += report.stats.comments;
        }
    }
    ChildResult {
        code,
        blanks,
        comments,
    }
}
