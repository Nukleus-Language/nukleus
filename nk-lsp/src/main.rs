//! Nukleus Language Server Protocol skeleton.
//! Consumes stable lexer/parser APIs and publishes diagnostics on document changes.

mod diagnostics;

use std::path::Path;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer, LspService, Server};

struct NukleusBackend {
    client: Client,
}

#[tower_lsp::async_trait]
impl LanguageServer for NukleusBackend {
    async fn initialize(
        &self,
        _params: InitializeParams,
    ) -> tower_lsp::jsonrpc::Result<InitializeResult> {
        let capability = ServerCapabilities {
            text_document_sync: Some(TextDocumentSyncCapability::Options(
                TextDocumentSyncOptions {
                    open_close: Some(true),
                    change: Some(TextDocumentSyncKind::FULL),
                    ..Default::default()
                },
            )),
            ..Default::default()
        };

        Ok(InitializeResult {
            capabilities: capability,
            server_info: Some(ServerInfo {
                name: "nk-lsp".into(),
                version: Some(env!("CARGO_PKG_VERSION").into()),
            }),
        })
    }

    async fn initialized(&self, _: InitializedParams) {
        self.client
            .log_message(MessageType::INFO, "nk-lsp initialized")
            .await;
    }

    async fn shutdown(&self) -> tower_lsp::jsonrpc::Result<()> {
        Ok(())
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        self.publish_diagnostics(params.text_document.uri, params.text_document.text)
            .await;
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        let content = params
            .content_changes
            .into_iter()
            .next()
            .map(|c| c.text)
            .unwrap_or_default();
        self.publish_diagnostics(params.text_document.uri, content)
            .await;
    }

    async fn did_close(&self, params: DidCloseTextDocumentParams) {
        self.client
            .publish_diagnostics(params.text_document.uri, vec![], None)
            .await;
    }
}

impl NukleusBackend {
    async fn publish_diagnostics(&self, uri: Url, source: String) {
        let path = uri.to_file_path().ok();
        let path_buf = path.unwrap_or_else(|| Path::new("untitled.nk").to_path_buf());

        let mut diags: Vec<Diagnostic> = Vec::new();

        let mut lexer = nk_lexer::Lexer::new(path_buf.clone(), &source);
        if let Err(e) = lexer.run() {
            diags.push(diagnostics::to_lsp_diagnostic(&e));
        } else {
            let tokens = lexer.tokens().to_vec();
            let mut parser = astgen::parser::Parser::new(&tokens, path_buf, &source);
            if let Err(e) = parser.run() {
                diags.push(diagnostics::to_lsp_diagnostic(&e.to_diagnostic()));
            }
        }

        self.client.publish_diagnostics(uri, diags, None).await;
    }
}

#[tokio::main]
async fn main() {
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, socket) = LspService::new(|client| NukleusBackend { client });
    Server::new(stdin, stdout, socket).serve(service).await;
}
