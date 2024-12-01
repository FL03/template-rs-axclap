/*
    Appellation: workspace <config>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::config::Scope;
use std::path::PathBuf;

fn _application() -> String {
    std::env::current_exe()
        .map(|path| path.display().to_string())
        .unwrap_or(crate::DEFAULT_APPLICATION.to_string())
}

fn _application_option() -> Option<String> {
    Some(_application())
}

fn _artifacts() -> String {
    crate::DEFAULT_DIR_ARTIFACTS.to_string()
}

fn _default_scope() -> Scope {
    Scope::new(crate::DEFAULT_WORKDIR)
}

fn _default_context() -> Option<String> {
    Some(".".to_string())
}

fn _default_workdir() -> PathBuf {
    std::env::current_dir().unwrap_or(crate::DEFAULT_WORKDIR.into())
}

/// [Scope] is a structure containing all of the information required for the service to operate.
#[derive(
    Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(default)]
pub struct WorkspaceConfig {
    /// the path to the executable
    #[serde(default = "_application")]
    pub(crate) application: String,
    /// the path to the directory used to store any artifacts
    #[serde(default = "_artifacts")]
    pub(crate) artifacts: String,
    /// a path to another build-script
    pub(crate) build: Option<String>,
    // The root directory of the service
    #[serde(default = "_default_workdir")]
    pub(crate) workdir: PathBuf,
}

impl WorkspaceConfig {
    pub fn new<T>(workdir: T) -> Self
    where
        PathBuf: From<T>,
    {
        Self {
            application: _application(),
            artifacts: _artifacts(),
            build: None,
            workdir: workdir.into(),
        }
    }

    getter!(application: String, artifacts: String, build: Option<String>, workdir: PathBuf);

    set_with!(artifacts: String, application: String);

    set_option!(application: String, artifacts: String);

    set_some!(build::<String>);
    /// change the current directory to the workspace
    pub fn set_current_dir(&self) {
        let path = self.workdir();
        debug_assert!(self.is_workdir_valid());
        tracing::info!("setting current directory to: {p}", p = path.display());
        std::env::set_current_dir(path).unwrap();
    }
    /// set the working directory of the scope
    pub fn set_workdir<T>(&mut self, workdir: T)
    where
        PathBuf: From<T>,
    {
        self.workdir = workdir.into();
    }
    /// if the workdir is set, set it to the given workdir
    pub fn set_workdir_option<T>(&mut self, workdir: Option<T>)
    where
        PathBuf: From<T>,
    {
        workdir.map(|w| self.set_workdir(w));
    }
    /// check if the workdir is valid
    pub fn is_workdir_valid(&self) -> bool {
        self.workdir().is_dir()
    }
    /// get the path to the application binary; if unspecified, the current executable is used
    /// otherwise, the path is assumed to be within the workspaces current directory.
    pub fn path_to_application(&self) -> PathBuf {
        let path = if self.application().is_empty() {
            std::env::current_exe().expect("unable to determine the location of the executable")
        } else {
            self.workdir().join(self.application())
        };
        // ensure the path is a file
        debug_assert!(path.is_file());
        // return the path
        path
    }
    /// get the path to the artifacts directory; the artifacts directory assumed to be a
    /// subdirectory of the workspace and is used to store various build artifacts, logs,
    /// temporary files, etc.
    pub fn path_to_artifacts(&self) -> PathBuf {
        self.workdir().join(self.artifacts())
    }
}

impl Default for WorkspaceConfig {
    fn default() -> Self {
        Self::new(crate::DEFAULT_WORKDIR)
    }
}
