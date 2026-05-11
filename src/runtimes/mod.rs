use crate::{error::AppError, models::LanguageInfo};

mod deno;
mod python;

#[derive(Debug, Clone, Copy)]
pub struct RuntimeSpec {
    pub id: &'static str,
    pub aliases: &'static [&'static str],
    pub image: &'static str,
    pub file_name: &'static str,
    pub command: &'static [&'static str],
    pub env: &'static [(&'static str, &'static str)],
}

impl RuntimeSpec {
    pub fn info(self) -> LanguageInfo {
        LanguageInfo {
            id: self.id,
            aliases: self.aliases,
        }
    }
}

pub fn all() -> &'static [RuntimeSpec] {
    &[deno::JAVASCRIPT, deno::TYPESCRIPT, python::PYTHON]
}

pub fn list() -> Vec<LanguageInfo> {
    all().iter().map(|runtime| runtime.info()).collect()
}

pub fn resolve(language: &str) -> Result<RuntimeSpec, AppError> {
    let normalized = language.trim().to_ascii_lowercase();

    all()
        .iter()
        .copied()
        .find(|runtime| {
            runtime.id == normalized
                || runtime
                    .aliases
                    .iter()
                    .any(|alias| alias.eq_ignore_ascii_case(&normalized))
        })
        .ok_or_else(|| AppError::UnsupportedLanguage(language.to_owned()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resolves_supported_language_ids_and_aliases() {
        assert_eq!(resolve("javascript").unwrap().id, "javascript");
        assert_eq!(resolve("js").unwrap().id, "javascript");
        assert_eq!(resolve("typescript").unwrap().id, "typescript");
        assert_eq!(resolve("ts").unwrap().id, "typescript");
        assert_eq!(resolve("python").unwrap().id, "python");
        assert_eq!(resolve("py").unwrap().id, "python");
    }

    #[test]
    fn language_resolution_is_case_and_whitespace_insensitive() {
        assert_eq!(resolve("  PyThOn  ").unwrap().id, "python");
    }

    #[test]
    fn rejects_unknown_language() {
        let err = resolve("ruby").unwrap_err();
        assert!(matches!(err, AppError::UnsupportedLanguage(_)));
    }
}
