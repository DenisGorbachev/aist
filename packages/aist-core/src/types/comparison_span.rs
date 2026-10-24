use ra_ap_span::{Edition, EditionedFileId, FileId, ROOT_ERASED_FILE_AST_ID, Span, SpanAnchor, SyntaxContext, TextRange, TextSize};
use ra_ap_syntax_bridge::SpanMapper;

#[derive(Clone, Copy, Debug)]
pub struct ComparisonSpan(pub Span);

impl Default for ComparisonSpan {
    fn default() -> Self {
        Self(Span {
            range: TextRange::empty(TextSize::new(0)),
            anchor: SpanAnchor {
                file_id: EditionedFileId::new(FileId::from_raw(0), Edition::CURRENT),
                ast_id: ROOT_ERASED_FILE_AST_ID,
            },
            ctx: SyntaxContext::root(Edition::CURRENT),
        })
    }
}

impl SpanMapper for ComparisonSpan {
    fn span_for(&self, _: TextRange) -> Span {
        self.0
    }
}
