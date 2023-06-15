use crate::formatting::{abbreviate_format_number, pretty_format_number};
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

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct OutputResult {
    pub language: String,
    pub code: usize,
    pub code_abbreviated: String,
    pub code_pretty: String,
    pub blanks: usize,
    pub blanks_abbreviated: String,
    pub blanks_pretty: String,
    pub comments: usize,
    pub comments_abbreviated: String,
    pub comments_pretty: String,
}

impl From<TokeiResult> for OutputResult {
    fn from(result: TokeiResult) -> Self {
        OutputResult {
            language: result.language,
            code: result.code,
            code_abbreviated: abbreviate_format_number(result.code),
            code_pretty: pretty_format_number(result.code),
            blanks: result.blanks,
            blanks_abbreviated: abbreviate_format_number(result.blanks),
            blanks_pretty: pretty_format_number(result.blanks),
            comments: result.comments,
            comments_abbreviated: abbreviate_format_number(result.comments),
            comments_pretty: pretty_format_number(result.comments),
        }
    }
}

pub fn get_loc<T: AsRef<Path>>(
    paths: &[T],
    excluded: &[&str],
) -> Result<Vec<OutputResult>, Box<dyn error::Error>> {
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
    Ok(results.into_iter().map(|x| x.into()).collect())
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
