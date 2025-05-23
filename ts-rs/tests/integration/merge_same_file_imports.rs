use std::fmt::Write;

use serde::de::Expected;
use ts_rs::TS;

#[derive(TS)]
#[ts(export_to = "merge_same_file_imports/a.ts")]
pub struct EditProfile {
    pub name: Option<String>,

    pub game_version: Option<String>,
    pub loader: Option<ModLoader>,

    pub hooks: Option<Hooks>,
    pub extra: Extra,
}

#[derive(TS)]
#[ts(rename_all = "lowercase", export_to = "merge_same_file_imports/b.ts")]
pub enum ModLoader {
    Vanilla,
    Forge,
    Fabric,
    Quilt,
    NeoForge,
}

#[derive(TS)]
#[ts(export_to = "merge_same_file_imports/b.ts")]
pub struct Hooks {
    pub pre_launch: Option<String>,
    pub wrapper: Option<String>,
    pub post_exit: Option<String>,
}

#[derive(TS)]
#[ts(export_to = "merge_same_file_imports/c.ts")]
pub struct Extra {
    pub foo: i32,
}

#[test]
fn merge_same_file_imports() {
    EditProfile::export_all().unwrap();
    let text = std::fs::read_to_string(EditProfile::default_output_path().unwrap()).unwrap();

    let extension = if cfg!(feature = "import-esm") {
        ".js"
    } else {
        ""
    };

    let mut expected = String::with_capacity(text.len());
    writeln!(expected, "// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.").unwrap();
    writeln!(
        expected,
        r#"import type {{ Hooks, ModLoader }} from "./b{extension}";"#
    )
    .unwrap();
    writeln!(
        expected,
        r#"import type {{ Extra }} from "./c{extension}";"#
    )
    .unwrap();
    writeln!(expected);

    if cfg!(feature = "format") {
        writeln!(expected, "export type EditProfile = {{").unwrap();
        writeln!(expected, "  name: string | null;").unwrap();
        writeln!(expected, "  game_version: string | null;").unwrap();
        writeln!(expected, "  loader: ModLoader | null;").unwrap();
        writeln!(expected, "  hooks: Hooks | null;").unwrap();
        writeln!(expected, "  extra: Extra;").unwrap();
        writeln!(expected, "}};").unwrap();
    } else {
        writeln!(expected, "export {}", EditProfile::decl()).unwrap();
    }

    assert_eq!(text, expected)
}
