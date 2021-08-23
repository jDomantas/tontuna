use lsp_types::{
    notification::{self, Notification},
    request,
};

use crate::{FileKey, Handlers, TicServer};

fn semantic_tokens(
    server: &mut TicServer,
    params: lsp_types::SemanticTokensParams,
) -> Option<lsp_types::SemanticTokensResult> {
    let key = FileKey::from(params.text_document.uri);
    let compilation = server.compilations.get_mut(&key).unwrap();
    let tokens = crate::semantic_tokens::get_semantic_tokens(compilation.source());
    Some(lsp_types::SemanticTokensResult::Tokens(lsp_types::SemanticTokens {
        result_id: None,
        data: tokens,
    }))
}

fn on_open(
    server: &mut TicServer,
    params: lsp_types::DidOpenTextDocumentParams,
) {
    let key = FileKey::from(params.text_document.uri);
    server.compilations.add_file(key.clone(), params.text_document.text);
    on_file_update(server, key);
}

fn on_change(
    server: &mut TicServer,
    params: lsp_types::DidChangeTextDocumentParams,
) {
    let key = FileKey::from(params.text_document.uri);
    server.compilations.set_source(key.clone(), params.content_changes[0].text.clone());
    on_file_update(server, key);
}

fn on_close(
    server: &mut TicServer,
    params: lsp_types::DidCloseTextDocumentParams,
) {
    let key = FileKey::from(params.text_document.uri);
    server.compilations.remove_file(&key);
}

fn on_file_update(
    server: &mut TicServer,
    file: FileKey,
) {
    send_diagnostics(server, file);
}

fn send_diagnostics(
    server: &mut TicServer,
    file: FileKey,
) {
    let compilation = server.compilations.get_mut(&file).unwrap();
    let diagnostics = crate::diagnostics::get_diagnostics(compilation.source());
    server.sender.send(lsp_server::Message::Notification(lsp_server::Notification {
        method: notification::PublishDiagnostics::METHOD.to_owned(),
        params: serde_json::to_value(lsp_types::PublishDiagnosticsParams {
            uri: file.0.clone(),
            diagnostics,
            version: None,
        }).unwrap(),
    })).unwrap();
}

pub(crate) fn handlers() -> Handlers {
    let mut handlers = Handlers::default();

    handlers.add_for_request::<request::SemanticTokensFullRequest>(semantic_tokens);

    handlers.add_for_notification::<notification::DidOpenTextDocument>(on_open);
    handlers.add_for_notification::<notification::DidChangeTextDocument>(on_change);
    handlers.add_for_notification::<notification::DidCloseTextDocument>(on_close);

    handlers
}
