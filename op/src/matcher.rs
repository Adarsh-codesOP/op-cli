use crate::discovery::App;
use fuzzy_matcher::FuzzyMatcher;
use fuzzy_matcher::skim::SkimMatcherV2;

pub fn match_apps(query: &str, apps: &[App]) -> Vec<App> {
    let matcher = SkimMatcherV2::default();
    let mut matches: Vec<(&App, i64)> = apps
        .iter()
        .filter_map(|app| {
            matcher.fuzzy_match(&app.name, query).map(|score| (app, score))
        })
        .collect();

    matches.sort_by(|a, b| b.1.cmp(&a.1));

    matches.into_iter().map(|(app, _)| app.clone()).collect()
}
