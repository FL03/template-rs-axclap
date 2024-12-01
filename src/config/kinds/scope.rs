/*
    Appellation: scope <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use std::path::PathBuf;

fn _default_context() -> Option<String> {
    Some(".".to_string())
}

fn _default_workdir() -> String {
    crate::DEFAULT_WORKDIR.to_string()
}

/// [Scope] stores critical information regarding the applications current position within
/// the filesystem. The context is considered to be the current working directory of the
/// application while the workdir is used to point to the directory where all of the assets
/// are stored.
#[derive(
    Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(default)]
pub struct Scope {
    // The root directory of the service
    pub(crate) context: Option<String>,
    // The directory where all of the assets
    #[serde(default = "_default_workdir")]
    pub(crate) workdir: String,
}

impl Scope {
    pub fn new(workdir: impl ToString) -> Self {
        Self {
            context: None,
            workdir: workdir.to_string(),
        }
    }
    getter!(context: Option<String>, workdir: String);
    set_with!(workdir: String);
    /// converts the scope into a path
    pub fn as_path(&self) -> PathBuf {
        // initialize a new path
        let mut path = PathBuf::new();
        // include the context, if it exists
        self.context().clone().map(|context| path.push(context));
        // add the workdir
        path.push(self.workdir());
        // ensure the path is a directory
        debug_assert!(path.is_dir());
        // return the path
        path
    }
    /// converts the scope into a string
    pub fn as_path_str(&self) -> String {
        self.as_path().display().to_string()
    }
    /// sets the current working directory to the scope
    pub fn set_cwd(&self) {
        std::env::set_current_dir(self.as_path()).unwrap();
    }
    /// sets the context of the scope
    pub fn set_context(&mut self, context: impl ToString) {
        self.context = Some(context.to_string());
    }
    /// if the
    pub fn set_some_context(&mut self, rhs: Option<impl ToString>) {
        rhs.map(|data| self.context = Some(data.to_string()));
    }

    pub fn set_some_workdir(&mut self, rhs: Option<impl ToString>) {
        rhs.map(|data| self.workdir = data.to_string());
    }
}

impl Default for Scope {
    fn default() -> Self {
        Self {
            context: None,
            workdir: "dist".into(),
        }
    }
}

impl core::fmt::Display for Scope {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{path}", path = self.as_path().display())
    }
}

impl core::str::FromStr for Scope {
    type Err = std::io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::new(s))
    }
}
