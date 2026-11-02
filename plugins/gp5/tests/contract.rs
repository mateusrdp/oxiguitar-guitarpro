// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2026 Mateus Rocha de Paula and the OxiGuitar contributors

//! The contract suite: builds this crate as an `.oxp` file (alone, in the workspace's target/oxp, so it
//! carries only its own reader) and checks it through OxiGuitar's own plugin
//! host, as the app would load it. `cargo test` runs it with the rest.

use std::path::Path;

#[test]
fn the_plugin_file_keeps_the_contract() {
    let workspace = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let oxp = oxiguitar_plugin_test::build(&workspace, env!("CARGO_PKG_NAME"), "gp5", false)
        .expect("the .oxp file builds");
    let report = oxiguitar_plugin_test::check(&oxp);
    for passed in &report.passed {
        println!("ok    {passed}");
    }
    assert!(report.ok(), "failed: {:#?}", report.failed);
}
