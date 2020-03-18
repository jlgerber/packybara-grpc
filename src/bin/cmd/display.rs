use log;
use packybara::db::find::versionpins::FindVersionPinsRow;
use packybara::db::find_all::versionpins::FindAllVersionPinsRow;
use prettytable::{cell, format, row, table};
use serde_json;
/// Truncate a provides &str to a supplied number of characters
///
/// # Arguments
/// * `s` - The incoming str
/// * `max_chars` - The maximum number of characters.
///
/// # Returns
/// * a &str that is, at most `max_chars` long
#[inline]
pub(super) fn truncate(s: &str, max_chars: usize) -> &str {
    match s.char_indices().nth(max_chars) {
        None => s,
        Some((idx, _)) => &s[..idx],
    }
}

pub(crate) fn versionpin(response: FindVersionPinsRow, full_withs: bool, json: bool) {
    if json {
        let serialized =
            serde_json::to_string_pretty(&response).expect("unable to unwrap response");
        println!("{}", serialized);
    } else {
        let withs = response.withs.unwrap_or(Vec::new());
        let withs = if withs.len() > 0 {
            if full_withs {
                format!("[{}]", withs.join(","))
            } else {
                format!("[{}...]", truncate(withs.join(",").as_ref(), 40))
            }
        } else {
            "[]".to_string()
        };
        let mut table =
            table!([bFg => "PIN ID", "DISTRIBUTION", "ROLE", "LEVEL", "PLATFORM", "SITE", "WITHS"]);
        table.add_row(row![
            response.versionpin_id,
            response.distribution,
            response.coords.role,
            response.coords.level,
            response.coords.platform,
            response.coords.site,
            withs,
        ]);

        table.set_format(*format::consts::FORMAT_CLEAN); //FORMAT_NO_LINESEP_WITH_TITLE  FORMAT_NO_BORDER_LINE_SEPARATOR
        table.printstd();
    }
}

pub(crate) fn versionpins(response: Vec<FindAllVersionPinsRow>, full_withs: bool) {
    let mut table =
        table!([bFg => "PIN ID", "DISTRIBUTION", "ROLE", "LEVEL", "PLATFORM", "SITE", "WITHS"]);
    for response in response {
        let withs = response.withs.unwrap_or(Vec::new());
        let withs = if withs.len() > 0 {
            if full_withs {
                format!("[{}]", withs.join(","))
            } else {
                format!("[{}...]", truncate(withs.join(",").as_ref(), 40))
            }
        } else {
            "[]".to_string()
        };
        table.add_row(row![
            response.versionpin_id,
            response.distribution,
            response.coords.role,
            response.coords.level,
            response.coords.platform,
            response.coords.site,
            withs,
        ]);
    }
    table.set_format(*format::consts::FORMAT_CLEAN); //FORMAT_NO_LINESEP_WITH_TITLE  FORMAT_NO_BORDER_LINE_SEPARATOR
    table.printstd();
}
