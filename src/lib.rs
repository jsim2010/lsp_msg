//! Defines structures for interacting with LSP messages.
mod general;
mod structures;

pub use general::{
    DidChangeConfigurationCapabilities, DidChangeWatchedFilesCapabilities,
    ExecuteCommandCapabilities, FailureHandlingKind, ResourceOperationKind, SymbolCapabilities,
    SymbolKindCapabilities, WorkspaceClientCapabilities, WorkspaceEditCapabilities,
};
pub use lsp_msg_internal::{Elective, MarkupKind};
pub use structures::{Diagnostic, Range, Symbol, SymbolKind, TextDocumentItem};

use jsonrpc_core::Value;
use lsp_msg_derive::{lsp_kind, lsp_object};
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

/// The first request from the client to the server.
#[lsp_object]
pub struct InitializeParams {
    /// The process id of the process that started the server.
    ///
    /// If `Option::None`, the server has not been started.
    pub process_id: Option<u64>,
    /// The root path of the workspace.
    ///
    /// If `Option::None`, no folder is open.
    ///
    /// Deprecated in favor of `InitializeParams::root_uri`.
    pub root_path: Elective<Option<String>>,
    /// The root URI of the workspace.
    ///
    /// If `Option::None`, no folder is open. Else, overrides `InitializeParams::root_path`.
    pub root_uri: Option<String>,
    /// User provided initialization options.
    pub initialization_options: Elective<Value>,
    /// Capabilities provided by the client.
    pub capabilities: ClientCapabilities,
    /// The initial trace setting.
    #[serde(default)]
    pub trace: TraceKind,
    /// The workspace folders configured in the client.
    ///
    /// If `Elective::Absent`, client does not support workspace folders. If `Option::None`, client supports
    /// workspace folders but none are configured.
    pub workspace_folders: Elective<Option<Vec<WorkspaceFolder>>>,
}

/// Defines capabilities for dynamic registration, workspace and text document features the client
/// supports.
///
/// `experimental` can be used to pass experimental capabilities under development. For future
/// compatibility `ClientCapabilities` can have more properties set than currently defined.
#[lsp_object(allow_missing)]
pub struct ClientCapabilities {
    /// Workspace specific client capabilities.
    workspace: WorkspaceClientCapabilities,
    /// Text document specific client capabilities.
    text_document: TextDocumentClientCapabilities,
    /// Experimental client capabilities.
    experimental: Elective<Value>,
}

/// Defines capabilities the client provides on text documents.
#[lsp_object(allow_missing)]
struct TextDocumentClientCapabilities {
    /// Capabilities specific to synchronization.
    synchronization: SynchronizationCapabilities,
    /// Capabilities specific to the `textDocument/completion` request.
    completion: CompletionCapabilities,
    /// Capabilities specific to the `textDocument/hover` request.
    hover: HoverCapabilities,
    /// Capabilities specific to the `textDocument/signatureHelp` request.
    signature_help: SignatureHelpCapabilities,
    /// Capabilities specific to the `textDocument/references` request.
    references: ReferencesCapabilities,
    /// Capabilities specific to the `textDocument/documentHighlight` request.
    document_highlight: DocumentHighlightCapabilities,
    /// Capabilities specific to the `textDocument/documentSymbol` request.
    document_symbol: DocumentSymbolCapabilities,
    /// Capabilities specific to the `textDocument/formatting` request.
    formatting: FormattingCapabilities,
    /// Capabilities specific to the `textDocument/rangeFormatting` request.
    range_formatting: RangeFormattingCapabilities,
    /// Capabilities specific to the `textDocument/onTypeFormatting` request.
    on_type_formatting: OnTypeFormattingCapabilities,
    /// Capabilities specific to the `textDocument/declaration` request.
    declaration: DeclarationCapabilities,
    /// Capabilities specific to the `textDocument/definition` request.
    definition: DefinitionCapabilities,
    /// Capabilities specific to the `textDocument/typeDefinition` request.
    type_definition: TypeDefinitionCapabilities,
    /// Capabilities specific to the `textDocument/implementation` request.
    implementation: ImplementationCapabilities,
    /// Capabilities specific to the `textDocument/codeAction` request.
    code_action: CodeActionCapabilities,
    /// Capabilities specific to the `textDocument/codeLens` request.
    code_lens: CodeLensCapabilities,
    /// Capabilities specific to the `textDocument/documentLink` request.
    document_link: DocumentLinkCapabilities,
    /// Capabilities specific to the `textDocument/documentColor` and
    /// `textDocument/colorPresentation` requests.
    color_provider: ColorProviderCapabilities,
    /// Capabilities specific to the `textDocument/rename` request.
    rename: RenameCapabilities,
    /// Capabilities specific to the `textDocument/publishDiagnostics` request.
    publish_diagnostics: PublishDiagnosticsCapabilities,
    /// Capabilities specific to the `textDocument/foldingRange` request.
    folding_range: FoldingRangeCapabilities,
}

