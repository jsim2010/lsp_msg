//! Defines LSP objects under the General category.
use crate::structures::SymbolKind;
use lsp_msg_derive::{lsp_kind, lsp_object};
use lsp_msg_internal::Elective;
use serde::{Deserialize, Serialize};

/// Defines capabilities the client provides on the workspace.
#[lsp_object(allow_missing)]
pub struct WorkspaceClientCapabilities {
    /// Supports applying batch edits to the workspace by the request `workspace/applyEdit`.
    apply_edit: bool,
    /// Capabilities specific to `WorkspaceEdit`s.
    workspace_edit: WorkspaceEditCapabilities,
    /// Capabilities specific to the `workspace/didChangeConfiguration` notification.
    did_change_configuration: DidChangeConfigurationCapabilities,
    /// Capabilities specific to the `worksapce/didChangeWatchedFiles` notification.
    did_change_watched_files: DidChangeWatchedFilesCapabilities,
    /// Capabilities specific to the `workspace/symbol` request.
    symbol: SymbolCapabilities,
    /// Capabilities specific to the `workspace/executeCommand` request.
    execute_command: ExecuteCommandCapabilities,
    /// Supports workspace folders.
    workspace_folders: bool,
    /// Supports `workspace/configuration` requests.
    configuration: bool,
}

/// Defines capabilities specific to `WorkspaceEdit`s.
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
#[lsp_kind(type = "string")]
#[derive(Clone, Copy)]
pub enum ResourceOperationKind {
    /// Creating new files and folders.
    Create,
    /// Renaming existing files and folders.
    Rename,
    /// Deleting existing files and folders.
    Delete,
}

/// The strategy of the client to handle a failure to apply a `WorkspaceEdit`.
#[lsp_kind(type = "string")]
#[derive(Clone, Copy)]
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

/// Defines capabilities specific to the `workspace/didChangeConfiguration` notification.
#[lsp_object(
    allow_missing,
    dynamic_registration = "`workspace/didChangeConfiguration` notification"
)]
pub struct DidChangeConfigurationCapabilities {}

/// Defines capabilities specific to the `workspace/didChangeWatchedFiles` notification.
#[lsp_object(
    allow_missing,
    dynamic_registration = "`workspace/didChangeWatchedFiles` notification"
)]
pub struct DidChangeWatchedFilesCapabilities {}

/// Describes capabilities specific to `SymbolKind`s.
#[lsp_object(value_set(SymbolKind, "Symbol::is_version1()"))]
pub struct SymbolKindCapabilities {}

/// Defines capabilities specific to the `workspace/symbol` request.
#[lsp_object(allow_missing, dynamic_registration = "`workspace/symbol` request")]
pub struct SymbolCapabilities {
    /// Capabilities specific to the `SymbolKind` in the `workspace/symbol` request.
    pub symbol_kind: SymbolKindCapabilities,
}

/// Defines capabilities specific to the `workspace/executeCommand` request.
#[lsp_object(
    allow_missing,
    dynamic_registration = "`workspace/executeCommand` request"
)]
pub struct ExecuteCommandCapabilities {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::structures::Symbol;
    use serde_test::{assert_de_tokens, assert_ser_tokens, assert_tokens, Token};

    mod workspace_edit_capabilities {
        use super::*;

        #[test]
        fn serde() {
            let obj = WorkspaceEditCapabilities {
                document_changes: false,
                resource_operations: vec![ResourceOperationKind::Create],
                failure_handling: Elective::Present(FailureHandlingKind::Abort),
            };

            assert_tokens(
                &obj,
                &[
                    Token::Struct {
                        name: "WorkspaceEditCapabilities",
                        len: 3,
                    },
                    Token::String("documentChanges"),
                    Token::Bool(false),
                    Token::String("resourceOperations"),
                    Token::Seq { len: Some(1) },
                    Token::UnitVariant {
                        name: "ResourceOperationKind",
                        variant: "create",
                    },
                    Token::SeqEnd,
                    Token::String("failureHandling"),
                    Token::UnitVariant {
                        name: "FailureHandlingKind",
                        variant: "abort",
                    },
                    Token::StructEnd,
                ],
            );
        }

        #[test]
        fn default() {
            assert_eq!(
                WorkspaceEditCapabilities::default(),
                WorkspaceEditCapabilities {
                    document_changes: false,
                    resource_operations: vec![],
                    failure_handling: Elective::Absent,
                }
            );
        }

        #[test]
        fn deserialize_missing() {
            assert_de_tokens(
                &WorkspaceEditCapabilities::default(),
                &[
                    Token::Struct {
                        name: "WorkspaceEditCapabilities",
                        len: 0,
                    },
                    Token::StructEnd,
                ],
            );
        }

        #[test]
        fn serialize_absent() {
            let object = WorkspaceEditCapabilities {
                document_changes: false,
                resource_operations: vec![],
                failure_handling: Elective::Absent,
            };

            assert_ser_tokens(
                &object,
                &[
                    Token::Struct {
                        name: "WorkspaceEditCapabilities",
                        len: 2,
                    },
                    Token::String("documentChanges"),
                    Token::Bool(false),
                    Token::String("resourceOperations"),
                    Token::Seq { len: Some(0) },
                    Token::SeqEnd,
                    Token::StructEnd,
                ],
            );
        }
    }

    mod resource_operation_kind {
        use super::*;

        #[test]
        fn serde() {
            assert_tokens(
                &ResourceOperationKind::Create,
                &[Token::UnitVariant {
                    name: "ResourceOperationKind",
                    variant: "create",
                }],
            );
            assert_tokens(
                &ResourceOperationKind::Rename,
                &[Token::UnitVariant {
                    name: "ResourceOperationKind",
                    variant: "rename",
                }],
            );
            assert_tokens(
                &ResourceOperationKind::Delete,
                &[Token::UnitVariant {
                    name: "ResourceOperationKind",
                    variant: "delete",
                }],
            );
        }
    }

