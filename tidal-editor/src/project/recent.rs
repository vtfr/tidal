use std::path::PathBuf;

use eframe::Storage;
use serde::{Deserialize, Serialize};

use crate::project::project::ProjectState;

#[derive(Serialize, Deserialize)]
pub struct RecentProjectsEntry {
    pub name: String,
    pub project_path_lossy: String,
}

#[derive(Default, Serialize, Deserialize)]
pub struct RecentProjects {
    entries: Vec<RecentProjectsEntry>,
}

impl RecentProjects {
    pub fn add(&mut self, entry: RecentProjectsEntry) {
        self.entries
            .retain(|e| e.project_path_lossy != entry.project_path_lossy);
        self.entries.push(entry);
    }

    pub fn iter(&self) -> impl Iterator<Item = &RecentProjectsEntry> {
        self.entries.iter().rev()
    }

    pub fn store(&self, storage: &mut dyn Storage) {
        let contents = serde_json::to_string(self).unwrap_or_default();

        storage.set_string("recent", contents);
    }

    pub fn load(storage: &dyn Storage) -> Self {
        storage
            .get_string("recent")
            .and_then(|s| serde_json::from_str::<RecentProjects>(&s).ok())
            .unwrap_or_default()
    }

    pub fn serialize(&mut self) -> String {
        serde_json::to_string(self).unwrap()
    }
}
