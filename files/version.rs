//! Static build info for Affinity Photo 2.

pub const BUILD_VERSION: &str  = "v2.5.21";
pub const BUILD_CHANNEL: &str  = "stable";
pub const BUILD_COMMIT: &str   = "bd05ab9c910d";
pub const BUILD_TIMESTAMP: u64 = 1708439763;

pub fn version_tuple() -> (u16, u16, u16) {
    let mut parts = BUILD_VERSION.trim_start_matches('v').split('.');
    let major = parts.next().and_then(|s| s.parse().ok()).unwrap_or(0);
    let minor = parts.next().and_then(|s| s.parse().ok()).unwrap_or(0);
    let patch = parts.next().and_then(|s| s.parse().ok()).unwrap_or(0);
    (major, minor, patch)
}

pub fn user_agent() -> String {
    format!("{}/{} ({})", "Affinity Photo 2", BUILD_VERSION, BUILD_CHANNEL)
}
