use skrifa::string::StringId;

use super::*;

/// Every character RSpice Symbols guarantees, as `tools/fonts/build_rspice_symbols.py`
/// lists them: electrical, math and units, GD&T, interface marks and keys.
const GUARANTEED_SYMBOLS: [&str; 5] = [
    "⏚⏛⎓∿⏦⏧⌁⎍⎎⏻⏼⏽⏾⭘⚡℧",
    "∅∇∝∓≡⋅℃⁻∠⊥∥",
    "⌀⏤⏥○⌭⌒⌓⟂⌖◎⌯⌰⌴⌵↧□ⓂⓁⒻⓉⓅⓈ⌲⌳⌱",
    "⌄▸▾▴◂★☆✕✗●◻◼■△▽◇⊞⚠ⓘ⇒",
    "⌘⌥⇧⌃⏎⌫⎋⇥⌦⇪",
];

fn guaranteed_symbols() -> impl Iterator<Item = char> {
    GUARANTEED_SYMBOLS.iter().flat_map(|group| group.chars())
}

fn font(face: Face) -> FontRef<'static> {
    FontRef::new(face.bytes()).unwrap_or_else(|error| panic!("{face:?} does not parse: {error}"))
}

fn name(face: Face, id: StringId) -> Option<String> {
    font(face)
        .localized_strings(id)
        .english_or_first()
        .map(|name| name.to_string())
}

#[test]
fn every_face_parses_and_records_its_weight() {
    for face in Face::ALL {
        let weight = font(face).attributes().weight.value();
        assert_eq!(weight, f32::from(face.weight()), "{face:?}");
    }
}

#[test]
fn every_face_is_named_for_its_family() {
    for face in Face::ALL {
        let family = name(face, StringId::TYPOGRAPHIC_FAMILY_NAME)
            .or_else(|| name(face, StringId::FAMILY_NAME))
            .unwrap_or_default();
        assert_eq!(family, face.family(), "{face:?}");
    }
}

#[test]
fn rspice_symbols_does_not_take_the_reserved_font_name() {
    // A Modified Version of an OFL font may not be named with its Reserved
    // Font Name, which for IBM Plex is "Plex".
    let names = [
        StringId::FAMILY_NAME,
        StringId::SUBFAMILY_NAME,
        StringId::UNIQUE_ID,
        StringId::FULL_NAME,
        StringId::POSTSCRIPT_NAME,
        StringId::TYPOGRAPHIC_FAMILY_NAME,
        StringId::TYPOGRAPHIC_SUBFAMILY_NAME,
        StringId::WWS_FAMILY_NAME,
        StringId::WWS_SUBFAMILY_NAME,
    ];
    for id in names {
        for name in font(Face::Symbols).localized_strings(id) {
            let name = name.to_string();
            assert!(!name.to_lowercase().contains("plex"), "{id:?} is {name:?}");
        }
    }
    let licence = Face::Symbols.licence();
    assert!(licence.contains("Reserved Font Name \"Plex\""));
    assert!(licence.contains("The Noto Project Authors"));
    assert!(licence.contains("SIL OPEN FONT LICENSE Version 1.1"));
}

#[test]
fn rspice_symbols_has_every_guaranteed_symbol_and_the_replacement() {
    let missing: String = guaranteed_symbols()
        .filter(|symbol| !Face::Symbols.covers(*symbol))
        .collect();
    assert_eq!(missing, "", "RSpice Symbols lacks these");
    assert_eq!(guaranteed_symbols().count(), 82);
    assert!(Face::Symbols.covers(REPLACEMENT));
}

#[test]
fn every_role_letters_every_guaranteed_symbol() {
    for role in Role::ALL {
        for symbol in guaranteed_symbols() {
            for sink in [Sink::Screen, Sink::Print] {
                assert_eq!(
                    role.resolve(sink, symbol),
                    (Face::Symbols, symbol),
                    "{role:?}"
                );
            }
        }
    }
}

#[test]
fn every_chain_starts_at_its_role_and_print_ends_in_symbols() {
    for (role, family, weight) in [
        (Role::Sans, "IBM Plex Sans", 400),
        (Role::SansMedium, "IBM Plex Sans", 500),
        (Role::SansSemibold, "IBM Plex Sans", 600),
        (Role::Mono, "IBM Plex Mono", 400),
        (Role::MonoMedium, "IBM Plex Mono", 500),
    ] {
        let chain = role.chain();
        assert_eq!(role.primary().family(), family, "{role:?}");
        assert_eq!(role.primary().weight(), weight, "{role:?}");
        assert_eq!(
            role.faces_in(Sink::Print).last(),
            Some(Face::Symbols),
            "{role:?}"
        );
        let mut seen = chain.to_vec();
        seen.sort();
        seen.dedup();
        assert_eq!(seen.len(), chain.len(), "{role:?} repeats a face");
    }
}

