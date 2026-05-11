use crate::{error::AppError, models::LanguageInfo};

mod deno;
mod mainstream;
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
            id: self.id.to_owned(),
            aliases: self
                .aliases
                .iter()
                .map(|alias| (*alias).to_owned())
                .collect(),
        }
    }
}

pub fn all() -> &'static [RuntimeSpec] {
    &[
        deno::JAVASCRIPT,
        deno::TYPESCRIPT,
        python::PYTHON,
        mainstream::BASH,
        mainstream::C,
        mainstream::CPP,
        mainstream::CSHARP,
        mainstream::GO,
        mainstream::JAVA,
        mainstream::NODE,
        mainstream::PERL,
        mainstream::PHP,
        mainstream::R,
        mainstream::RUBY,
        mainstream::RUST,
        mainstream::SWIFT,
    ]
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
        assert_eq!(resolve("bash").unwrap().id, "bash");
        assert_eq!(resolve("sh").unwrap().id, "bash");
        assert_eq!(resolve("c").unwrap().id, "c");
        assert_eq!(resolve("cpp").unwrap().id, "cpp");
        assert_eq!(resolve("c++").unwrap().id, "cpp");
        assert_eq!(resolve("csharp").unwrap().id, "csharp");
        assert_eq!(resolve("cs").unwrap().id, "csharp");
        assert_eq!(resolve("go").unwrap().id, "go");
        assert_eq!(resolve("golang").unwrap().id, "go");
        assert_eq!(resolve("java").unwrap().id, "java");
        assert_eq!(resolve("node").unwrap().id, "node");
        assert_eq!(resolve("nodejs").unwrap().id, "node");
        assert_eq!(resolve("perl").unwrap().id, "perl");
        assert_eq!(resolve("php").unwrap().id, "php");
        assert_eq!(resolve("r").unwrap().id, "r");
        assert_eq!(resolve("rscript").unwrap().id, "r");
        assert_eq!(resolve("ruby").unwrap().id, "ruby");
        assert_eq!(resolve("rb").unwrap().id, "ruby");
        assert_eq!(resolve("rust").unwrap().id, "rust");
        assert_eq!(resolve("rs").unwrap().id, "rust");
        assert_eq!(resolve("swift").unwrap().id, "swift");
    }

    #[test]
    fn language_resolution_is_case_and_whitespace_insensitive() {
        assert_eq!(resolve("  PyThOn  ").unwrap().id, "python");
    }

    #[test]
    fn rejects_unknown_language() {
        let err = resolve("klingon").unwrap_err();
        assert!(matches!(err, AppError::UnsupportedLanguage(_)));
    }

    #[test]
    fn init_script_pulls_all_runtime_images() {
        let init_script = include_str!("../../scripts/init-runtimes.sh");

        for runtime in all() {
            assert!(
                init_script.contains(runtime.image),
                "missing {} from init script",
                runtime.image
            );
        }
    }

    #[test]
    fn runtime_ids_are_unique() {
        let mut ids = std::collections::HashSet::new();

        for runtime in all() {
            assert!(
                ids.insert(runtime.id),
                "duplicate runtime id {}",
                runtime.id
            );
        }
    }
}
