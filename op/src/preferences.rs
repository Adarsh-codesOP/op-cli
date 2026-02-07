use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use crate::discovery::{App, get_cache_path};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default)]
pub struct Preferences {
    pub selections: HashMap<String, String>, // query -> app_path
}

impl Preferences {
    fn get_path() -> Option<PathBuf> {
        if let Some(mut path) = get_cache_path() {
            path.pop(); // Go up to 'op' dir
            path.push("preferences.json");
            return Some(path);
        }
        None
    }

    pub fn load() -> Self {
        if let Some(path) = Self::get_path() {
            if path.exists() {
                if let Ok(file) = fs::File::open(path) {
                    if let Ok(prefs) = serde_json::from_reader(file) {
                        return prefs;
                    }
                }
            }
        }
        Preferences::default()
    }

    pub fn save(&self) {
        if let Some(path) = Self::get_path() {
            if let Ok(file) = fs::File::create(path) {
                let _ = serde_json::to_writer_pretty(file, self);
            }
        }
    }

    pub fn get_preferred_app<'a>(&self, query: &str, apps: &'a [App]) -> Option<&'a App> {
        if let Some(path) = self.selections.get(query) {
            return apps.iter().find(|app| &app.path == path);
        }
        None
    }

    pub fn set_preference(&mut self, query: &str, app: &App) {
        self.selections.insert(query.to_string(), app.path.clone());
        self.save();
    }
}
