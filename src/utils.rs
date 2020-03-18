/// Build a tuple of coordinates given a their components as Options.
/// This takes care of default initialization
///
/// # Arguments
/// * `level` - A reference to an Option wrapped string
/// * `role` - A reference to an Option wrapped string
/// * `platform` - A reference to an Option wrapped string
/// * `site` - A reference to an Option wrapped String
/// * `mode` - A reference to an Option wrapped string
///
/// # Returns
/// * tuple of strings (level, role, platform, site, mode)
pub fn extract_coords(
    level: Option<String>,
    role: Option<String>,
    platform: Option<String>,
    site: Option<String>,
    mode: Option<String>,
) -> (String, String, String, String, String) {
    let r = role.unwrap_or("any".to_string());
    let l = level.unwrap_or("facility".to_string());
    let p = platform.unwrap_or("any".to_string());
    let s = site.unwrap_or("any".to_string());
    let m = mode.unwrap_or("ancestor".to_string());

    (l, r, p, s, m)
}

/// Truncate a provides &str to a supplied number of characters
///
/// # Arguments
/// * `s` - The incoming str
/// * `max_chars` - The maximum number of characters.
///
/// # Returns
/// * a &str that is, at most `max_chars` long
pub fn truncate(s: &str, max_chars: usize) -> &str {
    match s.char_indices().nth(max_chars) {
        None => s,
        Some((idx, _)) => &s[..idx],
    }
}
