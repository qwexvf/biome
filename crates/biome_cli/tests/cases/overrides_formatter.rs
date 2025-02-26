use crate::run_cli;
use crate::snap_test::{assert_cli_snapshot, assert_file_contents, SnapshotPayload};
use biome_console::BufferConsole;
use biome_fs::MemoryFileSystem;
use biome_service::DynRef;
use bpaf::Args;
use std::path::Path;

const UNFORMATTED: &str = "  statement(  )  ";
const UNFORMATTED_JSON: &str = r#"{ "asta": ["lorem", "ipsum", "first", "second"] }"#;
const FORMATTED_JSON: &str =
    "{\n    \"asta\": [\n        \"lorem\",\n        \"ipsum\",\n        \"first\",\n        \"second\"\n    ]\n}\n";

const UNFORMATTED_LINE_WIDTH: &str = r#"const a = ["loreum", "ipsum"]"#;
const FORMATTED: &str = "statement();\n";
const FORMATTED_LINE_WIDTH_OVERRIDDEN: &str = "const a = [\n\t\"loreum\",\n\t\"ipsum\",\n];\n";

const FORMATTED_LINE_WITH_SPACES: &str = "const a = [\n  \"loreum\",\n  \"ipsum\",\n];\n";

const FORMATTED_LINE_WIDTH: &str = "const a = [\"loreum\", \"ipsum\"];\n";

const FORMATTED_WITH_SINGLE_QUOTES: &str = "const a = ['loreum', 'ipsum'];\n";
const FORMATTED_WITH_NO_SEMICOLONS: &str = "const a = [\"loreum\", \"ipsum\"]\n";

#[test]
fn does_not_handle_ignored_file() {
    let mut console = BufferConsole::default();
    let mut fs = MemoryFileSystem::default();
    let file_path = Path::new("biome.json");
    fs.insert(
        file_path.into(),
        r#"{
  "files": {
    "include": ["test.js", "special/**"]
  },
  "overrides": [{ "ignore": ["special/**"] }]
}

"#
        .as_bytes(),
    );

    let test = Path::new("test.js");
    fs.insert(test.into(), UNFORMATTED.as_bytes());

    let test2 = Path::new("special/test2.js");
    fs.insert(test2.into(), UNFORMATTED.as_bytes());

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from(
            [
                ("format"),
                ("--write"),
                test.as_os_str().to_str().unwrap(),
                test2.as_os_str().to_str().unwrap(),
            ]
            .as_slice(),
        ),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_file_contents(&fs, test2, UNFORMATTED);
    assert_file_contents(&fs, test, FORMATTED);

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "does_not_handle_ignored_file",
        fs,
        console,
        result,
    ));
}

#[test]
fn does_handle_included_file() {
    let mut console = BufferConsole::default();
    let mut fs = MemoryFileSystem::default();
    let file_path = Path::new("biome.json");
    fs.insert(
        file_path.into(),
        r#"{
  "files": {
    "ignore": ["test.js", "special/**"]
  },
  "overrides": [{ "include": ["special/**"] }]
}

"#
        .as_bytes(),
    );

    let test = Path::new("test.js");
    fs.insert(test.into(), UNFORMATTED.as_bytes());

    let test2 = Path::new("special/test2.js");
    fs.insert(test2.into(), UNFORMATTED.as_bytes());

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from(
            [
                ("format"),
                ("--write"),
                test.as_os_str().to_str().unwrap(),
                test2.as_os_str().to_str().unwrap(),
            ]
            .as_slice(),
        ),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_file_contents(&fs, test2, FORMATTED);
    assert_file_contents(&fs, test, UNFORMATTED);

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "does_handle_included_file",
        fs,
        console,
        result,
    ));
}

