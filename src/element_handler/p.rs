use crate::{
    Context, Element,
    element_handler::element_util::{
        serialize_element_when_faithful, serialize_if_extra_attrs_or_inline,
    },
    element_handler::{HandlerResult, Handlers},
    html_block::starts_html_block,
    options::TranslationMode,
    smart_punctuation::{has_leading_open_quote_mismatch, is_isolated_em_dash},
    util::text::frame_as_block,
};

pub(super) fn p_handler(handlers: &dyn Handlers, element: &Element) -> Option<HandlerResult> {
    serialize_if_extra_attrs_or_inline!(handlers, element, 0);
    if handlers.options().translation_mode == TranslationMode::Faithful {
        let children = element.node.children.borrow();
        if children.len() == 1
            && let markup5ever_rcdom::NodeData::Text { contents } = &children[0].data
        {
            let text = contents.borrow();
            if is_isolated_em_dash(&text) || has_leading_open_quote_mismatch(&text) {
                return Some(
                    crate::element_handler::element_util::serialize_element_result(
                        handlers, element,
                    ),
                );
            }
        }
    }
    // A paragraph is a leaf block: its children begin an inline context.
    let content = handlers.walk_children_content(element.node, Context::INLINE);
    // An HTML block opening at the start of the content dissolves the paragraph
    // holding it, so writing the content alone would lose the `<p>`; the whole
    // element goes out as HTML instead. See the "Paragraphs" section of
    // `translation_approach.md`.
    serialize_element_when_faithful!(handlers, element, starts_html_block(&content));
    Some(frame_as_block(&content).into())
}
