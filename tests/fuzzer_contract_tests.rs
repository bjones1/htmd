mod common;
#[path = "common/fuzzer_support.rs"]
mod fuzzer_support;
// The team draft is not a Cargo target; compile it here to test seed replay only.
#[allow(dead_code)]
#[path = "../fuzzer 2nd team specification/fuzzer-implementation.rs"]
mod draft;

use html5ever::{
    ParseOpts, parse_document, serialize, tendril::TendrilSink, tree_builder::TreeBuilderOpts,
};
use markup5ever_rcdom::{Handle, NodeData, RcDom, SerializableHandle};
use rand::{RngCore, SeedableRng};
use rand_chacha::ChaCha8Rng;

fn serialized(nodes: &[Handle]) -> Vec<u8> {
    let mut bytes = Vec::new();
    for node in nodes {
        serialize(
            &mut bytes,
            &SerializableHandle::from(node.clone()),
            html5ever::serialize::SerializeOpts {
                traversal_scope: html5ever::serialize::TraversalScope::IncludeNode,
                ..Default::default()
            },
        )
        .unwrap();
    }
    bytes
}

#[test]
fn oracle_body_matches_explicit_document_parsing() {
    for input in [
        " \n<p>x</p>",
        "<td>x</td>",
        "<pre>\nx</pre>",
        "<p>&amp;&#10;&nbsp;</p>",
        "<script>if (a < b) x()</script><p>x</p>",
        "<textarea>\nx &amp; y</textarea>",
        "<div><noscript>x</noscript></div>",
    ] {
        let dom = parse_document(
            RcDom::default(),
            ParseOpts {
                tree_builder: TreeBuilderOpts {
                    scripting_enabled: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        )
        .one(input);
        let html = dom.document.children.borrow().iter().find(|n|
            matches!(&n.data, NodeData::Element { name, .. } if name.local.as_ref() == "html")
        ).unwrap().clone();
        let body = html.children.borrow().iter().find(|n|
            matches!(&n.data, NodeData::Element { name, .. } if name.local.as_ref() == "body")
        ).unwrap().clone();
        assert_eq!(
            serialized(&fuzzer_support::parse_body_children(input).unwrap()),
            serialized(&body.children.borrow()),
            "{input:?}"
        );
    }
}

#[test]
fn reference_spellings_and_decoded_text_are_distinct() {
    let nodes = fuzzer_support::parse_body_children("<p>&amp;&#10;&nbsp;&amp;#10;</p>").unwrap();
    let children = nodes[0].children.borrow();
    let NodeData::Text { contents } = &children[0].data else {
        panic!("expected text")
    };
    assert_eq!(contents.borrow().as_ref(), "&\n\u{a0}&#10;");
    assert_eq!("e\u{301}😀".chars().count(), 3);
    assert_eq!("e\u{301}😀".len(), 7);
    // Use the common Faithful helper for the actual conversion assertion.
    assert_eq!(
        common::convert_faithful("<p>&amp;#10;</p>").unwrap(),
        r"\&#10;"
    );
}

#[test]
fn case_replay_does_not_depend_on_previous_cases() {
    for profile in [draft::ParserProfile::Core, draft::ParserProfile::Gfm] {
        let mut sequence = draft::Fuzzer::new(42, 0, profile);
        for index in 0..100 {
            let sequential = sequence.generate_valid_case();
            let isolated = draft::Fuzzer::new(42, index, profile).generate_valid_case();
            assert_eq!(sequential.case_index, index);
            assert_eq!(sequential.master_seed, 42);
            assert_eq!(sequential.input, isolated.input);
        }
    }
}

#[test]
fn case_seed_wraps_without_panicking() {
    let mut sequence = draft::Fuzzer::new(u64::MAX, u64::MAX, draft::ParserProfile::Core);
    let first = sequence.generate_valid_case();
    let second = sequence.generate_valid_case();
    assert_eq!(first.case_index, u64::MAX);
    assert_eq!(second.case_index, 0);
    assert_eq!(
        second.input,
        draft::Fuzzer::new(u64::MAX, 0, draft::ParserProfile::Core)
            .generate_valid_case()
            .input
    );
}

#[test]
fn pinned_chacha8_seed_stream() {
    let mut rng = ChaCha8Rng::seed_from_u64(42);
    let actual: Vec<u32> = (0..4).map(|_| rng.next_u32()).collect();
    assert_eq!(actual, vec![962419617, 2928721845, 628724104, 4081401798]);
}

#[test]
fn oracle_block_set_tracks_htmd_source() {
    let source = include_str!("../src/util/node.rs");
    let block = source
        .split("pub(crate) static BLOCK_ELEMENTS:")
        .nth(1)
        .unwrap()
        .split("phf_set! {")
        .nth(1)
        .unwrap()
        .split("};")
        .next()
        .unwrap();
    let actual: std::collections::BTreeSet<&str> = block.split('"').skip(1).step_by(2).collect();
    let expected: std::collections::BTreeSet<&str> =
        fuzzer_support::BLOCK_ELEMENTS.iter().copied().collect();
    assert_eq!(
        expected, actual,
        "update the versioned oracle block set when htmd changes"
    );
}

#[test]
fn draft_uses_wrapping_sum_as_case_seed() {
    let profile = draft::ParserProfile::Core;
    assert_eq!(
        draft::Fuzzer::new(42, 7, profile)
            .generate_valid_case()
            .input,
        draft::Fuzzer::new(49, 0, profile)
            .generate_valid_case()
            .input
    );
    assert_eq!(
        draft::Fuzzer::new(u64::MAX, 1, profile)
            .generate_valid_case()
            .input,
        draft::Fuzzer::new(0, 0, profile)
            .generate_valid_case()
            .input
    );
}
