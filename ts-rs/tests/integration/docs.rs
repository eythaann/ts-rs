#![allow(dead_code)]

use std::{concat, fs};

use ts_rs::TS;

/// Doc comment.
/// Supports new lines.
///
/// Testing
#[derive(TS)]
#[ts(export_to = "docs/")]
struct A {
    /// Doc of field
    ///
    /// Testing
    name: String,
}

#[derive(TS)]
#[ts(export_to = "docs/")]
/// Doc comment.
/// Supports new lines.
///
/// Testing
struct B {
    /// Doc of field
    ///
    /// Testing
    name: String,
}

#[derive(TS)]
#[ts(export_to = "docs/")]
/// Doc comment.
/// Supports new lines.
///
/// Testing
struct C {}

#[derive(TS)]
#[ts(export_to = "docs/")]
/// Doc comment.
/// Supports new lines.
///
/// Testing
struct D;

#[derive(TS)]
#[ts(export_to = "docs/")]
/// Doc comment.
/// Supports new lines.
///
/// Testing
enum E {}

#[derive(TS)]
#[ts(export_to = "docs/")]
/// Doc comment.
/// Supports new lines.
///
/// Testing
enum F {
    /// Doc of variant
    ///
    /// Testing
    VarA,
    /// Doc of variant
    ///
    /// Testing
    VarB(),
    /// Doc of variant
    ///
    /// Testing
    VarC {
        /// Doc of field of variant
        ///
        /// Testing
        variant_field: i32,
    },
}

#[derive(TS)]
#[ts(export_to = "docs/")]
struct G {
    /// Docs
    some_other_field: i32,

    /// Some documentation that should probably NOT be exported
    #[ts(flatten)]
    f: F,
}

#[derive(TS)]
#[ts(export_to = "docs/")]
/**
 * Block doc comment
 *
 * works
 */
struct H {
    foo: i32,
}

#[derive(TS)]
#[ts(export_to = "docs/")]
#[doc = concat!("line ", line!())]
struct I {
    #[doc = concat!("column ", column!())]
    a: i32,

    #[doc = concat!("path ", module_path!())]
    b: i32,
}

#[test]
fn export_a() {
    A::export().unwrap();

    let expected_content = if cfg!(feature = "format") {
        concat!(
            "// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.\n\n",
            "/**\n",
            " * Doc comment.\n",
            " * Supports new lines.\n",
            " *\n",
            " * Testing\n",
            " */\n",
            "export type A = {\n",
            "  /**\n",
            "   * Doc of field\n",
            "   *\n",
            "   * Testing\n",
            "   */\n",
            "  name: string;\n",
            "};\n",
        )
    } else {
        concat!(
            "// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.\n\n",
            "/**\n",
            " * Doc comment.\n",
            " * Supports new lines.\n",
            " *\n",
            " * Testing\n",
            " */\n",
            "export type A = { \n",
            "/**\n",
            " * Doc of field\n",
            " *\n",
            " * Testing\n",
            " */\n",
            "name: string, };",
            "\n",
        )
    };

    let actual_content = fs::read_to_string(A::default_output_path().unwrap()).unwrap();

    assert_eq!(actual_content, expected_content);
}

#[test]
fn export_b() {
    B::export().unwrap();

    let expected_content = if cfg!(feature = "format") {
        concat!(
            "// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.\n\n",
            "/**\n",
            " * Doc comment.\n",
            " * Supports new lines.\n",
            " *\n",
            " * Testing\n",
            " */\n",
            "export type B = {\n",
            "  /**\n",
            "   * Doc of field\n",
            "   *\n",
            "   * Testing\n",
            "   */\n",
            "  name: string;\n",
            "};\n",
        )
    } else {
        concat!(
            "// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.\n\n",
            "/**\n",
            " * Doc comment.\n",
            " * Supports new lines.\n",
            " *\n",
            " * Testing\n",
            " */\n",
            "export type B = { \n",
            "/**\n",
            " * Doc of field\n",
            " *\n",
            " * Testing\n",
            " */\n",
            "name: string, };",
            "\n",
        )
    };

    let actual_content = fs::read_to_string(B::default_output_path().unwrap()).unwrap();

    assert_eq!(actual_content, expected_content);
}

#[test]
fn export_c() {
    C::export().unwrap();

    let expected_content = if cfg!(feature = "format") {
        concat!(
            "// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.\n\n",
            "/**\n",
            " * Doc comment.\n",
            " * Supports new lines.\n",
            " *\n",
            " * Testing\n",
            " */\n",
            "export type C = Record<string, never>;\n",
        )
    } else {
        concat!(
            "// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.\n\n",
            "/**\n",
            " * Doc comment.\n",
            " * Supports new lines.\n",
            " *\n",
            " * Testing\n",
            " */\n",
            "export type C = Record<string, never>;",
            "\n",
        )
    };

    let actual_content = fs::read_to_string(C::default_output_path().unwrap()).unwrap();

    assert_eq!(actual_content, expected_content);
}

