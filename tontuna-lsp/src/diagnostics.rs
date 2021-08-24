use lsp_types::{Diagnostic, DiagnosticSeverity};
use crate::utils::LocationTranslator;

pub fn get_diagnostics(source: &str) -> Vec<Diagnostic> {
    let mut translator = LocationTranslator::for_source(source);

    tontuna::parse(source)
        .err()
        .map(|e| Diagnostic {
            range: translator.to_lsp(e.span),
            severity: Some(DiagnosticSeverity::Error),
            code: None,
            code_description: None,
            source: Some("ticc".to_owned()),
            message: e.message.clone(),
            related_information: None,
            tags: None,
            data: None,
        })
        .into_iter()
        .collect()
}
