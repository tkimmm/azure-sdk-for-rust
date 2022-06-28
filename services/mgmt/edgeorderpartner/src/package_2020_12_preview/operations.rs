#![doc = "generated by AutoRust"]
#![allow(unused_mut)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(clippy::redundant_clone)]
use super::models;
#[derive(Clone)]
pub struct Client {
    endpoint: String,
    credential: std::sync::Arc<dyn azure_core::auth::TokenCredential>,
    scopes: Vec<String>,
    pipeline: azure_core::Pipeline,
}
#[derive(Clone)]
pub struct ClientBuilder {
    credential: std::sync::Arc<dyn azure_core::auth::TokenCredential>,
    endpoint: Option<String>,
    scopes: Option<Vec<String>>,
}
pub const DEFAULT_ENDPOINT: &str = azure_core::resource_manager_endpoint::AZURE_PUBLIC_CLOUD;
impl ClientBuilder {
    pub fn new(credential: std::sync::Arc<dyn azure_core::auth::TokenCredential>) -> Self {
        Self {
            credential,
            endpoint: None,
            scopes: None,
        }
    }
    pub fn endpoint(mut self, endpoint: impl Into<String>) -> Self {
        self.endpoint = Some(endpoint.into());
        self
    }
    pub fn scopes(mut self, scopes: &[&str]) -> Self {
        self.scopes = Some(scopes.iter().map(|scope| (*scope).to_owned()).collect());
        self
    }
    pub fn build(self) -> Client {
        let endpoint = self.endpoint.unwrap_or_else(|| DEFAULT_ENDPOINT.to_owned());
        let scopes = self.scopes.unwrap_or_else(|| vec![format!("{}/", endpoint)]);
        Client::new(endpoint, self.credential, scopes)
    }
}
impl Client {
    pub(crate) fn endpoint(&self) -> &str {
        self.endpoint.as_str()
    }
    pub(crate) fn token_credential(&self) -> &dyn azure_core::auth::TokenCredential {
        self.credential.as_ref()
    }
    pub(crate) fn scopes(&self) -> Vec<&str> {
        self.scopes.iter().map(String::as_str).collect()
    }
    pub(crate) async fn send(&self, request: &mut azure_core::Request) -> azure_core::Result<azure_core::Response> {
        let mut context = azure_core::Context::default();
        self.pipeline.send(&mut context, request).await
    }
    pub fn new(
        endpoint: impl Into<String>,
        credential: std::sync::Arc<dyn azure_core::auth::TokenCredential>,
        scopes: Vec<String>,
    ) -> Self {
        let endpoint = endpoint.into();
        let pipeline = azure_core::Pipeline::new(
            option_env!("CARGO_PKG_NAME"),
            option_env!("CARGO_PKG_VERSION"),
            azure_core::ClientOptions::default(),
            Vec::new(),
            Vec::new(),
        );
        Self {
            endpoint,
            credential,
            scopes,
            pipeline,
        }
    }
}
impl Client {
    pub fn list_operations_partner(&self) -> list_operations_partner::Builder {
        list_operations_partner::Builder { client: self.clone() }
    }
    #[doc = "API for updating inventory metadata and inventory configuration"]
    pub fn manage_inventory_metadata(
        &self,
        family_identifier: impl Into<String>,
        subscription_id: impl Into<String>,
        location: impl Into<String>,
        serial_number: impl Into<String>,
        manage_inventory_metadata_request: impl Into<models::ManageInventoryMetadataRequest>,
    ) -> manage_inventory_metadata::Builder {
        manage_inventory_metadata::Builder {
            client: self.clone(),
            family_identifier: family_identifier.into(),
            subscription_id: subscription_id.into(),
            location: location.into(),
            serial_number: serial_number.into(),
            manage_inventory_metadata_request: manage_inventory_metadata_request.into(),
        }
    }
    #[doc = "API for linking management resource with inventory"]
    pub fn manage_link(
        &self,
        family_identifier: impl Into<String>,
        subscription_id: impl Into<String>,
        location: impl Into<String>,
        serial_number: impl Into<String>,
        manage_link_request: impl Into<models::ManageLinkRequest>,
    ) -> manage_link::Builder {
        manage_link::Builder {
            client: self.clone(),
            family_identifier: family_identifier.into(),
            subscription_id: subscription_id.into(),
            location: location.into(),
            serial_number: serial_number.into(),
            manage_link_request: manage_link_request.into(),
        }
    }
    #[doc = "API for Search inventories"]
    pub fn search_inventories(
        &self,
        subscription_id: impl Into<String>,
        search_inventories_request: impl Into<models::SearchInventoriesRequest>,
    ) -> search_inventories::Builder {
        search_inventories::Builder {
            client: self.clone(),
            subscription_id: subscription_id.into(),
            search_inventories_request: search_inventories_request.into(),
        }
    }
}
pub mod list_operations_partner {
    use super::models;
    type Response = models::OperationListResult;
    #[derive(Clone)]
    pub struct Builder {
        pub(crate) client: super::Client,
    }
    impl Builder {
        pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
            let make_request = move |continuation: Option<azure_core::prelude::Continuation>| {
                let this = self.clone();
                async move {
                    let mut url = azure_core::Url::parse(&format!(
                        "{}/providers/Microsoft.EdgeOrderPartner/operations",
                        this.client.endpoint(),
                    ))?;
                    let rsp = match continuation {
                        Some(azure_core::prelude::Continuation::String(value)) => {
                            url.set_path("");
                            url = url.join(&value)?;
                            let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                            let credential = this.client.token_credential();
                            let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                            req.insert_header(
                                azure_core::headers::AUTHORIZATION,
                                format!("Bearer {}", token_response.token.secret()),
                            );
                            let has_api_version_already =
                                req.url_mut().query_pairs().any(|(k, _)| k == azure_core::query_param::API_VERSION);
                            if !has_api_version_already {
                                req.url_mut()
                                    .query_pairs_mut()
                                    .append_pair(azure_core::query_param::API_VERSION, "2020-12-01-preview");
                            }
                            let req_body = azure_core::EMPTY_BODY;
                            req.set_body(req_body);
                            this.client.send(&mut req).await?
                        }
                        Some(azure_core::prelude::Continuation::Range(_)) => {
                            panic!("unexpected continuation type");
                        }
                        None => {
                            let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                            let credential = this.client.token_credential();
                            let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                            req.insert_header(
                                azure_core::headers::AUTHORIZATION,
                                format!("Bearer {}", token_response.token.secret()),
                            );
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair(azure_core::query_param::API_VERSION, "2020-12-01-preview");
                            let req_body = azure_core::EMPTY_BODY;
                            req.set_body(req_body);
                            this.client.send(&mut req).await?
                        }
                    };
                    let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                    match rsp_status {
                        azure_core::StatusCode::Ok => {
                            let rsp_body = azure_core::collect_pinned_stream(rsp_stream).await?;
                            let rsp_value: models::OperationListResult = serde_json::from_slice(&rsp_body)?;
                            Ok(rsp_value)
                        }
                        status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                            status: status_code as u16,
                            error_code: None,
                        })),
                    }
                }
            };
            azure_core::Pageable::new(make_request)
        }
    }
}
pub mod manage_inventory_metadata {
    use super::models;
    #[derive(Debug)]
    pub enum Response {
        Ok200,
        Accepted202,
        NoContent204,
    }
    #[derive(Clone)]
    pub struct Builder {
        pub(crate) client: super::Client,
        pub(crate) family_identifier: String,
        pub(crate) subscription_id: String,
        pub(crate) location: String,
        pub(crate) serial_number: String,
        pub(crate) manage_inventory_metadata_request: models::ManageInventoryMetadataRequest,
    }
    impl Builder {
        #[doc = "only the first response will be fetched as long running operations are not supported yet"]
        pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
            Box::pin({
                let this = self.clone();
                async move {
                    let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/providers/Microsoft.EdgeOrderPartner/locations/{}/productFamilies/{}/inventories/{}/manageInventoryMetadata" , this . client . endpoint () , & this . subscription_id , & this . location , & this . family_identifier , & this . serial_number)) ? ;
                    let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                    let credential = this.client.token_credential();
                    let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                    req.insert_header(
                        azure_core::headers::AUTHORIZATION,
                        format!("Bearer {}", token_response.token.secret()),
                    );
                    req.url_mut()
                        .query_pairs_mut()
                        .append_pair(azure_core::query_param::API_VERSION, "2020-12-01-preview");
                    req.insert_header("content-type", "application/json");
                    let req_body = azure_core::to_json(&this.manage_inventory_metadata_request)?;
                    req.set_body(req_body);
                    let rsp = this.client.send(&mut req).await?;
                    let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                    match rsp_status {
                        azure_core::StatusCode::Ok => Ok(Response::Ok200),
                        azure_core::StatusCode::Accepted => Ok(Response::Accepted202),
                        azure_core::StatusCode::NoContent => Ok(Response::NoContent204),
                        status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                            status: status_code as u16,
                            error_code: None,
                        })),
                    }
                }
            })
        }
    }
}
pub mod manage_link {
    use super::models;
    #[derive(Debug)]
    pub enum Response {
        Ok200,
        NoContent204,
    }
    #[derive(Clone)]
    pub struct Builder {
        pub(crate) client: super::Client,
        pub(crate) family_identifier: String,
        pub(crate) subscription_id: String,
        pub(crate) location: String,
        pub(crate) serial_number: String,
        pub(crate) manage_link_request: models::ManageLinkRequest,
    }
    impl Builder {
        pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
            Box::pin({
                let this = self.clone();
                async move {
                    let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/providers/Microsoft.EdgeOrderPartner/locations/{}/productFamilies/{}/inventories/{}/manageLink" , this . client . endpoint () , & this . subscription_id , & this . location , & this . family_identifier , & this . serial_number)) ? ;
                    let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                    let credential = this.client.token_credential();
                    let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                    req.insert_header(
                        azure_core::headers::AUTHORIZATION,
                        format!("Bearer {}", token_response.token.secret()),
                    );
                    req.url_mut()
                        .query_pairs_mut()
                        .append_pair(azure_core::query_param::API_VERSION, "2020-12-01-preview");
                    req.insert_header("content-type", "application/json");
                    let req_body = azure_core::to_json(&this.manage_link_request)?;
                    req.set_body(req_body);
                    let rsp = this.client.send(&mut req).await?;
                    let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                    match rsp_status {
                        azure_core::StatusCode::Ok => Ok(Response::Ok200),
                        azure_core::StatusCode::NoContent => Ok(Response::NoContent204),
                        status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                            status: status_code as u16,
                            error_code: None,
                        })),
                    }
                }
            })
        }
    }
}
pub mod search_inventories {
    use super::models;
    type Response = models::PartnerInventoryList;
    #[derive(Clone)]
    pub struct Builder {
        pub(crate) client: super::Client,
        pub(crate) subscription_id: String,
        pub(crate) search_inventories_request: models::SearchInventoriesRequest,
    }
    impl Builder {
        pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
            let make_request = move |continuation: Option<azure_core::prelude::Continuation>| {
                let this = self.clone();
                async move {
                    let mut url = azure_core::Url::parse(&format!(
                        "{}/subscriptions/{}/providers/Microsoft.EdgeOrderPartner/searchInventories",
                        this.client.endpoint(),
                        &this.subscription_id
                    ))?;
                    let rsp = match continuation {
                        Some(azure_core::prelude::Continuation::String(value)) => {
                            url.set_path("");
                            url = url.join(&value)?;
                            let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                            let credential = this.client.token_credential();
                            let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                            req.insert_header(
                                azure_core::headers::AUTHORIZATION,
                                format!("Bearer {}", token_response.token.secret()),
                            );
                            let has_api_version_already =
                                req.url_mut().query_pairs().any(|(k, _)| k == azure_core::query_param::API_VERSION);
                            if !has_api_version_already {
                                req.url_mut()
                                    .query_pairs_mut()
                                    .append_pair(azure_core::query_param::API_VERSION, "2020-12-01-preview");
                            }
                            let req_body = azure_core::EMPTY_BODY;
                            req.set_body(req_body);
                            this.client.send(&mut req).await?
                        }
                        Some(azure_core::prelude::Continuation::Range(_)) => {
                            panic!("unexpected continuation type");
                        }
                        None => {
                            let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                            let credential = this.client.token_credential();
                            let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                            req.insert_header(
                                azure_core::headers::AUTHORIZATION,
                                format!("Bearer {}", token_response.token.secret()),
                            );
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair(azure_core::query_param::API_VERSION, "2020-12-01-preview");
                            req.insert_header("content-type", "application/json");
                            let req_body = azure_core::to_json(&this.search_inventories_request)?;
                            req.set_body(req_body);
                            this.client.send(&mut req).await?
                        }
                    };
                    let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                    match rsp_status {
                        azure_core::StatusCode::Ok => {
                            let rsp_body = azure_core::collect_pinned_stream(rsp_stream).await?;
                            let rsp_value: models::PartnerInventoryList = serde_json::from_slice(&rsp_body)?;
                            Ok(rsp_value)
                        }
                        status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                            status: status_code as u16,
                            error_code: None,
                        })),
                    }
                }
            };
            azure_core::Pageable::new(make_request)
        }
    }
}
