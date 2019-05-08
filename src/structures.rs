//! Defines LSP objects that are basic structures.
use jsonrpc_core::Value;
use lsp_msg_derive::{lsp_kind, lsp_object};
use lsp_msg_internal::{Elective, MarkupKind};
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use std::collections::HashMap;

/// A URI as specified by IETF RFC 3986.
type DocumentUri = String;

/// A line and character gap offset of a text document.
#[lsp_object]
#[derive(Clone, Copy, Eq)]
pub struct Position {
    /// Zero-based index of the line.
    pub line: u64,
    /// Zero-based index of the character gap within the given line.
    ///
    /// If `character` is greater than the line length (not including line ending characters), it
    /// defaults to the line length.
    pub character: u64,
}

impl Position {
    /// Moves 1 line up.
    pub fn move_up(&mut self) {
        self.line -= 1;
    }

    /// Moves to the end of the line.
    pub fn move_to_end_of_line(&mut self) {
        self.character = u64::max_value();
    }

    /// Moves 1 character to the left.
    pub fn move_left(&mut self) {
        self.character -= 1;
    }

    /// Moves 1 character to the right.
    pub fn move_right(&mut self) {
        self.character += 1;
    }

    /// `Position` is at the start of its line.
    pub const fn is_first_character(&self) -> bool {
        self.character == 0
    }

    /// `Position` is at the first line in its text document.
    pub const fn is_first_line(&self) -> bool {
        self.line == 0
    }
}

/// `Position`s in between 2 given `Position`s.
///
/// The start `Position` is inclusive while the end `Position` is exclusive.
#[lsp_object]
#[derive(Clone, Copy, Eq)]
pub struct Range {
    /// Start `Position` of the `Range`.
    pub start: Position,
    /// End `Position` of the `Range`.
    pub end: Position,
}

impl Range {
    /// Creates a new `Range` that describes an entire line.
    ///
    /// An entire line does not include line ending characters.
    pub const fn entire_line(line: u64) -> Self {
        Self::partial_line(line, 0, u64::max_value())
    }

    /// Creates a new `Range` that describes the specified characters on a line.
    pub const fn partial_line(line: u64, start: u64, end: u64) -> Self {
        Self {
            start: Position {
                line,
                character: start,
            },
            end: Position {
                line,
                character: end,
            },
        }
    }
}

impl From<Position> for Range {
    fn from(value: Position) -> Self {
        Self {
            start: value,
            end: value,
        }
    }
}

/// A part of a text document.
#[lsp_object]
struct Location {
    /// The URI of the text document.
    uri: DocumentUri,
    /// The `Range` within the text document.
    range: Range,
}

/// A link between an origin and a target `Location`.
#[lsp_object]
struct LocationLink {
    /// Origin of this link.
    ///
    /// If `Elective::Absent`, set to the `Range` of the word at the mouse position.
    origin_selection_range: Elective<Range>,
    /// `DocumentUri` of the target.
    target_uri: DocumentUri,
    /// The full `Range` of the target.
    target_range: Range,
    /// The `Range` that should be selected when the link is followed.
    target_selection_range: Range,
}

/// A diagnostic such as a compiler error or warning.
#[lsp_object]
pub struct Diagnostic {
    /// `Range` at which the message applies.
    range: Range,
    /// The severity of the diagnostic.
    ///
    /// If `Elective::Absent`, client is responsible for interpreting severity.
    severity: Elective<DiagnosticSeverity>,
    /// Code of the diagnostic.
    code: Elective<DiagnosticCode>,
    /// Human-readable description of the source of the diagnostic.
    source: Elective<String>,
    /// Message of the diagnostic.
    message: String,
    /// Related information about a diagnostic.
    related_information: Elective<Vec<DiagnosticRelatedInformation>>,
}

/// Supported severities of a diagnostic.
#[lsp_kind(type = "number")]
enum DiagnosticSeverity {
    /// A `Diagnostic` that prevents successful completion.
    Error = 1,
    /// A `Diagnostic` that does not prevent successful completion but may need to be addressed.
    Warning,
    /// A `Diagnotic` that provides noncritical information that does not need to be addressed.
    Information,
    /// A `Diagnostic` that may assist debugging efforts.
    Hint,
}

