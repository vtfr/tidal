use eframe::egui::Context;

use crate::project::project::Project;

#[derive(Debug, Default)]
pub struct ProjectSaveDialog {
    is_opened: bool,
    path: String,
    last_error: Option<String>,
}

impl ProjectSaveDialog {
    pub fn open(&mut self) {
        self.is_opened = true
    }

    pub fn show(&mut self, ctx: &Context, project: &mut Project) {
        if !self.is_opened {
            return;
        }

        // if !project.has_set_storage_details() {
        if let Some(path) = rfd::FileDialog::new().pick_folder() {
            if let Err(err) = project
                .set_path(&path.join("tidal.json"))
                .and_then(|_| project.save())
            {}
        };

        self.is_opened = false;
        // }
    }
}
