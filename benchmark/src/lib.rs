/// A parsed fixture file.
///
/// Fixtures may contain git-conflict-style edit annotations:
///
/// ```text
/// @prefix ex: <http://example.org/> .
///
/// <<<<<<< before
/// ex:alice ex:age 30 .
/// =======
/// ex:alice ex:age 31 .
/// >>>>>>> after
/// ```
///
/// Lines outside any marker block are included verbatim in both states.
/// Multiple conflict sections per file are all applied simultaneously.
/// A file with no conflict markers is a static fixture (`is_static == true`).
#[derive(Debug, Clone)]
pub struct Fixture {
    pub name: String,
    /// The "before" state of the document (original text).
    pub before: String,
    /// The "after" state of the document (edited text).
    pub after: String,
    /// True when the file contains no conflict markers (before == after).
    pub is_static: bool,
}

impl Fixture {
    /// Parse fixture content from a string.
    pub fn parse(name: impl Into<String>, content: &str) -> Self {
        let name = name.into();
        let mut before = String::new();
        let mut after = String::new();
        let mut has_markers = false;

        #[derive(PartialEq)]
        enum Section {
            Shared,
            Before,
            After,
        }

        let mut section = Section::Shared;

        for line in content.lines() {
            if line.starts_with("<<<<<<< before") {
                has_markers = true;
                section = Section::Before;
            } else if line == "=======" && section == Section::Before {
                section = Section::After;
            } else if line.starts_with(">>>>>>> after") && section == Section::After {
                section = Section::Shared;
            } else {
                match section {
                    Section::Shared => {
                        before.push_str(line);
                        before.push('\n');
                        after.push_str(line);
                        after.push('\n');
                    }
                    Section::Before => {
                        before.push_str(line);
                        before.push('\n');
                    }
                    Section::After => {
                        after.push_str(line);
                        after.push('\n');
                    }
                }
            }
        }

        Fixture {
            name,
            before,
            after,
            is_static: !has_markers,
        }
    }

    /// Load a fixture from a file path.
    pub fn from_file(path: &std::path::Path) -> Self {
        let name = path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("unknown")
            .to_string();
        let content = std::fs::read_to_string(path)
            .unwrap_or_else(|e| panic!("failed to read fixture {}: {}", path.display(), e));
        Self::parse(name, &content)
    }
}

/// Load all fixtures with the given extension from a directory.
pub fn load_fixtures_ext(fixtures_dir: &str, ext: &str) -> Vec<Fixture> {
    let dir = std::path::Path::new(fixtures_dir);
    let mut fixtures: Vec<Fixture> = std::fs::read_dir(dir)
        .unwrap_or_else(|e| panic!("failed to read fixtures dir {}: {}", dir.display(), e))
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let path = entry.path();
            if path.extension()?.to_str()? == ext {
                Some(Fixture::from_file(&path))
            } else {
                None
            }
        })
        .collect();
    fixtures.sort_by(|a, b| a.name.cmp(&b.name));
    fixtures
}

/// Load all `.ttl` fixtures from a directory.
pub fn load_fixtures(fixtures_dir: &str) -> Vec<Fixture> {
    load_fixtures_ext(fixtures_dir, "ttl")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_static_fixture() {
        let content = "@prefix ex: <http://example.org/> .\nex:alice ex:age 30 .\n";
        let fix = Fixture::parse("test", content);
        assert!(fix.is_static);
        assert_eq!(fix.before, fix.after);
        assert!(fix.before.contains("ex:alice ex:age 30 ."));
    }

    #[test]
    fn test_single_conflict_section() {
        let content = "\
@prefix ex: <http://example.org/> .
<<<<<<< before
ex:alice ex:age 30 .
=======
ex:alice ex:age 31 .
>>>>>>> after
";
        let fix = Fixture::parse("test", content);
        assert!(!fix.is_static);
        assert!(fix.before.contains("ex:age 30"));
        assert!(!fix.before.contains("ex:age 31"));
        assert!(fix.after.contains("ex:age 31"));
        assert!(!fix.after.contains("ex:age 30"));
        // Shared header in both
        assert!(fix.before.contains("@prefix ex:"));
        assert!(fix.after.contains("@prefix ex:"));
    }

    #[test]
    fn test_multiple_conflict_sections() {
        let content = "\
@prefix ex: <http://example.org/> .
<<<<<<< before
ex:alice ex:age 30 .
=======
ex:alice ex:age 31 .
>>>>>>> after
<<<<<<< before
ex:bob foaf:name \"Bob\" .
=======
ex:bob foaf:name \"Robert\" .
>>>>>>> after
";
        let fix = Fixture::parse("test", content);
        assert!(!fix.is_static);
        assert!(fix.before.contains("ex:age 30"));
        assert!(fix.before.contains("\"Bob\""));
        assert!(fix.after.contains("ex:age 31"));
        assert!(fix.after.contains("\"Robert\""));
    }

    #[test]
    fn test_shared_lines_appear_in_both() {
        let content = "\
@prefix ex: <http://example.org/> .
ex:shared ex:prop \"unchanged\" .
<<<<<<< before
ex:item ex:val 1 .
=======
ex:item ex:val 2 .
>>>>>>> after
ex:also ex:shared true .
";
        let fix = Fixture::parse("test", content);
        assert!(fix.before.contains("ex:shared ex:prop \"unchanged\""));
        assert!(fix.after.contains("ex:shared ex:prop \"unchanged\""));
        assert!(fix.before.contains("ex:also ex:shared true"));
        assert!(fix.after.contains("ex:also ex:shared true"));
    }
}
