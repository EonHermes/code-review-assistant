//! Language abstraction layer
//! Currently unused in favor of direct parser implementations

pub trait LanguageHandler {
    fn analyze(&self, path: &std::path::Path) -> crate::errors::Result<()>;
}