/// Defines capabilities specific to text document synchronization.
#[lsp_object(allow_missing, dynamic_registration = "text document synchronization")]
struct SynchronizationCapabilities {
    /// Supports the `textDocument/willSave` notification.
    will_save: bool,
    /// Supports the `textDocument/willSaveWaitUntil` notification.
    will_save_until: bool,
    /// Supports the `textDocument/didSave` notification.
    did_save: bool,
}

/// Defines capabilities specific to the `textDocument/completion` request.
#[lsp_object(
    allow_missing,
    dynamic_registration = "`textDocument/completion` request"
)]
struct CompletionCapabilities {
    /// Capabilities specific to `CompletionItem`s.
    completion_item: CompletionItemCapabilities,
    /// Capabilities specific to `CompletionItemKinds`s.
    completion_item_kind: CompletionItemKindCapabilities,
    /// Supports including additional context information in the `textDocument/completion` request.
    context_support: bool,
}

/// Defines capabilities specific to the `textDocument/hover` request.
#[lsp_object(
    allow_missing,
    dynamic_registration = "`textDocument/hover` request",
    markup_kind_list = "content"
)]
struct HoverCapabilities {}

/// Defines capabilities specific to the `textDocument/signatureHelp` request.
#[lsp_object(
    allow_missing,
    dynamic_registration = "`textDocument/signatureHelp` request"
)]
struct SignatureHelpCapabilities {
    /// Capabilities specific to `SignatureInformation`s.
    signature_information: SignatureInformationCapabilities,
}

/// Defines capabilities specific to the `textDocument/references` request.
#[lsp_object(
    allow_missing,
    dynamic_registration = "`textDocument/references` request"
)]
struct ReferencesCapabilities {}

/// Defines capabilities specific to the `textDocument/documentHighlight` request.
#[lsp_object(
    allow_missing,
    dynamic_registration = "`textDocument/documentHighlight` request"
)]
struct DocumentHighlightCapabilities {}

/// Defines capabilities specific to the `textDocument/documentSymbol` request.
#[lsp_object(
    allow_missing,
    dynamic_registration = "`textDocument/documentSymbol` request"
)]
struct DocumentSymbolCapabilities {
    /// Capabilities specific to `SymbolKind` in the `textDocument/documentSymbol` request.
    symbol_kind: SymbolKindCapabilities,
    /// Supports hierarchical document symbols.
    hierarchical_document_symbol_support: bool,
}

/// Defines capabilities specific to the `textDocument/formatting` request.
#[lsp_object(
    allow_missing,
    dynamic_registration = "`textDocument/formatting` request"
)]
struct FormattingCapabilities {}

/// Defines capabilities specific to the `textDocument/rangeFormatting` request.
#[lsp_object(
    allow_missing,
    dynamic_registration = "`textDocument/rangeFormatting` request"
)]
struct RangeFormattingCapabilities {}

/// Defines capabilities specific to the `textDocument/onTypeFormatting` request.
#[lsp_object(
    allow_missing,
    dynamic_registration = "`textDocument/onTypeFormatting` request"
)]
struct OnTypeFormattingCapabilities {}

/// Defines capabilities specific to the `textDocument/declaration` request.
#[lsp_object(
    allow_missing,
    dynamic_registration = "`textDocument/declaration` request",
    link_support = "declaration"
)]
struct DeclarationCapabilities {}

/// Defines capabilities specific to the `textDocument/definition` request.
#[lsp_object(
    allow_missing,
    dynamic_registration = "`textDocument/definition` request",
    link_support = "definition"
)]
struct DefinitionCapabilities {}

