
use serde::{Deserialize, Serialize};

/// Data structure containing trusted publisher information extracted from JWT claims
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "provider")]
pub enum TrustpubData {
    #[serde(rename = "github")]
    GitHub {
        /// Repository (e.g. "octo-org/octo-repo")
        repository: String,
        /// Workflow run ID
        run_id: String,
        /// SHA of the commit
        sha: String,
    },
    #[serde(rename = "gitlab")]
    GitLab {
        /// Project path (e.g. "rust-lang/cargo")
        project_path: String,
        /// Job ID
        job_id: String,
        /// SHA of the commit
        sha: String,
    },
}
