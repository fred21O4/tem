#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use std::path::PathBuf;

use eframe::egui;

fn main() -> eframe::Result {
    env_logger::init();

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([650.0, 250.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Tem: Track Elite Missions",
        options,
        Box::new(|_cc| Ok(Box::<App>::default())),
    )
}

struct App {
    journal_folder: String,
}

impl Default for App {
    fn default() -> Self {
        let user_dirs = directories::UserDirs::new().expect("Failed to get user folder");
        let mut journal_folder = PathBuf::new();
        journal_folder.push(user_dirs.home_dir());

        #[cfg(target_family = "windows")]
        journal_folder.push(
            ["Saved Games", "Frontier Developments", "Elite Dangerous"]
                .iter()
                .collect(),
        );

        #[cfg(target_family = "unix")]
        journal_folder.push(
            [
                ".local",
                "share",
                "Steam",
                "steamapps",
                "compatdata",
                "359320",
                "pfx",
                "drive_c",
                "users",
                "steamuser",
                "Saved Games",
                "Frontier Developments",
                "Elite Dangerous",
            ]
            .iter()
            .collect::<PathBuf>(),
        );

        // TODO check if folder is valid and if not prompt user to choose with file chooser dialog

        Self {
            journal_folder: journal_folder.to_string_lossy().to_string(),
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label(format!("Journal Folder is: {}", self.journal_folder));
        });
    }
}