/// Defines capabilities specific to the `textDocument/typeDefinition` request.
#[lsp_object(
    allow_missing,
    dynamic_registration = "`textDocument/typeDefinition` request",
    link_support = "definition"
)]
struct TypeDefinitionCapabilities {}

/// Defines capabilities specific to the `textDocument/implementation` request.
#[lsp_object(
    allow_missing,
    dynamic_registration = "`textDocument/implementation` request",
    link_support = "implementation"
)]
struct ImplementationCapabilities {}

/// Defines capabilities specific to the `textDocument/codeAction` request.
#[lsp_object(
    allow_missing,
    dynamic_registration = "`textDocument/codeAction` request"
)]
struct CodeActionCapabilities {
    /// Capabilities specific to code action literals.
    code_action_literal_support: Elective<CodeActionLiteralCapabilities>,
}

/// Defines capabilities specific to the `textDocument/codeLens` request.
#[lsp_object(
    allow_missing,
    dynamic_registration = "`textDocument/codeLens` request"
)]
struct CodeLensCapabilities {}

/// Defines capabilities specific to the `textDocument/documentLink` request.
#[lsp_object(
    allow_missing,
    dynamic_registration = "`textDocument/documentLink` request"
)]
struct DocumentLinkCapabilities {}

/// Defines capabilities specific to the `textDocument/documentColor` and
/// `textDocument/colorPresentation` requests.
#[lsp_object(allow_missing, dynamic_registration = "color provider")]
struct ColorProviderCapabilities {}

/// Defines capabilities specific to the `textDocument/rename` request.
#[lsp_object(allow_missing, dynamic_registration = "`textDocument/rename` request")]
struct RenameCapabilities {
    /// Supports testing for validity of rename operations before execution.
    prepare_support: bool,
}

/// Defines capabilities specific to the `textDocument/publishDiagnostics` notification.
#[lsp_object(allow_missing)]
struct PublishDiagnosticsCapabilities {
    /// Supports diagnostics with related information.
    related_information: bool,
}

/// Defines capabilities specific to the `textDocument/foldingRange` request.
#[lsp_object(
    allow_missing,
    dynamic_registration = "`textDocument/foldingRange` request"
)]
struct FoldingRangeCapabilities {
    /// The preferred maximum number of folding ranges per document.
    ///
    /// Serves as a hint; servers are free to follow the limit.
    range_limit: u64,
    /// Only supports folding complete lines.
    line_folding_only: bool,
}

/// Describes capabilities specific to `CompletionItem`s.
#[lsp_object(allow_missing, markup_kind_list = "documentation")]
struct CompletionItemCapabilities {
    /// Supports snippets as insert text.
    snippet_support: bool,
    /// Supports commit characters on a `CompletionItem`.
    commit_characters_support: bool,
    /// Supports the deprecated property on a `CompletionItem`.
    deprecated_support: bool,
    /// Supports the preselect property on a `CompletionItem`.
    preselect_support: bool,
}

/// Describes capabilities specific to `CompletionItemKind`s.
#[lsp_object(value_set("CompletionItemKind", "CompletionItemKind::is_version1()"))]
struct CompletionItemKindCapabilities {}

/// Describes capabilities specific to `SignatureInformation`s.
#[lsp_object(allow_missing, markup_kind_list = "documentation")]
struct SignatureInformationCapabilities {
    /// Capabilities specific to parameter information.
    parameter_information: ParameterInformationCapabilities,
}

/// Describes capabilities specific to code action literals
#[lsp_object(allow_missing)]
struct CodeActionLiteralCapabilities {
    /// Capabilities specific to `CodeActionKind`s.
    code_action_kind: CodeActionKindCapabilities,
}

/// The kind of a `CompletionItem`.
#[lsp_kind(type = "string")]
#[derive(Clone, Copy, PartialOrd)]
pub enum CompletionItemKind {
    /// A text.
    Text = 1,
    /// A method.
    Method,
    /// A function.
    Function,
    /// A constructor.
    Constructor,
    /// A field.
    Field,
    /// A variable.
    Variable,
    /// A class.
    Class,
    /// An interface.
    Interface,
    /// A module.
    Module,
    /// A property.
    Property,
    /// A unit.
    Unit,
    /// A value.
    Value,
    /// An enum.
    Enum,
    /// A keyword.
    Keyword,
    /// A snippet.
    Snippet,
    /// A color.
    Color,
    /// A file.
    File,
    /// A reference.
    Reference,
    /// A folder.
    Folder,
    /// An enum member.
    EnumMember,
    /// A constant.
    Constant,
    /// A struct.
    Struct,
    /// An event.
    Event,
    /// An operator.
    Operator,
    /// A type parameter.
    TypeParameter,
    /// An unknown completion item kind.
    #[serde(other)]
    Unknown,
}