#[test]
fn does_handle_included_file_and_disable_formatter() {
    let mut console = BufferConsole::default();
    let mut fs = MemoryFileSystem::default();
    let file_path = Path::new("biome.json");
    fs.insert(
        file_path.into(),
        r#"{
  "files": {
    "include": ["test.js", "special/**"]
  },
  "overrides": [{ "include": ["special/**"], "formatter": { "enabled": false } }]
}

"#
        .as_bytes(),
    );

    let test = Path::new("test.js");
    fs.insert(test.into(), UNFORMATTED.as_bytes());

    let test2 = Path::new("special/test2.js");
    fs.insert(test2.into(), UNFORMATTED.as_bytes());

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from(
            [
                ("format"),
                ("--write"),
                test.as_os_str().to_str().unwrap(),
                test2.as_os_str().to_str().unwrap(),
            ]
            .as_slice(),
        ),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_file_contents(&fs, test2, UNFORMATTED);
    assert_file_contents(&fs, test, FORMATTED);

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "does_handle_included_file_and_disable_formatter",
        fs,
        console,
        result,
    ));
}

#[test]
fn does_include_file_with_different_formatting() {
    let mut console = BufferConsole::default();
    let mut fs = MemoryFileSystem::default();
    let file_path = Path::new("biome.json");
    fs.insert(
        file_path.into(),
        r#"{
  "overrides": [{ "include": ["special/**"], "formatter": { "lineWidth": 20 } }]
}

"#
        .as_bytes(),
    );

    let test = Path::new("test.js");
    fs.insert(test.into(), UNFORMATTED_LINE_WIDTH.as_bytes());

    let test2 = Path::new("special/test2.js");
    fs.insert(test2.into(), UNFORMATTED_LINE_WIDTH.as_bytes());

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from(
            [
                ("format"),
                ("--write"),
                test.as_os_str().to_str().unwrap(),
                test2.as_os_str().to_str().unwrap(),
            ]
            .as_slice(),
        ),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_file_contents(&fs, test2, FORMATTED_LINE_WIDTH_OVERRIDDEN);
    assert_file_contents(&fs, test, FORMATTED_LINE_WIDTH);

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "does_include_file_with_different_formatting",
        fs,
        console,
        result,
    ));
}

#[test]
fn does_include_file_with_different_formatting_and_applies_the_first_one() {
    let mut console = BufferConsole::default();
    let mut fs = MemoryFileSystem::default();
    let file_path = Path::new("biome.json");
    fs.insert(
        file_path.into(),
        r#"{
  "overrides": [
    { "include": ["special/**"], "formatter": { "lineWidth": 20 } },
    { "include": ["special/**"], "formatter": { "lineWidth": 130 } }
   ]
}

"#
        .as_bytes(),
    );

    let test = Path::new("test.js");
    fs.insert(test.into(), UNFORMATTED_LINE_WIDTH.as_bytes());

    let test2 = Path::new("special/test2.js");
    fs.insert(test2.into(), UNFORMATTED_LINE_WIDTH.as_bytes());

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from(
            [
                ("format"),
                ("--write"),
                test.as_os_str().to_str().unwrap(),
                test2.as_os_str().to_str().unwrap(),
            ]
            .as_slice(),
        ),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_file_contents(&fs, test2, FORMATTED_LINE_WIDTH_OVERRIDDEN);
    assert_file_contents(&fs, test, FORMATTED_LINE_WIDTH);

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "does_include_file_with_different_formatting_and_applies_the_first_one",
        fs,
        console,
        result,
    ));
}

#[test]
fn does_include_file_with_different_overrides() {
    let mut console = BufferConsole::default();
    let mut fs = MemoryFileSystem::default();
    let file_path = Path::new("biome.json");
    fs.insert(
        file_path.into(),
        r#"{
  "overrides": [
    { "include": ["test.js"], "formatter": { "lineWidth": 20 } },
    { "include": ["test2.js"], "formatter": { "lineWidth": 20, "indentStyle": "space" } }
   ]
}

"#
        .as_bytes(),
    );

    let test = Path::new("test.js");
    fs.insert(test.into(), UNFORMATTED_LINE_WIDTH.as_bytes());

    let test2 = Path::new("test2.js");
    fs.insert(test2.into(), UNFORMATTED_LINE_WIDTH.as_bytes());

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from(
            [
                ("format"),
                ("--write"),
                test.as_os_str().to_str().unwrap(),
                test2.as_os_str().to_str().unwrap(),
            ]
            .as_slice(),
        ),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_file_contents(&fs, test, FORMATTED_LINE_WIDTH_OVERRIDDEN);
    assert_file_contents(&fs, test2, FORMATTED_LINE_WITH_SPACES);

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "does_include_file_with_different_overrides",
        fs,
        console,
        result,
    ));
}

