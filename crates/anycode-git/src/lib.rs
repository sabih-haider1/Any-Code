//! Git status and diff for the workbench (PRD §58, §83). Read-only for Phase 1 — no
//! commit/stage/push here yet, those are gated by approval per docs/SECURITY.md and
//! belong with the agent tool runtime in a later phase.

use serde::Serialize;
use std::path::Path;

#[derive(Debug, thiserror::Error)]
pub enum GitError {
    #[error(transparent)]
    Git(#[from] git2::Error),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum FileStatus {
    Modified,
    Added,
    Deleted,
    Renamed,
    Untracked,
    Conflicted,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StatusEntry {
    pub path: String,
    pub status: FileStatus,
}

/// Current branch name, or `None` for a detached HEAD or an empty repository.
pub fn current_branch(repo_root: &Path) -> Result<Option<String>, GitError> {
    let repo = git2::Repository::open(repo_root)?;
    let result = match repo.head() {
        Ok(head) => Ok(head.shorthand().map(str::to_string)),
        Err(e) if e.code() == git2::ErrorCode::UnbornBranch => Ok(None),
        Err(e) => Err(e.into()),
    };
    result
}

/// Working-tree status, combining the index and the working directory into one entry
/// per path — the workbench shows "what's changed", not staged/unstaged as two lists.
pub fn status(repo_root: &Path) -> Result<Vec<StatusEntry>, GitError> {
    let repo = git2::Repository::open(repo_root)?;
    let mut opts = git2::StatusOptions::new();
    opts.include_untracked(true).recurse_untracked_dirs(true);
    let statuses = repo.statuses(Some(&mut opts))?;

    let mut entries = Vec::with_capacity(statuses.len());
    for entry in statuses.iter() {
        let Some(path) = entry.path() else { continue };
        let flags = entry.status();
        let status = if flags.is_conflicted() {
            FileStatus::Conflicted
        } else if flags.is_wt_new() || flags.is_index_new() {
            FileStatus::Untracked
        } else if flags.is_wt_deleted() || flags.is_index_deleted() {
            FileStatus::Deleted
        } else if flags.is_wt_renamed() || flags.is_index_renamed() {
            FileStatus::Renamed
        } else {
            FileStatus::Modified
        };
        entries.push(StatusEntry {
            path: path.to_string(),
            status,
        });
    }
    Ok(entries)
}

/// HEAD's and the working tree's content for one file, for a diff editor to compare.
/// `None` on either side means "file doesn't exist there" (new or deleted file).
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FileDiff {
    pub head_content: Option<String>,
    pub working_content: Option<String>,
}

pub fn diff_file(repo_root: &Path, relative_path: &str) -> Result<FileDiff, GitError> {
    let repo = git2::Repository::open(repo_root)?;

    let head_content = repo
        .head()
        .ok()
        .and_then(|head| head.peel_to_tree().ok())
        .and_then(|tree| tree.get_path(Path::new(relative_path)).ok())
        .and_then(|entry| repo.find_blob(entry.id()).ok())
        .map(|blob| String::from_utf8_lossy(blob.content()).into_owned());

    let working_content = std::fs::read_to_string(repo_root.join(relative_path)).ok();

    Ok(FileDiff {
        head_content,
        working_content,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::process::Command;

    struct TempRepo(std::path::PathBuf);

    /// Nanosecond timestamps alone can collide: parallel test threads share a process id,
    /// and clock resolution on CI runners isn't guaranteed to be finer than a test's
    /// startup jitter. An atomic counter guarantees uniqueness regardless of clock grain.
    static NEXT_ID: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);

    impl TempRepo {
        fn init() -> Self {
            let n = NEXT_ID.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            let path = std::env::temp_dir().join(format!(
                "anycode-git-test-{}-{}-{n}",
                std::process::id(),
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_nanos()
            ));
            std::fs::create_dir_all(&path).unwrap();
            run(&path, &["init", "-q", "-b", "main"]);
            run(&path, &["config", "user.email", "test@example.com"]);
            run(&path, &["config", "user.name", "Test"]);
            Self(path)
        }
    }

    impl Drop for TempRepo {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.0);
        }
    }

    fn run(dir: &Path, args: &[&str]) {
        let status = Command::new("git")
            .args(args)
            .current_dir(dir)
            .status()
            .unwrap();
        assert!(status.success(), "git {args:?} failed");
    }

    #[test]
    fn reports_untracked_and_modified_files() {
        let repo = TempRepo::init();
        std::fs::write(repo.0.join("a.txt"), "hello\n").unwrap();
        run(&repo.0, &["add", "a.txt"]);
        run(&repo.0, &["commit", "-q", "-m", "init"]);

        std::fs::write(repo.0.join("a.txt"), "hello again\n").unwrap();
        std::fs::write(repo.0.join("b.txt"), "new\n").unwrap();

        let mut entries = status(&repo.0).unwrap();
        entries.sort_by(|a, b| a.path.cmp(&b.path));
        assert_eq!(entries.len(), 2);
        assert_eq!(entries[0].path, "a.txt");
        assert_eq!(entries[0].status, FileStatus::Modified);
        assert_eq!(entries[1].path, "b.txt");
        assert_eq!(entries[1].status, FileStatus::Untracked);

        assert_eq!(current_branch(&repo.0).unwrap(), Some("main".to_string()));
    }

    #[test]
    fn diff_reports_head_and_working_content() {
        let repo = TempRepo::init();
        std::fs::write(repo.0.join("a.txt"), "hello\n").unwrap();
        run(&repo.0, &["add", "a.txt"]);
        run(&repo.0, &["commit", "-q", "-m", "init"]);
        std::fs::write(repo.0.join("a.txt"), "hello again\n").unwrap();

        let diff = diff_file(&repo.0, "a.txt").unwrap();
        assert_eq!(diff.head_content.as_deref(), Some("hello\n"));
        assert_eq!(diff.working_content.as_deref(), Some("hello again\n"));
    }

    #[test]
    fn empty_repository_has_no_branch() {
        let repo = TempRepo::init();
        assert_eq!(current_branch(&repo.0).unwrap(), None);
    }
}
