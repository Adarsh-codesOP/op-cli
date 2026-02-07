use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use walkdir::WalkDir;
use lnk::ShellLink;
use dirs;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct App {
    pub name: String,
    pub path: String, // The actual executable path
    pub original_path: String, // The .lnk file path
}

pub fn get_start_menu_paths() -> Vec<PathBuf> {
    let mut paths = vec![];
    
    // System Start Menu
    paths.push(PathBuf::from(r"C:\ProgramData\Microsoft\Windows\Start Menu\Programs"));
    
    // User Start Menu
    if let Some(data_dir) = dirs::data_dir() {
        paths.push(data_dir.join(r"Microsoft\Windows\Start Menu\Programs"));
    }
    
    paths
}

use std::panic;
use encoding_rs;

pub fn resolve_lnk(path: &Path) -> Option<String> {
    let path_buf = path.to_path_buf();
    let result = panic::catch_unwind(move || {
        ShellLink::open(&path_buf, encoding_rs::WINDOWS_1252)
    });

    match result {
        Ok(Ok(link)) => {
            // Try explicit link target from LinkInfo
            if let Some(target) = link.link_target() {
                return Some(target);
            }
            // Fallback to relative path in StringData
            if let Some(rel) = link.string_data().relative_path() {
                return Some(rel.to_string());
            }
            None
        },
        _ => None, // Panic or Error
    }
}

use std::fs;

pub fn get_cache_path() -> Option<PathBuf> {
    if let Some(mut path) = dirs::data_local_dir() {
        path.push("op");
        fs::create_dir_all(&path).ok()?;
        path.push("cache.json");
        return Some(path);
    }
    None
}

pub fn load_cache() -> Option<Vec<App>> {
    let cache_path = get_cache_path()?;
    if !cache_path.exists() {
        return None;
    }

    // Check invalidation based on directory mtime
    if let Ok(metadata) = fs::metadata(&cache_path) {
        if let Ok(cache_mtime) = metadata.modified() {
            for start_menu in get_start_menu_paths() {
                 if let Ok(dir_meta) = fs::metadata(&start_menu) {
                     if let Ok(dir_mtime) = dir_meta.modified() {
                         if dir_mtime > cache_mtime {
                             return None; // Cache is stale
                         }
                     }
                 }
            }
        }
    }

    let file = fs::File::open(cache_path).ok()?;
    serde_json::from_reader(file).ok()
}

pub fn save_cache(apps: &[App]) {
    if let Some(cache_path) = get_cache_path() {
        if let Ok(file) = fs::File::create(cache_path) {
            let _ = serde_json::to_writer(file, apps);
        }
    }
}

pub fn scan_apps() -> Vec<App> {
    if let Some(cached_apps) = load_cache() {
        return cached_apps;
    }

    let mut apps = Vec::new();
    let paths = get_start_menu_paths();

    for start_menu in &paths {
        if !start_menu.exists() {
            continue;
        }

        for entry in WalkDir::new(start_menu).into_iter().filter_map(|e| e.ok()) {
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("lnk") {
                if let Some(target) = resolve_lnk(path) {
                     if !target.to_lowercase().ends_with(".exe") {
                         continue;
                     }
                     
                     let name = path.file_stem().and_then(|s| s.to_str()).unwrap_or("Unknown").to_string();
                     apps.push(App {
                         name,
                         path: target,
                         original_path: path.to_string_lossy().to_string(),
                     });
                }
            }
        }
    }
    
    save_cache(&apps);
    apps
}