/// A code representing a `Diagnostic`.
#[lsp_kind]
enum DiagnosticCode {
    /// Number format.
    Number(i64),
    /// String format.
    String(String),
}

/// A related message for a `Diagnostic`.
#[lsp_object]
struct DiagnosticRelatedInformation {
    /// Location of the related information.
    location: Location,
    /// Message of the related information.
    message: String,
}

/// A command.
#[lsp_object]
struct Command {
    /// Representation of the command in the UI.
    title: String,
    /// Identifier of the command handler.
    command: String,
    /// Arguments the command handler should be invoked with.
    arguments: Vec<Value>,
}

/// A textual edit of a text document.
#[lsp_object]
struct TextEdit {
    /// `Range` of the text document to be manipulated.
    range: Range,
    /// String to replace the text in the given `Range`.
    new_text: String,
}

/// Textual changes of a given text document.
#[lsp_object]
struct TextDocumentEdit {
    /// Text document to change.
    ///
    /// Version is set to the current version, prior to the changes being made.
    text_document: VersionedTextDocumentIdentifier,
    /// `TextEdit`s to be applied.
    edits: Vec<TextEdit>,
}

/// Options to create a file.
#[lsp_object(allow_missing)]
struct CreateFileOptions {
    /// If existing file is overwritten.
    overwrite: bool,
    /// If existing file is ignored.
    ignore_if_exists: bool,
}

/// Delete file options.
#[lsp_object(allow_missing)]
struct DeleteFileOptions {
    /// Delete the content recursively.
    recursive: bool,
    /// Ignore the operation if resource does not exist.
    ignore_if_not_exists: bool,
}

/// Resource operation.
#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase", tag = "kind")]
enum ResourceOperation {
    /// Create resource operation.
    Create {
        /// Resource to create.
        uri: DocumentUri,
        /// Additional options.
        options: Elective<CreateFileOptions>,
    },
    /// Rename resource operation.
    Rename {
        /// The existing resource.
        old_uri: DocumentUri,
        /// The new resource.
        new_uri: DocumentUri,
        /// Rename options.
        options: Elective<CreateFileOptions>,
    },
    /// Delete resource operation.
    Delete {
        /// Resource to delete.
        uri: DocumentUri,
        /// Delete options.
        options: Elective<DeleteFileOptions>,
    },
}

/// Changes to many resources managed in the workspace.
#[lsp_object]
struct WorkspaceEdit {
    #[serde(flatten)]
    /// Changes to a workspace.
    changes: WorkspaceChanges,
}

/// Changes to many resources managed in the workspace.
#[lsp_kind]
enum WorkspaceChanges {
    /// Changes to a text document.
    Changes(HashMap<DocumentUri, Vec<TextEdit>>),
    /// Changes to any part of a workspace.
    DocumentChanges(Vec<Change>),
}

impl Default for WorkspaceChanges {
    fn default() -> Self {
        WorkspaceChanges::Changes(HashMap::new())
    }
}

/// Change.
#[lsp_kind]
enum Change {
    /// Change to the text of a text document.
    TextDocument(TextDocumentEdit),
    /// Change to a resource.
    Resource(ResourceOperation),
}

/// Identifier of a text document.
#[lsp_object]
struct TextDocumentIdentifier {
    /// Uri of the text document.
    uri: DocumentUri,
}

/// An item to transfer a text document from the client to the server.
#[lsp_object]
#[derive(Clone)]
pub struct TextDocumentItem {
    /// URI of text document.
    pub uri: String,
    /// Language identifier of text document.
    pub language_id: LanguageId,
    /// Version number of text document.
    pub version: i64,
    /// Content of the text document.
    pub text: String,
}

impl TextDocumentItem {
    /// Increments the version.
    pub fn increment_version(&mut self) {
        self.version += 1;
    }
}

