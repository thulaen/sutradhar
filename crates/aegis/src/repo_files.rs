//! Reading a repository's files from disk, without ever following a
//! link or reading a file too large to check (task #119).

use std::fs;
use std::io::Read;
use std::path::Path;

use super::{MOST_READ, Repo};

impl Repo {
    /// The text of a file, or `None` when it cannot be read.
    #[must_use]
    pub fn read(&self, path: &str) -> Option<String> {
        let bytes = self.read_bytes(path)?;
        Some(String::from_utf8_lossy(&bytes).into_owned())
    }

    /// The bytes of a file, or `None` when it cannot be read. A path that
    /// is, or passes through, a link is never read, so a pushed link
    /// cannot make the gate read a file outside the repository; nor is a
    /// file over [`MOST_READ`] bytes (see [`Repo::too_large`]).
    #[must_use]
    pub fn read_bytes(&self, path: &str) -> Option<Vec<u8>> {
        if self.is_link(path) {
            return None;
        }
        let full = self.content.join(path);
        if fs::symlink_metadata(&full).ok()?.len() > MOST_READ {
            return None;
        }
        // Read under the same cap, so a file that grows after the check
        // is still not read whole.
        let mut bytes = Vec::new();
        let file = fs::File::open(full).ok()?;
        let _read = file
            .take(MOST_READ.saturating_add(1))
            .read_to_end(&mut bytes)
            .ok()?;
        (u64::try_from(bytes.len()).unwrap_or(u64::MAX) <= MOST_READ)
            .then_some(bytes)
    }

    /// True when a file at `path` is there but over [`MOST_READ`] bytes,
    /// so it is not read and must be refused, never skipped.
    #[must_use]
    pub fn too_large(&self, path: &str) -> bool {
        !self.is_link(path)
            && fs::symlink_metadata(self.content.join(path))
                .is_ok_and(|m| m.len() > MOST_READ)
    }

    /// True when the path, or a folder on the way to it, is a link. A
    /// path that climbs out (`..`) or starts at the top (`/`) counts as
    /// one: git never gives such paths, so they are refused, not read.
    #[must_use]
    pub fn is_link(&self, path: &str) -> bool {
        let mut at = self.content.clone();
        Path::new(path).components().any(|part| {
            if !matches!(part, std::path::Component::Normal(_)) {
                return true;
            }
            at.push(part);
            fs::symlink_metadata(&at).is_ok_and(|m| m.file_type().is_symlink())
        })
    }
}