impl CompletionItemKind {
    /// Returns if `CompletionItemKind` is supported by version 1.
    pub fn is_version1(self) -> bool {
        self <= CompletionItemKind::Reference
    }
}

impl Default for CompletionItemKind {
    fn default() -> Self {
        CompletionItemKind::Text
    }
}

/// Describes capabilities specific to parameter information.
#[lsp_object(allow_missing)]
struct ParameterInformationCapabilities {
    /// Supports processing label offsets instread of a simple label string.
    label_offset_support: bool,
}

/// Describes capabilities specific to `CodeActionKind`s.
// TODO: String should be converted to CodeActionKind after finding a way to represent hierarchy of
// CodeActionKinds using serde.
#[lsp_object(value_set("String"))]
struct CodeActionKindCapabilities {}

/// The trace setting of the server.
#[lsp_kind(type = "string")]
#[derive(Clone, Copy)]
pub enum TraceKind {
    /// No messages are output.
    Off,
    /// Some messages are output.
    Messages,
    /// All messages are output.
    Verbose,
}

impl Default for TraceKind {
    fn default() -> Self {
        TraceKind::Off
    }
}

/// Describes a folder in a workspace.
#[lsp_object]
pub struct WorkspaceFolder {
    /// The associated URI.
    uri: String,
    /// The name as used in the user interface.
    name: String,
}

/// The result of a `initialize` request.
#[lsp_object]
pub struct InitializeResult {
    /// Capabilities provided by the server.
    pub capabilities: ServerCapabilities,
}

/// Describes capabilities provided by the server.
#[lsp_object(allow_missing)]
pub struct ServerCapabilities {
    /// Defines how documents are synced.
    text_document_sync: TextDocumentSyncProvider,
    /// Provides hover support.
    hover_provider: bool,
    /// Provides completion support.
    completion_provider: CompletionOptions,
    /// Provides signature help support.
    signature_help_provider: SignatureHelpOptions,
    /// Provides goto definition support.
    definition_provider: bool,
    /// Provides goto type definition support.
    type_definition_provider: BooleanOrOptions<GotoOptions>,
    /// Provides goto implementation support.
    implementation_provider: BooleanOrOptions<GotoOptions>,
    /// Provides find references support.
    references_provider: bool,
    /// Provides document highlight support.
    document_highlight_provider: bool,
    /// Provides document symbol support.
    document_symbol_provider: bool,
    /// Provides workspace symbol support.
    workspace_symbol_provider: bool,
    /// Provides code actions.
    code_action_provider: BooleanOrOptions<CodeActionOptions>,
    /// Provides code lens.
    code_lens_provider: CodeLensOptions,
    /// Provides document formatting.
    document_formatting_provider: bool,
    /// Provides document range formatting.
    document_range_formatting_provider: bool,
    /// Provides document formatting on typing.
    document_on_type_formatting_provider: DocumentOnTypeFormattingOptions,
    /// Provides rename support.
    rename_provider: BooleanOrOptions<RenameOptions>,
    /// Provides document link support.
    document_link_provider: DocumentLinkOptions,
    /// Provides color provider support.
    color_provider: BooleanOrOptionsOrStaticDocumentSelectorOptions<ColorProviderOptions>,
    /// Provides folding provider support.
    folding_range_provider:
        BooleanOrOptionsOrStaticDocumentSelectorOptions<FoldingRangeProviderOptions>,
    /// Provides goto declaration support.
    declaration_provider: BooleanOrOptions<GotoOptions>,
    /// Provides execute command support.
    execute_command_provider: ExecuteCommandOptions,
    /// Server capabilities specific to a workspace.
    workspace: WorkspaceOptions,
    /// Experimental server capabilities.
    experimental: Elective<Value>,
}

/// Information about text document synchronization.
#[lsp_kind]
pub enum TextDocumentSyncProvider {
    /// Options about the text documents to by synced.
    Options(TextDocumentSyncOptions),
    /// The kind of text Documents to be synced.
    Kind(TextDocumentSyncKind),
}

