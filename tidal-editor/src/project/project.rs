use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use tidal_core::graph::Graph;

#[derive(Debug, Clone)]
pub struct ProjectStorageDetails {
    pub path: PathBuf,
    pub root_path: PathBuf,
}

#[derive(Serialize, Deserialize)]
pub struct ProjectState {
    pub graph: Graph,
}

#[derive(Serialize, Deserialize)]
pub struct Project {
    #[serde(skip)]
    pub storage_details: Option<ProjectStorageDetails>,

    pub state: ProjectState,
}

#[derive(thiserror::Error, Debug)]
pub enum ProjectError {
    #[error("project must be resided in an directory")]
    MissingRoot,
    #[error("io error")]
    IoError(#[from] std::io::Error),
    #[error("serialization error")]
    SerializationError(#[from] serde_json::Error),
    #[error("missing storage details")]
    MissingStorageDetails,
}

impl Project {
    pub fn new(state: ProjectState) -> Self {
        Self {
            storage_details: None,
            state,
        }
    }

    pub fn load(path: &Path) -> Result<Self, ProjectError> {
        let root_path = path.parent().ok_or(ProjectError::MissingRoot)?;

        let file = File::open(path)?;
        let buffer = BufReader::new(file);

        let state: ProjectState = serde_json::from_reader(buffer)?;

        let storage_details = ProjectStorageDetails {
            root_path: root_path.into(),
            path: path.into(),
        };

        Ok(Self {
            storage_details: Some(storage_details),
            state,
        })
    }

    pub fn set_path(&mut self, path: &Path) -> Result<(), ProjectError> {
        let root_path = path.parent().ok_or(ProjectError::MissingRoot)?;

        self.storage_details = Some(ProjectStorageDetails {
            root_path: root_path.into(),
            path: path.into(),
        });
        Ok(())
    }

    pub fn save(&self) -> Result<(), ProjectError> {
        let storage_details = self
            .storage_details
            .as_ref()
            .ok_or(ProjectError::MissingStorageDetails)?;

        let file = File::create(&storage_details.path)?;
        let writer = BufWriter::new(file);

        serde_json::to_writer(writer, &self.state)?;

        Err(ProjectError::MissingStorageDetails)
    }

    #[inline]
    pub fn has_set_storage_details(&self) -> bool {
        self.storage_details.is_some()
    }
}
