// Reverse typographic substitutions made by smart-punctuation Markdown renderers.
pub(crate) fn reverse(text: &str) -> String {
    let mut result = String::with_capacity(text.len());
    for ch in text.chars() {
        match ch {
            '“' | '”' => result.push('"'),
            '‘' | '’' => result.push('\''),
            '…' => result.push_str("..."),
            '–' => result.push_str("--"),
            '—' => result.push_str("---"),
            _ => result.push(ch),
        }
    }
    result
}

// A reversed em dash on its own would be parsed as a thematic break.
pub(crate) fn is_isolated_em_dash(text: &str) -> bool {
    text.trim() == "—"
}

// A lone opening single quote is not reconstructed reliably by smart renderers.
// Matched quotation pairs remain reversible.
pub(crate) fn has_leading_open_quote_mismatch(text: &str) -> bool {
    let text = text.trim_start();
    text.strip_prefix('‘').is_some_and(|rest| {
        // A right curly mark followed by letters/digits is usually an
        // apostrophe (e.g. ’em), not the closing mark of a quotation.
        !rest.char_indices().any(|(index, ch)| {
            ch == '’'
                && !rest[index + ch.len_utf8()..]
                    .chars()
                    .next()
                    .is_some_and(char::is_alphanumeric)
        })
    })
}

// An isolated dash may be inside a larger list or quotation block.
pub(crate) fn has_isolated_em_dash_descendant(node: &std::rc::Rc<markup5ever_rcdom::Node>) -> bool {
    use markup5ever_rcdom::NodeData;
    if let NodeData::Element { name, .. } = &node.data {
        let tag = name.local.as_ref();
        if is_literal_tag(tag) {
            return false;
        }
        if (tag == "li" || tag == "p") && contains_only_em_dash(node) {
            return true;
        }
    }
    node.children
        .borrow()
        .iter()
        .any(has_isolated_em_dash_descendant)
}

// Recognize a list or quote whose sole visible content is an isolated em dash.
pub(crate) fn contains_only_em_dash(node: &std::rc::Rc<markup5ever_rcdom::Node>) -> bool {
    use markup5ever_rcdom::NodeData;
    let children = node.children.borrow();
    let mut found = false;
    for child in children.iter() {
        match &child.data {
            NodeData::Text { contents } => {
                let text = contents.borrow();
                if text.trim().is_empty() {
                    continue;
                }
                if found || !is_isolated_em_dash(&text) {
                    return false;
                }
                found = true;
            }
            NodeData::Element { name, .. } => {
                if is_literal_tag(name.local.as_ref()) || found || !contains_only_em_dash(child) {
                    return false;
                }
                found = true;
            }
            _ => return false,
        }
    }
    found
}

// Code and other literal elements must not trigger a Markdown-syntax fallback.
fn is_literal_tag(tag: &str) -> bool {
    matches!(
        tag,
        "code" | "pre" | "script" | "style" | "math" | "textarea"
    )
}
