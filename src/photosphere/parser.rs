use tree_sitter::{Parser, Language};

extern "C" {
    fn tree_sitter_elixir() -> Language;
}