    mod failure_handling_kind {
        use super::*;

        #[test]
        fn serde() {
            assert_tokens(
                &FailureHandlingKind::Abort,
                &[Token::UnitVariant {
                    name: "FailureHandlingKind",
                    variant: "abort",
                }],
            );
            assert_tokens(
                &FailureHandlingKind::Transactional,
                &[Token::UnitVariant {
                    name: "FailureHandlingKind",
                    variant: "transactional",
                }],
            );
            assert_tokens(
                &FailureHandlingKind::TextOnlyTransactional,
                &[Token::UnitVariant {
                    name: "FailureHandlingKind",
                    variant: "textOnlyTransactional",
                }],
            );
            assert_tokens(
                &FailureHandlingKind::Undo,
                &[Token::UnitVariant {
                    name: "FailureHandlingKind",
                    variant: "undo",
                }],
            );
        }
    }

    mod did_change_configuration_capabilities {
        use super::*;

        #[test]
        fn serde() {
            let object = DidChangeConfigurationCapabilities {
                dynamic_registration: bool::default(),
            };

            assert_tokens(
                &object,
                &[
                    Token::Struct {
                        name: "DidChangeConfigurationCapabilities",
                        len: 1,
                    },
                    Token::String("dynamicRegistration"),
                    Token::Bool(bool::default()),
                    Token::StructEnd,
                ],
            );
        }

        #[test]
        fn default() {
            assert_eq!(
                DidChangeConfigurationCapabilities::default(),
                DidChangeConfigurationCapabilities {
                    dynamic_registration: false,
                }
            );
        }

        #[test]
        fn deserialize_missing() {
            assert_de_tokens(
                &DidChangeConfigurationCapabilities::default(),
                &[
                    Token::Struct {
                        name: "DidChangeConfigurationCapabilities",
                        len: 0,
                    },
                    Token::StructEnd,
                ],
            );
        }
    }

    mod did_change_watched_files_capabilities {
        use super::*;

        #[test]
        fn serde() {
            let object = DidChangeWatchedFilesCapabilities {
                dynamic_registration: bool::default(),
            };

            assert_tokens(
                &object,
                &[
                    Token::Struct {
                        name: "DidChangeWatchedFilesCapabilities",
                        len: 1,
                    },
                    Token::String("dynamicRegistration"),
                    Token::Bool(bool::default()),
                    Token::StructEnd,
                ],
            );
        }

        #[test]
        fn default() {
            assert_eq!(
                DidChangeWatchedFilesCapabilities::default(),
                DidChangeWatchedFilesCapabilities {
                    dynamic_registration: false,
                }
            );
        }

        #[test]
        fn deserialize_missing() {
            assert_de_tokens(
                &DidChangeWatchedFilesCapabilities::default(),
                &[
                    Token::Struct {
                        name: "DidChangeWatchedFilesCapabilities",
                        len: 0,
                    },
                    Token::StructEnd,
                ],
            );
        }
    }

    mod symbol_kind_capabilities {
        use super::*;

        #[test]
        fn serde() {
            let object = SymbolKindCapabilities {
                value_set: Elective::Present(vec![SymbolKind::Known(Symbol::File)]),
            };

            assert_tokens(
                &object,
                &[
                    Token::Struct {
                        name: "SymbolKindCapabilities",
                        len: 1,
                    },
                    Token::String("valueSet"),
                    Token::Seq { len: Some(1) },
                    Token::U8(1),
                    Token::SeqEnd,
                    Token::StructEnd,
                ],
            );
        }

        #[test]
        fn default() {
            assert_eq!(
                SymbolKindCapabilities::default(),
                SymbolKindCapabilities {
                    value_set: Elective::Absent,
                }
            );
        }
    }

    mod symbol_capabilities {
        use super::*;

        #[test]
        fn serde() {
            let object = SymbolCapabilities {
                dynamic_registration: true,
                symbol_kind: SymbolKindCapabilities {
                    value_set: Elective::Present(vec![SymbolKind::Known(Symbol::File)]),
                },
            };

            assert_tokens(
                &object,
                &[
                    Token::Struct {
                        name: "SymbolCapabilities",
                        len: 2,
                    },
                    Token::String("dynamicRegistration"),
                    Token::Bool(true),
                    Token::String("symbolKind"),
                    Token::Struct {
                        name: "SymbolKindCapabilities",
                        len: 1,
                    },
                    Token::String("valueSet"),
                    Token::Seq { len: Some(1) },
                    Token::U8(1),
                    Token::SeqEnd,
                    Token::StructEnd,
                    Token::StructEnd,
                ],
            );
        }

        #[test]
        fn default() {
            assert_eq!(
                SymbolCapabilities::default(),
                SymbolCapabilities {
                    dynamic_registration: false,
                    symbol_kind: SymbolKindCapabilities::default(),
                }
            );
        }
    }

    mod execute_command_capabilities {
        use super::*;

        #[test]
        fn serde() {
            let object = ExecuteCommandCapabilities {
                dynamic_registration: true,
            };

            assert_tokens(
                &object,
                &[
                    Token::Struct {
                        name: "ExecuteCommandCapabilities",
                        len: 1,
                    },
                    Token::String("dynamicRegistration"),
                    Token::Bool(true),
                    Token::StructEnd,
                ],
            );
        }

        #[test]
        fn default() {
            assert_eq!(
                ExecuteCommandCapabilities::default(),
                ExecuteCommandCapabilities {
                    dynamic_registration: false,
                }
            );
        }
    }
}
