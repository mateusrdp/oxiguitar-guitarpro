// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2026 Mateus Rocha de Paula and the OxiGuitar contributors

//! The test corpus (`support/corpus.rs`: `OXIGUITAR_CORPUS`, or the
//! workspace's `corpus/files/`; never committed). Any folder of files,
//! searched recursively: each Guitar Pro file in it, recognised by its
//! contents, must read. With no corpus these tests check nothing (unless
//! `OXIGUITAR_CORPUS_REQUIRED` is set), so CI is unaffected.
//!
//! `cargo test --release -p oxiguitar-formats --test corpus -- --nocapture`

mod support {
    pub(crate) mod compare;
    pub(crate) mod corpus;
    pub(crate) mod pick;
}

use std::path::{Path, PathBuf};

use oxiguitar_formats::read;
use support::corpus::{self, Outcome};

fn fallback() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../corpus/files")
}

/// Every Guitar Pro file (3 to 8, recognised by its contents) reads, with
/// a sound structure and at least one track and bar.
#[test]
fn every_corpus_guitar_pro_file_reads() {
    corpus::check("guitar pro read", &fallback(), |_, data| {
        if !support::pick::is_guitar_pro(data) {
            return Ok(Outcome::NotTaken);
        }
        let format = oxiguitar_formats::detect(data);
        let imported = read(data).map_err(|e| format!("{format:?} doesn't read: {e}"))?;
        let score = &imported.score;
        score
            .check_structure()
            .map_err(|e| format!("{format:?}: {e}"))?;
        if score.bar_count() == 0 || score.tracks.is_empty() {
            return Err(format!("{format:?}: no bars or no tracks"));
        }
        Ok(Outcome::Passed)
    });
}

/// The corpus file named `name`, wherever it is in the corpus.
fn find(dir: &Path, name: &str) -> Option<PathBuf> {
    corpus::files(dir)
        .into_iter()
        .find(|p| p.file_name().is_some_and(|n| n == name))
}

/// Songs saved in two formats read into the same music: pairs of files of
/// our corpus, checked where they are in the corpus at hand.
///
/// The pairs are different editions of each transcription, so they need not
/// match exactly. Notes are compared by sounding pitch, so a passage fingered
/// on another string still matches. The differences left were each checked
/// against the files (with PyGuitarPro for the older ones) on 16 October 2026:
/// - Manhattan and Cirice: the .gp edition adds notes to some chords and
///   replaces some rests with dead notes.
/// - Trust: Guitar Pro 7's drum kit has no electric snare, so the .gp plays
///   38 where the .gp3 has 40; the .gp moves some vibratos by a beat.
///
/// The thresholds are today's figures, rounded down, to catch regressions.
#[test]
fn songs_in_two_formats_agree() {
    let Some(dir) = corpus::folder(&fallback()) else {
        return;
    };
    // (newer format, older format, minimum share of matching beats and notes)
    let pairs = [
        (
            "metallica-metallica-seek-and-destroy_1424280142.gp",
            "metallica-metallica-seek-and-destroy_1424280142.gp5",
            0.999,
        ),
        (
            "garoto-annibal-augusto-sardinha-desvairada.gp",
            "garoto-annibal-augusto-sardinha-desvairada.gp5",
            0.999,
        ),
        (
            "gary-moore-one_day_ver2.gp",
            "gary-moore-one_day_ver2.gp3",
            0.99,
        ),
        ("ghost-cirice.gp", "ghost-cirice.gp5", 0.98),
        (
            "Eric Johnson - Manhattan.gp",
            "Eric Johnson - Manhattan.gp5",
            0.95,
        ),
        ("megadeth-trust.gp", "megadeth-trust.gp3", 0.91),
    ];
    let mut failures = Vec::new();
    for (new, old, min) in pairs {
        // Our corpus's pairs: another corpus may have none of them.
        let (Some(old_path), Some(new_path)) = (find(&dir, old), find(&dir, new)) else {
            continue;
        };
        let a = read(&std::fs::read(old_path).unwrap()).unwrap().score;
        let b = read(&std::fs::read(new_path).unwrap()).unwrap().score;
        let report = support::compare::compare(&a, &b);
        eprintln!("{old} → {new}\n{}", report.summary());
        let exact = [
            "bar count",
            "time signatures",
            "key signatures",
            "repeats",
            "alternate endings",
            "markers",
            "track count",
            "tunings",
            "capos",
        ];
        for category in exact {
            if report.ratio(category) < 1.0 {
                failures.push(format!("{new}: {category} {:.3}", report.ratio(category)));
            }
        }
        for category in ["beats", "notes", "note effects", "beat effects"] {
            if report.ratio(category) < min {
                failures.push(format!("{new}: {category} {:.3}", report.ratio(category)));
            }
        }
        // Guitar Pro 3–5 store no clef; the reader picks it from the
        // instrument, which must give what Guitar Pro 7 wrote on converting.
        for (ta, tb) in a.tracks.iter().zip(&b.tracks) {
            let clef = |t: &oxiguitar_model::Track| {
                t.staves
                    .first()
                    .and_then(|s| s.bars.first())
                    .map(|bar| bar.clef)
            };
            if clef(ta) != clef(tb) {
                failures.push(format!(
                    "{new}: track {:?}: clef {:?} in the old format, {:?} in the new",
                    tb.name,
                    clef(ta),
                    clef(tb)
                ));
            }
        }
    }
    assert!(failures.is_empty(), "{failures:#?}");
}
