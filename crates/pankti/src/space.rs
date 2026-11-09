//! Free disk space: heavy work does not start when the disk is nearly
//! full, since builds and caches only grow.

/// The free disk space, in bytes, below which heavy work does not start
/// (the owner's floor of 50 GB, set on 9 November 2026). Builds and
/// caches only grow, so the queue stops work before the disk fills.
pub const MIN_FREE: u64 = 50_000_000_000;

/// Reads the free bytes from `df -B1 --output=avail PATH`.
#[must_use]
pub fn free_bytes(df_output: &str) -> Option<u64> {
    df_output.lines().nth(1)?.trim().parse().ok()
}

/// Refuses heavy work when the disk has less than [`MIN_FREE`] free.
///
/// # Errors
/// Says how much is free and what the floor is.
pub fn check_space(free: u64) -> Result<(), String> {
    if free >= MIN_FREE {
        return Ok(());
    }
    Err(format!(
        "only {} GB of disk is free; heavy work starts only with {} GB \
         free, so clear space first",
        free / 1_000_000_000,
        MIN_FREE / 1_000_000_000
    ))
}
