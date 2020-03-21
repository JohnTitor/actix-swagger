use crate::printer::Printable;
use inflections::Inflect;
use quote::{format_ident, quote};
use regex::Regex;

/// Create PascalName from string
pub fn to_struct_name(string: String) -> String {
    let re_name = Regex::new(r"[^\w_\-\d]+").expect("re_name invalid regex");

    re_name.replace_all(string.to_pascal_case().as_ref(), "").to_string()
}

/// Object describing main api structure and useful impls
#[derive(Default)]
pub struct ApiStruct {
    pub(crate) api_name: String,
    pub(crate) terms_of_service: Option<String>,
    pub(crate) description: Option<String>,
}

impl ApiStruct {
    pub fn new(api_name: String) -> Self {
        Self {
            api_name,
            terms_of_service: None,
            description: None,
        }
    }
}

impl From<openapiv3::Info> for ApiStruct {
    fn from(info: openapiv3::Info) -> Self {
        Self {
            api_name: to_struct_name(info.title),
            description: info.description,
            terms_of_service: info.terms_of_service,
        }
    }
}

impl Printable for ApiStruct {
    fn print(&self) -> proc_macro2::TokenStream {
        let api_name = format_ident!("{}", to_struct_name(self.api_name.to_owned()));
        let terms = self
            .terms_of_service
            .to_owned()
            .map_or(String::default(), |terms| format!("@see {}", terms));
        let description = self.description.to_owned().unwrap_or_default();

        let doc_comment = format!("{}\n{}", description, terms);
        let doc = doc_comment.trim();

        quote! {
            #[doc = #doc]
            pub struct #api_name {
                api: actix_swagger::Api,
            }

            impl #api_name {
                pub fn new() -> Self {
                    Self {
                        api: actix_swagger::Api::new()
                    }
                }
            }

            impl Default for #api_name {
                fn default() -> Self {
                    let api = Self::new();
                    api
                }
            }

            impl actix_web::dev::HttpServiceFactory for #api_name {
                fn register(mut self, config: &mut actix_web::dev::AppService) {
                    self.api.register(config);
                }
            }
        }
    }
}
