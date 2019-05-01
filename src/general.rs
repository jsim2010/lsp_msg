//! Defines LSP objects under the General category.
use lsp_msg_derive::{lsp_kind, lsp_object};
use lsp_msg_internal::Elective;
use serde::{Deserialize, Serialize};
use spec::spec;

/// Defines capabilities specific to `WorkspaceEdit`s.
#[spec(
    name = "serde",
    shall = "implement the `Deserialize` and `Serialize` traits",
    cert {
        use serde_test::{assert_tokens, Token};
        use lsp_msg::{Elective, FailureHandlingKind, ResourceOperationKind, WorkspaceEditCapabilities};

        let workspace_edit_capabilities = WorkspaceEditCapabilities {
            document_changes: false,
            resource_operations: vec![ResourceOperationKind::Create],
            failure_handling: Elective::Present(FailureHandlingKind::Abort),
        };

        assert_tokens(&workspace_edit_capabilities, &[
            Token::Struct { name: "WorkspaceEditCapabilities", len: 3 },
            Token::String("documentChanges"),
            Token::Bool(false),
            Token::String("resourceOperations"),
            Token::Seq { len: Some(1) },
            Token::UnitVariant { name: "ResourceOperationKind", variant: "create" },
            Token::SeqEnd,
            Token::String("failureHandling"),
            Token::UnitVariant { name: "FailureHandlingKind", variant: "abort" },
            Token::StructEnd,
        ]);
    }
)]
#[spec(
    name = "default",
    shall = "implement the `Default` trait",
    cert {
        use lsp_msg::{Elective, WorkspaceEditCapabilities};

        let default_value = WorkspaceEditCapabilities {
            document_changes: bool::default(),
            resource_operations: Vec::default(),
            failure_handling: Elective::default(),
        };

        assert_eq!(WorkspaceEditCapabilities::default(), default_value);
    }
)]
#[spec(
    name = "deserialize_missing",
    cond = "is missing fields, shall deserialize to default",
    cert {
        use lsp_msg::WorkspaceEditCapabilities;
        use serde_test::{assert_de_tokens, Token};

        assert_de_tokens(&WorkspaceEditCapabilities::default(), &[
            Token::Struct { name: "WorkspaceEditCapabilities", len: 0 },
            Token::StructEnd,
        ]);
    }
)]
#[spec(
    name = "serialize_absence",
    cond = "has failure_handling absent, shall skip serializing field",
    cert {
        use lsp_msg::{Elective, WorkspaceEditCapabilities};
        use serde_test::{assert_ser_tokens, Token};

        let object = WorkspaceEditCapabilities {
            document_changes: false,
            resource_operations: vec![],
            failure_handling: Elective::Absent,
        };

        assert_ser_tokens(&object, &[
            Token::Struct { name: "WorkspaceEditCapabilities", len: 2 },
            Token::String("documentChanges"),
            Token::Bool(false),
            Token::String("resourceOperations"),
            Token::Seq { len: Some(0) },
            Token::SeqEnd,
            Token::StructEnd,
        ]);
    }
)]
#[lsp_object(allow_missing)]
pub struct WorkspaceEditCapabilities {
    /// Supports versioned document changes in `WorkspaceEdit`s.
    pub document_changes: bool,
    /// The supported resource operations.
    pub resource_operations: Vec<ResourceOperationKind>,
    // Use Elective because an absence of the FailureHandlingKind capability is not defined.
    /// The failure handling strategy if applying the `WorkspaceEdit` fails.
    pub failure_handling: Elective<FailureHandlingKind>,
}

/// The kind of resource operations.
#[spec(
    name = "serde",
    shall = "implement the `Deserialize` and `Serialize` traits",
    cert {
        use serde_test::{assert_tokens, Token};
        use lsp_msg::ResourceOperationKind;

        assert_tokens(&ResourceOperationKind::Create, &[
            Token::UnitVariant { name: "ResourceOperationKind", variant: "create" },
        ]);
        assert_tokens(&ResourceOperationKind::Rename, &[
            Token::UnitVariant { name: "ResourceOperationKind", variant: "rename" },
        ]);
        assert_tokens(&ResourceOperationKind::Delete, &[
            Token::UnitVariant { name: "ResourceOperationKind", variant: "delete" },
        ]);
    }
)]
#[lsp_kind(type = "string")]
pub enum ResourceOperationKind {
    /// Creating new files and folders.
    Create,
    /// Renaming existing files and folders.
    Rename,
    /// Deleting existing files and folders.
    Delete,
}

/// The strategy of the client to handle a failure to apply a `WorkspaceEdit`.
#[spec(
    name = "serde",
    shall = "implement the `Deserialize` and `Serialize` traits",
    cert {
        use serde_test::{assert_tokens, Token};
        use lsp_msg::FailureHandlingKind;

        assert_tokens(&FailureHandlingKind::Abort, &[
            Token::UnitVariant { name: "FailureHandlingKind", variant: "abort" },
        ]);
        assert_tokens(&FailureHandlingKind::Transactional, &[
            Token::UnitVariant { name: "FailureHandlingKind", variant: "transactional" },
        ]);
        assert_tokens(&FailureHandlingKind::TextOnlyTransactional, &[
            Token::UnitVariant { name: "FailureHandlingKind", variant: "textOnlyTransactional" },
        ]);
        assert_tokens(&FailureHandlingKind::Undo, &[
            Token::UnitVariant { name: "FailureHandlingKind", variant: "undo" },
        ]);
    }
)]
#[lsp_kind(type = "string")]
pub enum FailureHandlingKind {
    /// Operations are simply aborted if one of the changes fails.
    ///
    /// All operations executed before the failing operation stayed executed.
    Abort,
    /// All operations are executed transactional.
    ///
    /// Either all operations succeed or no changes are applied to the workspace.
    Transactional,
    /// Textual file changes are executed transactional and resource changes are abort.
    TextOnlyTransactional,
    /// Client tries to undo operations already executed.
    ///
    /// There is no guarantee the undo is successful.
    Undo,
}
