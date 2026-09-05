//! Font loading and fallback strategy for SVG rendering.
//!
//! System fonts are loaded once per process and shared across renders.
//! Fonts missing from the SVG are substituted with system fonts while
//! emitting a warning: CJK characters fall back to Microsoft YaHei UI,
//! everything else falls back to the default font families.

use std::collections::HashSet;
use std::sync::{Arc, Mutex, OnceLock};

use resvg::usvg;
use resvg::usvg::fontdb;

/// Font families used when the requested family cannot be found.
const DEFAULT_FAMILIES: &[&str] = &["Times New Roman"];

/// Font families used to substitute missing CJK glyphs, tried in order.
const CJK_FALLBACK_FAMILIES: &[&str] = &["Microsoft YaHei UI", "Microsoft YaHei"];

/// Creates a font resolver that warns about missing fonts and substitutes
/// system fonts.
pub(super) fn font_resolver() -> usvg::FontResolver<'static> {
    usvg::FontResolver {
        select_font: Box::new(select_font),
        select_fallback: Box::new(select_fallback),
    }
}

/// Loads system fonts once and shares the database across all renders.
pub(super) fn system_fontdb() -> Arc<fontdb::Database> {
    static DATABASE: OnceLock<Arc<fontdb::Database>> = OnceLock::new();

    DATABASE
        .get_or_init(|| {
            let mut database = fontdb::Database::new();
            database.load_system_fonts();
            log::debug!("loaded {} font faces from the system", database.len());

            Arc::new(database)
        })
        .clone()
}

fn select_font(font: &usvg::Font, fontdb: &mut Arc<fontdb::Database>) -> Option<fontdb::ID> {
    let families: Vec<fontdb::Family> = font
        .families()
        .iter()
        .map(|family| match family {
            usvg::FontFamily::Serif => fontdb::Family::Serif,
            usvg::FontFamily::SansSerif => fontdb::Family::SansSerif,
            usvg::FontFamily::Cursive => fontdb::Family::Cursive,
            usvg::FontFamily::Fantasy => fontdb::Family::Fantasy,
            usvg::FontFamily::Monospace => fontdb::Family::Monospace,
            usvg::FontFamily::Named(name) => fontdb::Family::Name(name.as_str()),
        })
        .collect();

    let query = fontdb::Query {
        families: &families,
        weight: fontdb::Weight(font.weight()),
        stretch: fontdb::Stretch::from(font.stretch()),
        style: fontdb::Style::from(font.style()),
    };

    if let Some(id) = fontdb.query(&query) {
        return Some(id);
    }

    log::warn!(
        "No match for '{}' font-family, substituting the default font",
        font.families()
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
            .join(", ")
    );

    let mut fallback: Vec<fontdb::Family> = DEFAULT_FAMILIES
        .iter()
        .map(|name| fontdb::Family::Name(name))
        .collect();
    fallback.push(fontdb::Family::Serif);

    fontdb.query(&fontdb::Query {
        families: &fallback,
        ..query
    })
}

fn select_fallback(
    c: char,
    exclude_fonts: &[fontdb::ID],
    fontdb: &mut Arc<fontdb::Database>,
) -> Option<fontdb::ID> {
    if is_cjk(c) {
        for family in CJK_FALLBACK_FAMILIES {
            let id = fontdb.query(&fontdb::Query {
                families: &[fontdb::Family::Name(family)],
                ..fontdb::Query::default()
            });

            if let Some(id) =
                id.filter(|id| has_char(fontdb, *id, c) && !exclude_fonts.contains(id))
            {
                log_missing_glyph(c, family);
                return Some(id);
            }
        }
    }

    (usvg::FontResolver::default_fallback_selector())(c, exclude_fonts, fontdb)
}

/// Warns about a substituted glyph once per character to avoid log flooding
/// on long CJK texts, further occurrences are logged at debug level.
fn log_missing_glyph(c: char, family: &str) {
    static WARNED: OnceLock<Mutex<HashSet<u32>>> = OnceLock::new();

    let warned = WARNED.get_or_init(|| Mutex::new(HashSet::new()));
    let first_time = warned
        .lock()
        .map(|mut warned| warned.insert(u32::from(c)))
        .unwrap_or(true);

    let message = format!(
        "No glyph for U+{:04X} in the selected fonts, substituted '{family}'",
        u32::from(c)
    );

    if first_time {
        log::warn!("{message}");
    } else {
        log::debug!("{message}");
    }
}

/// Checks whether the face supports the character, mirroring the semantics of
/// the character check used inside `usvg` which is not exposed publicly.
fn has_char(fontdb: &fontdb::Database, id: fontdb::ID, c: char) -> bool {
    fontdb
        .with_face_data(id, |font_data, face_index| {
            ttf_parser::Face::parse(font_data, face_index)
                .ok()?
                .glyph_index(c)
        })
        .is_some_and(|glyph| glyph.is_some())
}

/// Checks whether the character belongs to a CJK script (Han, Kana, Hangul
/// and the CJK-specific punctuation and symbol blocks).
fn is_cjk(c: char) -> bool {
    matches!(u32::from(c),
        0x2E80..=0x2EFF      // CJK Radicals Supplement
        | 0x2F00..=0x2FDF    // Kangxi Radicals
        | 0x3000..=0x303F    // CJK Symbols and Punctuation
        | 0x3040..=0x30FF    // Hiragana and Katakana
        | 0x3130..=0x318F    // Hangul Compatibility Jamo
        | 0x3400..=0x4DBF    // CJK Unified Ideographs Extension A
        | 0x4E00..=0x9FFF    // CJK Unified Ideographs
        | 0xAC00..=0xD7AF    // Hangul Syllables
        | 0xF900..=0xFAFF    // CJK Compatibility Ideographs
        | 0xFE30..=0xFE4F    // CJK Compatibility Forms
        | 0xFF00..=0xFFEF    // Halfwidth and Fullwidth Forms
        | 0x20000..=0x2FA1F  // CJK Unified Ideographs Extensions B..F and Supplement
    )
}

#[cfg(test)]
mod tests {
    use super::is_cjk;

    #[test]
    fn cjk_ranges() {
        assert!(is_cjk('中'));
        assert!(is_cjk('あ'));
        assert!(is_cjk('한'));
        assert!(is_cjk('，'));
        assert!(is_cjk('Ａ')); // fullwidth latin
        assert!(!is_cjk('A'));
        assert!(!is_cjk('é'));
        assert!(!is_cjk('😀'));
    }
}