/// A language identifer of a text document.
#[lsp_kind]
#[derive(Clone)]
pub enum LanguageId {
    /// A language id that has been defined.
    Defined(LanguageIdKind),
    /// A language id that has not been defined.
    Undefined(String),
}

impl Default for LanguageId {
    fn default() -> Self {
        LanguageId::Undefined(String::default())
    }
}

/// The defined language ids.
#[lsp_kind(type = "language_id")]
#[derive(Clone, Copy)]
pub enum LanguageIdKind {
    /// Windows Bat language.
    Bat,
    /// BibTeX language.
    Bibtex,
    /// Clojure language.
    Clojure,
    /// Coffeescript language.
    Coffeescript,
    /// C language.
    C,
    /// C++ language.
    Cpp,
    /// C# language.
    Csharp,
    /// CSS language.
    Css,
    /// Diff language.
    Diff,
    /// Dart language.
    Dart,
    /// Dockerfile language.
    Dockerfile,
    /// F# language.
    Fsharp,
    /// Git commit message format.
    GitCommit,
    /// Git rebase message format.
    GitRebase,
    /// Go language.
    Go,
    /// Groovy language.
    Groovy,
    /// Handlebars language.
    Handlebars,
    /// HTML language.
    Html,
    /// Ini language.
    Ini,
    /// Java language.
    Java,
    /// JavaScript language.
    Javascript,
    /// JSON language.
    Json,
    /// LaTeX language.
    Latex,
    /// Less language.
    Less,
    /// Lua language.
    Lua,
    /// Makefile language.
    Makefile,
    /// Markdown language.
    Markdown,
    /// Objective-C language.
    ObjectiveC,
    /// Objective-C++ language.
    ObjectiveCpp,
    /// Perl language.
    Perl,
    /// Perl 6 language.
    Perl6,
    /// PHP language.
    Php,
    /// Powershell language.
    Powershell,
    /// Pug language.
    Jade,
    /// Python language.
    Python,
    /// R language.
    R,
    /// Razor (cshtml) language.
    Razor,
    /// Ruby language.
    Ruby,
    /// Rust language.
    Rust,
    /// Sass language with curly bracket syntax.
    Scss,
    /// Sass language with indented syntax.
    Sass,
    /// Scala language.
    Scala,
    /// ShaderLab language.
    Shaderlab,
    /// Shell Script (Bash) language.
    Shellscript,
    /// SQL language.
    Sql,
    /// Swift language.
    Swift,
    /// TypeScript language.
    Typescript,
    /// TeX language.
    Tex,
    /// Visual Basic language.
    Vb,
    /// XML language.
    Xml,
    /// XSL language.
    Xsl,
    /// YAML language.
    Yaml,
}

/// Identifier of a specific version of a text document.
#[lsp_object]
struct VersionedTextDocumentIdentifier {
    /// The document identifier.
    #[serde(flatten)]
    id: TextDocumentIdentifier,
    /// Version of the document.
    version: Option<i64>,
}

/// A `Position` inside a text document.
#[lsp_object]
struct TextDocumentPositionParams {
    /// The text document.
    text_document: TextDocumentIdentifier,
    /// The `Position` inside the text document.
    position: Position,
}

/// A filter that can be applied to documents.
#[lsp_object]
struct DocumentFilter {
    /// A `LanguageId`.
    language: LanguageId,
    /// A URI scheme.
    scheme: String,
    /// A glob pattern.
    pattern: String,
}

/// A string value which can be represented in different formats.
#[lsp_object]
struct MarkupContent {
    /// The type of the markup.
    kind: MarkupKind,
    /// The content itself.
    value: String,
}

/// A symbol kind.
#[lsp_kind]
#[derive(Clone, Copy)]
pub enum SymbolKind {
    /// A known `Symbol`.
    Known(Symbol),
    /// A value that is not known to map to a `Symbol`.
    #[serde(skip_serializing)]
    Unknown(u8),
}

