// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2026 Mateus Rocha de Paula and the OxiGuitar contributors

//! Which corpus files the corpus tests take: Guitar Pro 3 to 8, the
//! formats this repository's plugins read, recognised by their contents
//! (other formats `detect` knows, such as MuseScore's, are other plugins').

use oxiguitar_formats::Format;

/// A Guitar Pro 3, 4, 5, 6 or 7-to-8 file.
pub(crate) fn is_guitar_pro(data: &[u8]) -> bool {
    matches!(
        oxiguitar_formats::detect(data),
        Some(
            Format::GuitarPro3 { .. }
                | Format::GuitarPro4 { .. }
                | Format::GuitarPro5 { .. }
                | Format::GuitarPro6
                | Format::GuitarPro7Plus
        )
    )
}