#[test]
fn export_d() {
    D::export().unwrap();

    let expected_content = if cfg!(feature = "format") {
        concat!(
            "// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.\n\n",
            "/**\n",
            " * Doc comment.\n",
            " * Supports new lines.\n",
            " *\n",
            " * Testing\n",
            " */\n",
            "export type D = null;\n",
        )
    } else {
        concat!(
            "// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.\n\n",
            "/**\n",
            " * Doc comment.\n",
            " * Supports new lines.\n",
            " *\n",
            " * Testing\n",
            " */\n",
            "export type D = null;",
            "\n",
        )
    };
    let actual_content = fs::read_to_string(D::default_output_path().unwrap()).unwrap();

    assert_eq!(actual_content, expected_content);
}

#[test]
fn export_e() {
    E::export().unwrap();

    let expected_content = if cfg!(feature = "format") {
        concat!(
            "// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.\n\n",
            "/**\n",
            " * Doc comment.\n",
            " * Supports new lines.\n",
            " *\n",
            " * Testing\n",
            " */\n",
            "export type E = never;\n",
        )
    } else {
        concat!(
            "// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.\n\n",
            "/**\n",
            " * Doc comment.\n",
            " * Supports new lines.\n",
            " *\n",
            " * Testing\n",
            " */\n",
            "export type E = never;",
            "\n",
        )
    };

    let actual_content = fs::read_to_string(E::default_output_path().unwrap()).unwrap();

    assert_eq!(actual_content, expected_content);
}

#[test]
fn export_f() {
    F::export().unwrap();

    let expected_content = if cfg!(feature = "format") {
        concat!(
            "// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.\n\n",
            "/**\n",
            " * Doc comment.\n",
            " * Supports new lines.\n",
            " *\n",
            " * Testing\n",
            " */\n",
            "export type F = \"VarA\" | { \"VarB\": never[] } | {\n",
            "  \"VarC\": {\n",
            "    /**\n",
            "     * Doc of field of variant\n",
            "     *\n",
            "     * Testing\n",
            "     */\n",
            "    variant_field: number;\n",
            "  };\n",
            "};\n",
        )
    } else {
        concat!(
            "// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.\n\n",
            "/**\n",
            " * Doc comment.\n",
            " * Supports new lines.\n",
            " *\n",
            " * Testing\n",
            " */\n",
            "export type F = \"VarA\" | { \"VarB\": never[] } | { \"VarC\": { \n",
            "/**\n",
            " * Doc of field of variant\n",
            " *\n",
            " * Testing\n",
            " */\n",
            "variant_field: number, } };",
            "\n",
        )
    };

    let actual_content = fs::read_to_string(F::default_output_path().unwrap()).unwrap();

    assert_eq!(actual_content, expected_content);
}

#[test]
fn export_g() {
    G::export().unwrap();

    let expected_content = if cfg!(feature = "format") {
        concat!(
            "// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.\n\n",
            "export type G =\n",
            "  & {\n",
            "    /**\n",
            "     * Docs\n",
            "     */\n",
            "    some_other_field: number;\n",
            "  }\n",
            "  & (\"VarA\" | { \"VarB\": never[] } | {\n",
            "    \"VarC\": {\n",
            "      /**\n",
            "       * Doc of field of variant\n",
            "       *\n",
            "       * Testing\n",
            "       */\n",
            "      variant_field: number;\n",
            "    };\n",
            "  });\n",
        )
    } else {
        concat!(
            "// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.\n",
            "\n",
            "export type G = { \n",
            "/**\n",
            " * Docs\n",
            " */\n",
            "some_other_field: number, } & (\"VarA\" | { \"VarB\": never[] } | { \"VarC\": { \n",
            "/**\n",
            " * Doc of field of variant\n",
            " *\n",
            " * Testing\n",
            " */\n",
            "variant_field: number, } });",
            "\n",
        )
    };

    let actual_content = fs::read_to_string(G::default_output_path().unwrap()).unwrap();

    assert_eq!(actual_content, expected_content);
}

#[test]
fn export_h() {
    H::export().unwrap();

    let expected_content = if cfg!(feature = "format") {
        concat!(
            "// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.\n\n",
            "/**\n",
            " * Block doc comment\n",
            " *\n",
            " * works\n",
            " */\n",
            "export type H = { foo: number };\n",
        )
    } else {
        concat!(
            "// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.\n\n",
            "/**\n",
            " * Block doc comment\n",
            " *\n",
            " * works\n",
            " */\n",
            "export type H = { foo: number, };\n",
        )
    };

    let actual_content = fs::read_to_string(H::default_output_path().unwrap()).unwrap();

    assert_eq!(actual_content, expected_content);
}

#[test]
fn export_i() {
    I::export().unwrap();

    let expected_content = if cfg!(feature = "format") {
        concat!(
        "// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.\n\n",
        "/**\n",
        " * line 107\n",
        " */\n",
        "export type I = {\n",
        "  /**\n",
        "   * column 32\n",
        "   */\n",
        "  a: number;\n",
        "  /**\n",
        "   * path integration::docs\n",
        "   */\n",
        "  b: number;\n",
        "};\n",
        )
    } else {
        concat!(
        "// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.\n\n",
        "/**\n",
        " *line 107\n",
        " */\n",
        "export type I = { \n",
        "/**\n",
        " *column 32\n",
        " */\n",
        "a: number, \n",
        "/**\n",
        " *path integration::docs\n",
        " */\n",
        "b: number, };\n",
        )
    };

    let actual_content = fs::read_to_string(I::default_output_path().unwrap()).unwrap();

    assert_eq!(actual_content, expected_content);
}
