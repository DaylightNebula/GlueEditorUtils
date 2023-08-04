use std::fs::{ReadDir, DirEntry, self};
use egui::*;

pub fn render_file_list(ui: &mut egui::Ui, root: ReadDir) {
    // render scroll area
    egui::ScrollArea::both().show(ui, |ui| {
        // render file contents
        render_file_contents(ui, root)
    });
}

fn render_file_contents(ui: &mut egui::Ui, root: ReadDir) {
    // divide children of root file into directories and files
    let (dirs, files): (Vec<_>, Vec<_>) = root.into_iter()
        .map(|p| { p.unwrap() })
        .partition(|entry| { entry.file_type().unwrap().is_dir() });

    // draw directories
    for dir in dirs {
        render_directory(ui, dir)
    }

    // draw files
    for file in files {
        render_file_button(ui, file)
    }
}

fn render_directory(ui: &mut egui::Ui, dir: DirEntry) {
    // render collapsable to represent files and add contents to it
    ui.collapsing(dir.file_name().to_str().unwrap(), |ui| {
        render_file_contents(ui, fs::read_dir(dir.path()).unwrap())
    });
}

fn render_file_button(ui: &mut egui::Ui, file: DirEntry) {
    // render button for file
    ui.horizontal(|ui| {
        // build a button
        let label = egui::Label::new(file.file_name().to_str().unwrap()).sense(Sense::click());
        let response = ui.add(label);

        // check if button was clicked
        if response.clone().clicked() {
            println!("Left click");
        }

        response.context_menu(|ui| {
            if ui.button("Delete").clicked() {
                let remove_result = fs::remove_file(file.path());
                if remove_result.is_ok() { println!("Remove completed successfully!") } else { println!("Remove error: {}", remove_result.err().unwrap()) }
                ui.close_menu();
            }
        });
    });
}