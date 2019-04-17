extern crate proc_macro;

use proc_macro::{TokenTree, TokenStream};
use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::quote;
use syn::{Ident, Fields, parse_macro_input, Item};

enum AttributeParserState {
    Option,
    DynamicRegistrationValue,
    LinkSupportValue,
    MarkupKindListValue,
    TriggersValue,
    ResolveProviderValue,
}

impl AttributeParserState {
    fn is_searching_for_value(&self) -> bool {
        match self {
            AttributeParserState::Option => false,
            _ => true,
        }
    }
}

#[proc_macro_attribute]
pub fn lsp_object(attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut allow_missing_attr = quote!{};
    let mut dynamic_registration = None;
    let mut link_support = None;
    let mut markup_kind_list = None;
    let mut triggers = None;
    let mut resolve_provider = None;
    let mut has_document_selector = false;
    let mut has_static_registration = false;
    let mut state = AttributeParserState::Option;

    for token in attr {
        match state {
            AttributeParserState::Option => {
                match token {
                    TokenTree::Ident(ident) => {
                        match ident.to_string().as_str() {
                            "allow_missing" => {
                                allow_missing_attr = quote!{
                                    #[serde(default)]
                                };
                            }
                            "document_selector" => {
                                has_document_selector = true;
                            }
                            "static_registration" => {
                                has_static_registration = true;
                            }
                            "dynamic_registration" => {
                                state = AttributeParserState::DynamicRegistrationValue;
                            }
                            "link_support" => {
                                state = AttributeParserState::LinkSupportValue;
                            }
                            "markup_kind_list" => {
                                state = AttributeParserState::MarkupKindListValue;
                            }
                            "triggers" => {
                                state = AttributeParserState::TriggersValue;
                            }
                            "resolve_provider" => {
                                state = AttributeParserState::ResolveProviderValue;
                            }
                            option => {
                                panic!("Unsupported attribute option: {}", option);
                            }
                        }
                    }
                    _ => (),
                }
            }
            AttributeParserState::DynamicRegistrationValue => {
                match token {
                    TokenTree::Literal(literal) => {
                        dynamic_registration = Some(literal);
                        state = AttributeParserState::Option;
                    }
                    _ => (),
                }
            }
            AttributeParserState::LinkSupportValue => {
                match token {
                    TokenTree::Literal(literal) => {
                        link_support = Some(literal);
                        state = AttributeParserState::Option;
                    }
                    _ => (),
                }
            }
            AttributeParserState::MarkupKindListValue => {
                match token {
                    TokenTree::Literal(literal) => {
                        markup_kind_list = Some(literal);
                        state = AttributeParserState::Option;
                    }
                    _ => (),
                }
            }
            AttributeParserState::TriggersValue => {
                match token {
                    TokenTree::Literal(literal) => {
                        triggers = Some(literal);
                        state = AttributeParserState::Option;
                    }
                    _ => (),
                }
            }
            AttributeParserState::ResolveProviderValue => {
                match token {
                    TokenTree::Literal(literal) => {
                        resolve_provider = Some(literal);
                        state = AttributeParserState::Option;
                    }
                    _ => (),
                }
            }
        }
    }

    if state.is_searching_for_value() {
        panic!("Missing a value for an option.");
    }

    let input = if let Item::Struct(item_struct) = parse_macro_input!(item as Item) {
        item_struct
    } else {
        panic!("Error");
    };
    
    let name = input.ident;
    let vis = input.vis;
    let generics = input.generics;
    let attrs = input.attrs;
    let old_fields = if let Fields::Named(fields_named) = input.fields {
        fields_named.named
    } else {
        panic!("Error");
    };

    let mut fields: Vec<TokenStream2> = Vec::new();

    if has_document_selector {
        fields.push(quote!{
            /// Identifies the scope of the registration.
            ///
            /// If `Option::None`, `DocumentSelector` provided by client will be used.
            document_selector: Option<char>
        });
    }

    if has_static_registration {
        fields.push(quote!{
            /// The id used to register the request.
            id: Elective<String>
        });
    }

    if let Some(doc_var) = dynamic_registration {
        let mut d = doc_var.to_string();
        d.retain(|c| c != '"');
        let doc = format!("Supports dynamic registration of the {}.", d);

        fields.push(quote!{
            #[doc = #doc]
            dynamic_registration: bool
        });
    }

    if let Some(doc_var) = link_support {
        let mut d = doc_var.to_string();
        d.retain(|c| c != '"');
        let doc = format!("Supports additional metadata in the form of {} links.", d);

        fields.push(quote!{
            #[doc = #doc]
            link_support: bool
        });
    }

    if let Some(property) = markup_kind_list {
        let mut p = property.to_string();
        p.retain(|c| c != '"');
        let doc = format!("The supported `MarkupKind`s for the `{}` property.\n\nThe order describes the preferred format.", p);
        let property_name = format!("{}_format", p);
        let name = Ident::new(&property_name, Span::call_site());

        fields.push(quote!{
            #[doc = #doc]
            #name: Vec<MarkupKind>
        });
    }

    if let Some(doc_var) = triggers {
        let mut d = doc_var.to_string();
        d.retain(|c| c != '"');
        let doc = format!("Characters that trigger {} automatically.", d);

        fields.push(quote!{
            #[doc = #doc]
            trigger_characters: Vec<String>
        });
    }

    if let Some(doc_var) = resolve_provider {
        let mut d = doc_var.to_string();
        d.retain(|c| c != '"');
        let doc = format!("Provides support to resolve additional information for a {} item.", d);

        fields.push(quote!{
            #[doc = #doc]
            resolve_provider: bool
        });
    }

    for field in old_fields {
        let mut is_elective = false;
        match &field.ty {
            syn::Type::Path(p) => {
                if let Some(segment) = p.path.segments.first() {
                    if segment.value().ident.to_string() == String::from("Elective") {
                        is_elective = true;
                    }
                }
            }
            _ => (),
        };
        let elective_attr = if is_elective {
            quote!{
                #[serde(default, skip_serializing_if = "Elective::is_absent")]
            }
        } else {
            quote!{}
        };
        let field_type = field.ty;
        let field_name = field.ident;
        let field_attrs = field.attrs;
        let field_vis = field.vis;
        fields.push(quote!{
            #elective_attr
            #(#field_attrs)*
            #field_vis #field_name: #field_type
        });
    }

    let output = quote!{
        #[derive(Debug, Default, Deserialize, Serialize)]
        #[serde(rename_all = "camelCase")]
        #allow_missing_attr
        #(#attrs)*
        #vis struct #name #generics {
            #(#fields),*
        }
    };

    TokenStream::from(output)
}

#[proc_macro_attribute]
pub fn lsp_kind(attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut kind_attrs = quote!{
        #[derive(Debug, Deserialize, Serialize)]
        #[serde(rename_all = "camelCase")]
    };

    for token in attr {
        match token {
            TokenTree::Ident(ident) => {
                match ident.to_string().as_str() {
                    "number" => {
                        kind_attrs = quote!{
                            #[derive(Debug, Deserialize_repr, Serialize_repr)]
                            #[repr(u8)]
                        };
                    }
                    _ => {
                        panic!("Error parsing lsp_kind");
                    }
                }
            }
            _ => {
                panic!("Error parsing lsp_kind");
            }
        }
    }

    let input = if let Item::Enum(item_enum) = parse_macro_input!(item as Item) {
        item_enum
    } else {
        panic!("Error");
    };

    let output = quote!{
        #kind_attrs
        #input
    };

    TokenStream::from(output)
}