impl Default for TextDocumentSyncProvider {
    fn default() -> Self {
        TextDocumentSyncProvider::Kind(TextDocumentSyncKind::default())
    }
}

/// How the client should sync document changes with the server.
#[lsp_kind(type = "number")]
#[derive(Copy, Clone)]
pub enum TextDocumentSyncKind {
    /// Documents should not be synced at all.
    None = 0,
    /// Documents are synced by always sending the full content of the document.
    Full,
    /// Documents are synced by sending incremental updates.
    Incremental,
}

impl Default for TextDocumentSyncKind {
    fn default() -> Self {
        TextDocumentSyncKind::None
    }
}

/// Completion options.
#[lsp_object(
    allow_missing,
    trigger_characters = "completion",
    resolve_provider = "completion"
)]
struct CompletionOptions {}

/// Signature help options.
#[lsp_object(allow_missing, trigger_characters = "signature help")]
struct SignatureHelpOptions {}

#[lsp_object(static_registration)]
struct GotoOptions {
    /// Identifies the scope of the registration.
    ///
    /// If `Option::None`, `DocumentSelector` provided by client will be used.
    document_selector: Option<char>,
}

/// Either a boolean or `T`.
#[lsp_kind]
enum BooleanOrOptions<T> {
    /// A boolean.
    Boolean(bool),
    /// `T`.
    Options(T),
}

impl<T> Default for BooleanOrOptions<T> {
    fn default() -> Self {
        BooleanOrOptions::Boolean(false)
    }
}

/// Code Action options.
#[lsp_object(allow_missing)]
struct CodeActionOptions {
    // TODO: Use CodeActionKind when available.
    /// `CodeActionKind`s supported by server.
    code_action_kinds: Vec<String>,
}

/// Code lens options.
#[lsp_object(allow_missing, resolve_provider = "code lens")]
struct CodeLensOptions {}

/// Format document on type options.
#[lsp_object]
struct DocumentOnTypeFormattingOptions {
    /// Character on which formatting should be triggered.
    first_trigger_character: String,
    /// More trigger characters.
    #[serde(default)]
    more_trigger_character: Vec<String>,
}

/// Rename options.
#[lsp_object(allow_missing)]
struct RenameOptions {
    /// Renames should be checked and tested before being executed.
    prepare_provider: bool,
}

/// Document link options.
#[lsp_object(allow_missing, resolve_provider = "document links")]
struct DocumentLinkOptions {}

// TODO: Look into how to remove repetition for document_selector.
// TODO: Add DocumentSelector object.
#[lsp_object(static_registration)]
struct StaticDocumentSelectorOptions<T> {
    /// Identifies the scope of the registration.
    ///
    /// If `Option::None`, `DocumentSelector` provided by client will be used.
    document_selector: Option<char>,
    /// The options.
    options: T,
}

/// Color provider options.
#[lsp_object]
struct ColorProviderOptions {}

/// Folding range provider options.
#[lsp_object]
struct FoldingRangeProviderOptions {}

/// One of a boolean, `T`, or `StaticDocumentSelectorOptions<T>`.
#[lsp_kind]
enum BooleanOrOptionsOrStaticDocumentSelectorOptions<T> {
    /// A boolean.
    Boolean(bool),
    /// Only `T`.
    Options(T),
    /// Adds static registration and document selector fields to `T`.
    StaticDocumentSelectorOptions(StaticDocumentSelectorOptions<T>),
}

impl<T> Default for BooleanOrOptionsOrStaticDocumentSelectorOptions<T> {
    fn default() -> Self {
        BooleanOrOptionsOrStaticDocumentSelectorOptions::Boolean(false)
    }
}

/// Execute command options.
#[lsp_object]
struct ExecuteCommandOptions {
    /// Commands to be executed on the server.
    commands: Vec<String>,
}

/// Describes server capabilities specific to the workspace.
#[lsp_object(allow_missing)]
struct WorkspaceOptions {
    /// Capabilities specific to workspace folders.
    workspace_folders: WorkspaceFoldersOptions,
}