/// The value of a symbol kind.
#[lsp_kind(type = "number")]
#[derive(Clone, Copy, PartialOrd)]
pub enum Symbol {
    /// A file.
    File = 1,
    /// A module.
    Module,
    /// A namespace.
    Namespace,
    /// A package.
    Package,
    /// A class.
    Class,
    /// A method.
    Method,
    /// A property.
    Property,
    /// A field.
    Field,
    /// A constructor.
    Constructor,
    /// An enum.
    Enum,
    /// An interface.
    Interface,
    /// A function.
    Function,
    /// A variable.
    Variable,
    /// A constant.
    Constant,
    /// A string.
    String,
    /// A number.
    Number,
    /// A boolean.
    Boolean,
    /// An array.
    Array,
    /// An object.
    Object,
    /// A key.
    Key,
    /// A null.
    Null,
    /// An enum member.
    EnumMember,
    /// A struct.
    Struct,
    /// An event.
    Event,
    /// An operator.
    Operator,
    /// A type parameter.
    TypeParameter,
}

impl Symbol {
    /// Returns if `Symbol` is supported in version 1.
    pub fn is_version1(self) -> bool {
        self <= Symbol::Array
    }
}

impl Default for Symbol {
    fn default() -> Self {
        Symbol::Property
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_test::{
        assert_de_tokens, assert_ser_tokens, assert_ser_tokens_error, assert_tokens, Token,
    };

    mod symbol_kind {
        use super::*;

        #[test]
        fn serde() {
            assert_de_tokens(&SymbolKind::Unknown(0), &[Token::U8(0)]);
            assert_de_tokens(&SymbolKind::Unknown(255), &[Token::U8(255)]);
            assert_ser_tokens(&SymbolKind::Known(Symbol::Module), &[Token::U8(2)]);
            assert_ser_tokens_error(
                &SymbolKind::Unknown(0),
                &[],
                "the enum variant SymbolKind::Unknown cannot be serialized",
            );
        }
    }

    mod symbol {
        use super::*;

        #[test]
        fn serde() {
            assert_tokens(&Symbol::File, &[Token::U8(1)]);
            assert_tokens(&Symbol::Module, &[Token::U8(2)]);
            assert_tokens(&Symbol::Namespace, &[Token::U8(3)]);
            assert_tokens(&Symbol::Package, &[Token::U8(4)]);
            assert_tokens(&Symbol::Class, &[Token::U8(5)]);
            assert_tokens(&Symbol::Method, &[Token::U8(6)]);
            assert_tokens(&Symbol::Property, &[Token::U8(7)]);
            assert_tokens(&Symbol::Field, &[Token::U8(8)]);
            assert_tokens(&Symbol::Constructor, &[Token::U8(9)]);
            assert_tokens(&Symbol::Enum, &[Token::U8(10)]);
            assert_tokens(&Symbol::Interface, &[Token::U8(11)]);
            assert_tokens(&Symbol::Function, &[Token::U8(12)]);
            assert_tokens(&Symbol::Variable, &[Token::U8(13)]);
            assert_tokens(&Symbol::Constant, &[Token::U8(14)]);
            assert_tokens(&Symbol::String, &[Token::U8(15)]);
            assert_tokens(&Symbol::Number, &[Token::U8(16)]);
            assert_tokens(&Symbol::Boolean, &[Token::U8(17)]);
            assert_tokens(&Symbol::Array, &[Token::U8(18)]);
            assert_tokens(&Symbol::Object, &[Token::U8(19)]);
            assert_tokens(&Symbol::Key, &[Token::U8(20)]);
            assert_tokens(&Symbol::Null, &[Token::U8(21)]);
            assert_tokens(&Symbol::EnumMember, &[Token::U8(22)]);
            assert_tokens(&Symbol::Struct, &[Token::U8(23)]);
            assert_tokens(&Symbol::Event, &[Token::U8(24)]);
            assert_tokens(&Symbol::Operator, &[Token::U8(25)]);
            assert_tokens(&Symbol::TypeParameter, &[Token::U8(26)]);
        }

        #[test]
        fn default() {
            assert_eq!(Symbol::default(), Symbol::Property);
        }
    }
}