#[test]
fn does_include_file_with_different_languages() {
    let mut console = BufferConsole::default();
    let mut fs = MemoryFileSystem::default();
    let file_path = Path::new("biome.json");
    fs.insert(
        file_path.into(),
        r#"{
  "overrides": [
    { "include": ["test.js"], "formatter": { "lineWidth": 120 }, "javascript": { "formatter": { "quoteStyle": "single" } } },
    { "include": ["test2.js"], "formatter": { "lineWidth": 120, "indentStyle": "space" }, "javascript": { "formatter": { "semicolons": "asNeeded" } } }
   ]
}

"#
        .as_bytes(),
    );

    let test = Path::new("test.js");
    fs.insert(test.into(), UNFORMATTED_LINE_WIDTH.as_bytes());

    let test2 = Path::new("test2.js");
    fs.insert(test2.into(), UNFORMATTED_LINE_WIDTH.as_bytes());

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from(
            [
                ("format"),
                ("--write"),
                test.as_os_str().to_str().unwrap(),
                test2.as_os_str().to_str().unwrap(),
            ]
            .as_slice(),
        ),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_file_contents(&fs, test, FORMATTED_WITH_SINGLE_QUOTES);
    assert_file_contents(&fs, test2, FORMATTED_WITH_NO_SEMICOLONS);

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "does_include_file_with_different_languages",
        fs,
        console,
        result,
    ));
}

#[test]
fn does_include_file_with_different_languages_and_files() {
    let mut console = BufferConsole::default();
    let mut fs = MemoryFileSystem::default();
    let file_path = Path::new("biome.json");
    fs.insert(
        file_path.into(),
        r#"{
  "overrides": [
    { "include": ["test.js"], "formatter": { "lineWidth": 120 }, "javascript": { "formatter": { "quoteStyle": "single" } } },
    {
        "include": ["test2.js"],
        "formatter": { "lineWidth": 120, "indentStyle": "space" },
        "javascript": { "formatter": { "semicolons": "asNeeded" } },
        "json": { "formatter": { "indentStyle": "space", "lineWidth": 20, "indentWidth": 4 } }
    },
    {
        "include": ["test3.json"],
        "formatter": { "lineWidth": 120, "indentStyle": "space" },
        "json": { "formatter": { "indentStyle": "space", "lineWidth": 20, "indentWidth": 4 } }
    }
  ]
}

"#
            .as_bytes(),
    );

    let test = Path::new("test.js");
    fs.insert(test.into(), UNFORMATTED_LINE_WIDTH.as_bytes());

    let test2 = Path::new("test2.js");
    fs.insert(test2.into(), UNFORMATTED_LINE_WIDTH.as_bytes());

    let json_file = Path::new("test3.json");
    fs.insert(json_file.into(), UNFORMATTED_JSON.as_bytes());

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from(
            [
                ("format"),
                ("--write"),
                test.as_os_str().to_str().unwrap(),
                test2.as_os_str().to_str().unwrap(),
                json_file.as_os_str().to_str().unwrap(),
            ]
            .as_slice(),
        ),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_file_contents(&fs, test, FORMATTED_WITH_SINGLE_QUOTES);
    assert_file_contents(&fs, test2, FORMATTED_WITH_NO_SEMICOLONS);
    assert_file_contents(&fs, json_file, FORMATTED_JSON);

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "does_include_file_with_different_languages_and_files",
        fs,
        console,
        result,
    ));
}