#[lsp_object(allow_missing)]
pub struct TextDocumentSyncOptions {
    /// Client sends open and close notifications to server.
    open_close: bool,
    /// Client sends change notifications to server.
    change: TextDocumentSyncKind,
    /// Client sends will save notifications to server.
    will_save: bool,
    /// Client sends will save wait until notifications to server.
    will_save_wait_until: bool,
    /// Client sends save notifications to server.
    save: SaveOptions,
}

/// Describes server capabilities specific to `WorkspaceFolder`s.
#[lsp_object(allow_missing)]
struct WorkspaceFoldersOptions {
    /// Supports workspace folders.
    supported: bool,
    /// Supports `WorkspaceFolder` change notifications.
    change_notifications: ChangeNotificationsOptions,
}

/// Save options.
#[lsp_object(allow_missing)]
struct SaveOptions {
    /// Content is included in save notifications.
    include_text: bool,
}

/// Change notification options.
#[lsp_kind]
enum ChangeNotificationsOptions {
    /// Supports change notifications.
    Boolean(bool),
    /// The identifier that can unregister change notifications.
    ///
    /// Specifies support for change notifications.
    Id(String),
}

impl Default for ChangeNotificationsOptions {
    fn default() -> Self {
        ChangeNotificationsOptions::Boolean(false)
    }
}

/// Notification sent from client to server after client receives `InitializeResult`.
#[lsp_object]
pub struct InitializedParams {}

/// Request sent from server to client to register for a new capability on the client side.
#[lsp_object]
pub struct RegistrationParams {
    /// Registrations requested by the server.
    pub registrations: Vec<Registration>,
}

/// General parameters to register for a capability.
#[lsp_object]
pub struct Registration {
    /// Id associated with the request.
    id: String,
    /// Method/capability to register for.
    method: String,
    /// Options necessary for the registration.
    register_options: Elective<Value>,
}

/// Response to `client/registerCapability` request.
#[lsp_object]
struct RegistrationResult {}

/// Notification sent from the client to server to signal newly opened text documents.
#[lsp_object]
pub struct DidOpenTextDocumentParams {
    /// Document that was opened.
    text_document: TextDocumentItem,
}

impl From<TextDocumentItem> for DidOpenTextDocumentParams {
    fn from(text_document: TextDocumentItem) -> Self {
        Self { text_document }
    }
}

/// Notification sent from client to server to signal changes to a text document.
#[lsp_object]
pub struct DidChangeTextDocumentParams {
    /// Document that changed.
    ///
    /// Version number identifies version after all provided content changes have been applied.
    text_document: VersionedTextDocumentIdentifier,
    /// Changes to the content.
    content_changes: Vec<TextDocumentContentChangeEvent>,
}

impl DidChangeTextDocumentParams {
    /// Creates a new `DidChangeTextDocumentParams`.
    pub const fn new(
        text_document: VersionedTextDocumentIdentifier,
        content_changes: Vec<TextDocumentContentChangeEvent>,
    ) -> Self {
        Self {
            text_document,
            content_changes,
        }
    }
}

/// Denotes a specific version of a text document.
#[lsp_object]
pub struct VersionedTextDocumentIdentifier {
    /// URI of text document.
    uri: String,
    /// Version number of the document.
    ///
    /// If `Option::None`, content on disk is the truth.
    version: Option<i64>,
}

impl From<TextDocumentItem> for VersionedTextDocumentIdentifier {
    fn from(value: TextDocumentItem) -> Self {
        Self {
            uri: value.uri.clone(),
            version: Some(value.version),
        }
    }
}

/// Describes a change to a text document.
///
/// If `range` and `range_length` are `Elective::absent`, `text` is the full content of the
/// document.
#[lsp_object]
pub struct TextDocumentContentChangeEvent {
    /// `Range` of the changed document.
    range: Elective<Range>,
    /// Length of the `Range`.
    range_length: Elective<u64>,
    /// New text of the `Range`.
    text: String,
}

impl TextDocumentContentChangeEvent {
    /// Creates a new `TextDocumentContentChangeEvent`.
    pub fn new(range: Range, text: String) -> Self {
        Self {
            range: Elective::Present(range),
            range_length: Elective::Absent,
            text,
        }
    }
}

/// Notification sent from the server to the client to signal results of validation runs.
#[lsp_object]
pub struct PublishDiagnosticsParams {
    /// URI of document for which diagnostic information is reported.
    uri: String,
    /// Diagnostic information items.
    diagnostics: Vec<Diagnostic>,
}
