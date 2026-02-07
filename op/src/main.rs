mod discovery;
mod matcher;
mod preferences;

use clap::Parser;
use discovery::scan_apps;
use matcher::match_apps;
use preferences::Preferences;
use std::process::exit;
use std::io::{self, Write};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(index = 1)]
    query: Option<String>,

    #[arg(short, long)]
    select: bool,
}

fn main() {
    let args = Args::parse();
    
    let mut prefs = Preferences::load();
    let apps = scan_apps();

    let query = match args.query {
        Some(q) => q,
        None => {
            // If no query, list all apps (debug) or just exit?
            // "Produces minimal, script-safe output"
            // Let's just exit or print help.
            // But for verification, let's print count.
            // Task: "Parses arguments (<query>, -select)"
            // If no query, maybe nothing?
            // Let's print usage.
            println!("Usage: op <query> [-s]");
            return;
        }
    };

    // Check preference (if not in select mode)
    if !args.select {
        if let Some(app) = prefs.get_preferred_app(&query, &apps) {
             launch(app);
             return;
        }
    }

    let matches = match_apps(&query, &apps);

    if matches.is_empty() {
        println!("No apps found matching '{}'", query);
        exit(1);
    }

    if args.select {
        handle_selection(&query, &matches, &mut prefs);
    } else {
        // Auto-select top match
        launch(&matches[0]);
    }
}

fn handle_selection(query: &str, apps: &[discovery::App], prefs: &mut Preferences) {
    println!("Select an application for '{}':", query);
    for (i, app) in apps.iter().take(9).enumerate() {
        // Show the name and vaguely where it is (User vs System)
        // or just the target filename to clarify what it is.
        // For PWAs, target is chrome_proxy.exe, which is confusing.
        // Let's just show the Name.
        println!("[{}] {}", i + 1, app.name);
    }

    print!("> ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    
    if let Ok(choice) = input.trim().parse::<usize>() {
        if choice > 0 && choice <= apps.len().min(9) {
            let app = &apps[choice - 1];
            prefs.set_preference(query, app);
            launch(app);
            return;
        }
    }
    println!("Invalid selection");
    exit(1);
}

fn launch(app: &discovery::App) {
    // Launch the original shortcut (.lnk) to preserve arguments (e.g., PWA app-id),
    // working directory, and icon settings.
    if let Err(e) = open::that_detached(&app.original_path) {
        eprintln!("Failed to launch '{}': {}", app.name, e);
        exit(1);
    }
}