#[test]
fn the_mono_roles_borrow_greek_from_plex_sans_at_their_weight() {
    for greek in ['Δ', 'Ω', 'μ'] {
        assert!(!Face::MonoRegular.covers(greek));
        assert_eq!(
            Role::Mono.face_for(Sink::Print, greek),
            Some(Face::SansRegular)
        );
        assert_eq!(
            Role::MonoMedium.face_for(Sink::Print, greek),
            Some(Face::SansMedium)
        );
    }
    assert_eq!(
        Role::Mono.face_for(Sink::Screen, 'A'),
        Some(Face::MonoRegular)
    );
}

#[test]
fn a_character_no_face_has_resolves_to_the_replacement_square() {
    for role in Role::ALL {
        for sink in [Sink::Screen, Sink::Print] {
            assert!(!role.covers(sink, '\u{10FFFD}'));
            assert_eq!(
                role.resolve(sink, '\u{10FFFD}'),
                (Face::Symbols, REPLACEMENT)
            );
        }
    }
}

fn expected(runs: &[(PrintFace, &str)]) -> Vec<Run> {
    runs.iter()
        .map(|(face, text)| Run {
            face: *face,
            text: (*text).to_owned(),
        })
        .collect()
}

#[test]
fn runs_split_where_the_face_changes() {
    assert_eq!(
        Role::Mono.runs("⌀10 mm Δ"),
        expected(&[
            (PrintFace::Symbols, "⌀"),
            (PrintFace::MonoRegular, "10 mm "),
            (PrintFace::SansRegular, "Δ"),
        ])
    );
    assert!(Role::Sans.runs("").is_empty());
}

#[test]
fn a_grapheme_cluster_is_lettered_by_one_face() {
    // A variation selector does not draw, so the face that has the base
    // letters the whole cluster.
    assert_eq!(
        Role::Sans.runs("⚠\u{FE0F}"),
        expected(&[(PrintFace::Symbols, "⚠\u{FE0F}")])
    );
    // An unknown base takes its marks with it into one replacement.
    assert_eq!(
        Role::Sans.runs("a\u{10FFFD}\u{301}b"),
        expected(&[
            (PrintFace::SansRegular, "a"),
            (PrintFace::Symbols, "\u{25FB}"),
            (PrintFace::SansRegular, "b"),
        ])
    );
}

#[cfg(feature = "egui")]
#[test]
fn every_egui_family_holds_its_roles_chain() {
    let fonts = egui_font_definitions();
    assert_eq!(fonts.font_data.len(), Face::ALL.len());
    for face in Face::ALL {
        assert_eq!(&*fonts.font_data[face.key()].font, face.bytes(), "{face:?}");
    }
    assert_eq!(fonts.families.len(), Role::ALL.len());
    for role in Role::ALL {
        let keys: Vec<&str> = role.chain().iter().map(|face| face.key()).collect();
        assert_eq!(fonts.families[&role.egui_family()], keys, "{role:?}");
    }
    assert_eq!(Role::Sans.egui_family(), egui::FontFamily::Proportional);
    assert_eq!(Role::Mono.egui_family(), egui::FontFamily::Monospace);
}

#[test]
fn print_names_exactly_the_faces_it_letters_from() {
    let printed: Vec<Face> = Face::ALL
        .into_iter()
        .filter(|face| Sink::Print.letters(*face))
        .collect();
    assert_eq!(PrintFace::ALL.map(PrintFace::face).to_vec(), printed);
    for face in PrintFace::ALL {
        assert_eq!(face.face().printed(), Some(face), "{face:?}");
    }
    assert!(Face::ALL.into_iter().all(|face| Sink::Screen.letters(face)));
    for role in Role::ALL {
        assert_eq!(role.print_primary().face(), role.primary(), "{role:?}");
    }
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn print_letters_the_square_where_the_screen_letters_a_script_it_cannot_order() {
    for (letter, regular, semibold) in [
        ('ب', Face::SansArabicRegular, Face::SansArabicSemibold),
        ('ש', Face::SansHebrewRegular, Face::SansHebrewSemibold),
        ('ก', Face::SansThaiRegular, Face::SansThaiSemibold),
    ] {
        for role in Role::ALL {
            let screen = if role == Role::SansSemibold {
                semibold
            } else {
                regular
            };
            assert_eq!(
                role.resolve(Sink::Screen, letter),
                (screen, letter),
                "{role:?}"
            );
            assert_eq!(
                role.resolve(Sink::Print, letter),
                (Face::Symbols, REPLACEMENT),
                "{role:?}"
            );
        }
    }
    assert_eq!(
        Role::Sans.runs("قيمة R1"),
        expected(&[
            (PrintFace::Symbols, "\u{25FB}\u{25FB}\u{25FB}\u{25FB}"),
            (PrintFace::SansRegular, " R1"),
        ])
    );
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn latin_never_reaches_the_script_faces() {
    // Plex Sans Arabic, Hebrew and Thai carry Latin too; it must letter
    // from Plex Sans and Mono, as it does without them.
    for role in Role::ALL {
        for character in (' '..='~').chain("ΔΩμ±°·–—…".chars()) {
            let face = role.face_for(Sink::Screen, character);
            assert_eq!(
                face,
                role.face_for(Sink::Print, character),
                "{role:?} {character:?}"
            );
        }
    }
}
