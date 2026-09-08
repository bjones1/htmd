use crate::{
    Context, Element,
    element_handler::anchor::LinkReferenceCheckpoint,
    element_handler::element_util::{
        serialize_if_extra_attrs_or_inline, serialize_walked_element_when_faithful,
    },
    element_handler::{HandlerResult, Handlers},
    html_block::opens_html_block,
    text_util::frame_as_block,
};

pub(super) fn p_handler(handlers: &dyn Handlers, element: Element) -> Option<HandlerResult> {
    serialize_if_extra_attrs_or_inline!(handlers, element, 0);
    // A paragraph is a leaf block: its children begin an inline context. The
    // walk is speculative, so the link references it buffers are checkpointed.
    let checkpoint = LinkReferenceCheckpoint::new();
    let content = handlers.walk_children_content(element.node, Context::Inline);
    // A paragraph is opened only where the block scan matched nothing else, so
    // content meeting an HTML block start condition would be read back as that
    // block and the `<p>` around it lost. See the "Special case for paragraphs"
    // section of `unsupported_html.md`.
    serialize_walked_element_when_faithful!(
        handlers,
        element,
        opens_html_block(&content),
        checkpoint
    );
    Some(frame_as_block(&content).into())
}
