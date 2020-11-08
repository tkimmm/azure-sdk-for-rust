#![doc = "generated by AutoRust 0.1.0"]
#![allow(unused_mut)]
#![allow(unused_variables)]
#![allow(unused_imports)]
use crate::models::*;
use reqwest::StatusCode;
use snafu::{ResultExt, Snafu};
pub async fn list_operations(
    operation_config: &crate::OperationConfig,
) -> std::result::Result<OperationListResult, list_operations::Error> {
    let client = &operation_config.client;
    let uri_str = &format!("{}/providers/Microsoft.Features/operations", &operation_config.base_path,);
    let mut req_builder = client.get(uri_str);
    if let Some(token_credential) = &operation_config.token_credential {
        let token_response = token_credential
            .get_token(&operation_config.token_credential_resource)
            .await
            .context(list_operations::GetTokenError)?;
        req_builder = req_builder.bearer_auth(token_response.token.secret());
    }
    req_builder = req_builder.query(&[("api-version", &operation_config.api_version)]);
    let req = req_builder.build().context(list_operations::BuildRequestError)?;
    let rsp = client.execute(req).await.context(list_operations::ExecuteRequestError)?;
    match rsp.status() {
        StatusCode::OK => {
            let body: bytes::Bytes = rsp.bytes().await.context(list_operations::ResponseBytesError)?;
            let rsp_value: OperationListResult = serde_json::from_slice(&body).context(list_operations::DeserializeError { body })?;
            Ok(rsp_value)
        }
        status_code => {
            let body: bytes::Bytes = rsp.bytes().await.context(list_operations::ResponseBytesError)?;
            list_operations::UnexpectedResponse { status_code, body: body }.fail()
        }
    }
}
pub mod list_operations {
    use crate::{models, models::*};
    use reqwest::StatusCode;
    use snafu::Snafu;
    #[derive(Debug, Snafu)]
    #[snafu(visibility(pub(crate)))]
    pub enum Error {
        UnexpectedResponse { status_code: StatusCode, body: bytes::Bytes },
        BuildRequestError { source: reqwest::Error },
        ExecuteRequestError { source: reqwest::Error },
        ResponseBytesError { source: reqwest::Error },
        DeserializeError { source: serde_json::Error, body: bytes::Bytes },
        GetTokenError { source: azure_core::errors::AzureError },
    }
}
pub mod features {
    use crate::models::*;
    use reqwest::StatusCode;
    use snafu::{ResultExt, Snafu};
    pub async fn list_all(
        operation_config: &crate::OperationConfig,
        subscription_id: &str,
    ) -> std::result::Result<FeatureOperationsListResult, list_all::Error> {
        let client = &operation_config.client;
        let uri_str = &format!(
            "{}/subscriptions/{}/providers/Microsoft.Features/features",
            &operation_config.base_path, subscription_id
        );
        let mut req_builder = client.get(uri_str);
        if let Some(token_credential) = &operation_config.token_credential {
            let token_response = token_credential
                .get_token(&operation_config.token_credential_resource)
                .await
                .context(list_all::GetTokenError)?;
            req_builder = req_builder.bearer_auth(token_response.token.secret());
        }
        req_builder = req_builder.query(&[("api-version", &operation_config.api_version)]);
        let req = req_builder.build().context(list_all::BuildRequestError)?;
        let rsp = client.execute(req).await.context(list_all::ExecuteRequestError)?;
        match rsp.status() {
            StatusCode::OK => {
                let body: bytes::Bytes = rsp.bytes().await.context(list_all::ResponseBytesError)?;
                let rsp_value: FeatureOperationsListResult = serde_json::from_slice(&body).context(list_all::DeserializeError { body })?;
                Ok(rsp_value)
            }
            status_code => {
                let body: bytes::Bytes = rsp.bytes().await.context(list_all::ResponseBytesError)?;
                list_all::UnexpectedResponse { status_code, body: body }.fail()
            }
        }
    }
    pub mod list_all {
        use crate::{models, models::*};
        use reqwest::StatusCode;
        use snafu::Snafu;
        #[derive(Debug, Snafu)]
        #[snafu(visibility(pub(crate)))]
        pub enum Error {
            UnexpectedResponse { status_code: StatusCode, body: bytes::Bytes },
            BuildRequestError { source: reqwest::Error },
            ExecuteRequestError { source: reqwest::Error },
            ResponseBytesError { source: reqwest::Error },
            DeserializeError { source: serde_json::Error, body: bytes::Bytes },
            GetTokenError { source: azure_core::errors::AzureError },
        }
    }
    pub async fn list(
        operation_config: &crate::OperationConfig,
        resource_provider_namespace: &str,
        subscription_id: &str,
    ) -> std::result::Result<FeatureOperationsListResult, list::Error> {
        let client = &operation_config.client;
        let uri_str = &format!(
            "{}/subscriptions/{}/providers/Microsoft.Features/providers/{}/features",
            &operation_config.base_path, subscription_id, resource_provider_namespace
        );
        let mut req_builder = client.get(uri_str);
        if let Some(token_credential) = &operation_config.token_credential {
            let token_response = token_credential
                .get_token(&operation_config.token_credential_resource)
                .await
                .context(list::GetTokenError)?;
            req_builder = req_builder.bearer_auth(token_response.token.secret());
        }
        req_builder = req_builder.query(&[("api-version", &operation_config.api_version)]);
        let req = req_builder.build().context(list::BuildRequestError)?;
        let rsp = client.execute(req).await.context(list::ExecuteRequestError)?;
        match rsp.status() {
            StatusCode::OK => {
                let body: bytes::Bytes = rsp.bytes().await.context(list::ResponseBytesError)?;
                let rsp_value: FeatureOperationsListResult = serde_json::from_slice(&body).context(list::DeserializeError { body })?;
                Ok(rsp_value)
            }
            status_code => {
                let body: bytes::Bytes = rsp.bytes().await.context(list::ResponseBytesError)?;
                list::UnexpectedResponse { status_code, body: body }.fail()
            }
        }
    }
    pub mod list {
        use crate::{models, models::*};
        use reqwest::StatusCode;
        use snafu::Snafu;
        #[derive(Debug, Snafu)]
        #[snafu(visibility(pub(crate)))]
        pub enum Error {
            UnexpectedResponse { status_code: StatusCode, body: bytes::Bytes },
            BuildRequestError { source: reqwest::Error },
            ExecuteRequestError { source: reqwest::Error },
            ResponseBytesError { source: reqwest::Error },
            DeserializeError { source: serde_json::Error, body: bytes::Bytes },
            GetTokenError { source: azure_core::errors::AzureError },
        }
    }
    pub async fn get(
        operation_config: &crate::OperationConfig,
        resource_provider_namespace: &str,
        feature_name: &str,
        subscription_id: &str,
    ) -> std::result::Result<FeatureResult, get::Error> {
        let client = &operation_config.client;
        let uri_str = &format!(
            "{}/subscriptions/{}/providers/Microsoft.Features/providers/{}/features/{}",
            &operation_config.base_path, subscription_id, resource_provider_namespace, feature_name
        );
        let mut req_builder = client.get(uri_str);
        if let Some(token_credential) = &operation_config.token_credential {
            let token_response = token_credential
                .get_token(&operation_config.token_credential_resource)
                .await
                .context(get::GetTokenError)?;
            req_builder = req_builder.bearer_auth(token_response.token.secret());
        }
        req_builder = req_builder.query(&[("api-version", &operation_config.api_version)]);
        let req = req_builder.build().context(get::BuildRequestError)?;
        let rsp = client.execute(req).await.context(get::ExecuteRequestError)?;
        match rsp.status() {
            StatusCode::OK => {
                let body: bytes::Bytes = rsp.bytes().await.context(get::ResponseBytesError)?;
                let rsp_value: FeatureResult = serde_json::from_slice(&body).context(get::DeserializeError { body })?;
                Ok(rsp_value)
            }
            status_code => {
                let body: bytes::Bytes = rsp.bytes().await.context(get::ResponseBytesError)?;
                get::UnexpectedResponse { status_code, body: body }.fail()
            }
        }
    }
    pub mod get {
        use crate::{models, models::*};
        use reqwest::StatusCode;
        use snafu::Snafu;
        #[derive(Debug, Snafu)]
        #[snafu(visibility(pub(crate)))]
        pub enum Error {
            UnexpectedResponse { status_code: StatusCode, body: bytes::Bytes },
            BuildRequestError { source: reqwest::Error },
            ExecuteRequestError { source: reqwest::Error },
            ResponseBytesError { source: reqwest::Error },
            DeserializeError { source: serde_json::Error, body: bytes::Bytes },
            GetTokenError { source: azure_core::errors::AzureError },
        }
    }
    pub async fn register(
        operation_config: &crate::OperationConfig,
        resource_provider_namespace: &str,
        feature_name: &str,
        subscription_id: &str,
    ) -> std::result::Result<FeatureResult, register::Error> {
        let client = &operation_config.client;
        let uri_str = &format!(
            "{}/subscriptions/{}/providers/Microsoft.Features/providers/{}/features/{}/register",
            &operation_config.base_path, subscription_id, resource_provider_namespace, feature_name
        );
        let mut req_builder = client.post(uri_str);
        if let Some(token_credential) = &operation_config.token_credential {
            let token_response = token_credential
                .get_token(&operation_config.token_credential_resource)
                .await
                .context(register::GetTokenError)?;
            req_builder = req_builder.bearer_auth(token_response.token.secret());
        }
        req_builder = req_builder.query(&[("api-version", &operation_config.api_version)]);
        req_builder = req_builder.header(reqwest::header::CONTENT_LENGTH, 0);
        let req = req_builder.build().context(register::BuildRequestError)?;
        let rsp = client.execute(req).await.context(register::ExecuteRequestError)?;
        match rsp.status() {
            StatusCode::OK => {
                let body: bytes::Bytes = rsp.bytes().await.context(register::ResponseBytesError)?;
                let rsp_value: FeatureResult = serde_json::from_slice(&body).context(register::DeserializeError { body })?;
                Ok(rsp_value)
            }
            status_code => {
                let body: bytes::Bytes = rsp.bytes().await.context(register::ResponseBytesError)?;
                register::UnexpectedResponse { status_code, body: body }.fail()
            }
        }
    }
    pub mod register {
        use crate::{models, models::*};
        use reqwest::StatusCode;
        use snafu::Snafu;
        #[derive(Debug, Snafu)]
        #[snafu(visibility(pub(crate)))]
        pub enum Error {
            UnexpectedResponse { status_code: StatusCode, body: bytes::Bytes },
            BuildRequestError { source: reqwest::Error },
            ExecuteRequestError { source: reqwest::Error },
            ResponseBytesError { source: reqwest::Error },
            DeserializeError { source: serde_json::Error, body: bytes::Bytes },
            GetTokenError { source: azure_core::errors::AzureError },
        }
    }
    pub async fn unregister(
        operation_config: &crate::OperationConfig,
        resource_provider_namespace: &str,
        feature_name: &str,
        subscription_id: &str,
    ) -> std::result::Result<FeatureResult, unregister::Error> {
        let client = &operation_config.client;
        let uri_str = &format!(
            "{}/subscriptions/{}/providers/Microsoft.Features/providers/{}/features/{}/unregister",
            &operation_config.base_path, subscription_id, resource_provider_namespace, feature_name
        );
        let mut req_builder = client.post(uri_str);
        if let Some(token_credential) = &operation_config.token_credential {
            let token_response = token_credential
                .get_token(&operation_config.token_credential_resource)
                .await
                .context(unregister::GetTokenError)?;
            req_builder = req_builder.bearer_auth(token_response.token.secret());
        }
        req_builder = req_builder.query(&[("api-version", &operation_config.api_version)]);
        req_builder = req_builder.header(reqwest::header::CONTENT_LENGTH, 0);
        let req = req_builder.build().context(unregister::BuildRequestError)?;
        let rsp = client.execute(req).await.context(unregister::ExecuteRequestError)?;
        match rsp.status() {
            StatusCode::OK => {
                let body: bytes::Bytes = rsp.bytes().await.context(unregister::ResponseBytesError)?;
                let rsp_value: FeatureResult = serde_json::from_slice(&body).context(unregister::DeserializeError { body })?;
                Ok(rsp_value)
            }
            status_code => {
                let body: bytes::Bytes = rsp.bytes().await.context(unregister::ResponseBytesError)?;
                unregister::UnexpectedResponse { status_code, body: body }.fail()
            }
        }
    }
    pub mod unregister {
        use crate::{models, models::*};
        use reqwest::StatusCode;
        use snafu::Snafu;
        #[derive(Debug, Snafu)]
        #[snafu(visibility(pub(crate)))]
        pub enum Error {
            UnexpectedResponse { status_code: StatusCode, body: bytes::Bytes },
            BuildRequestError { source: reqwest::Error },
            ExecuteRequestError { source: reqwest::Error },
            ResponseBytesError { source: reqwest::Error },
            DeserializeError { source: serde_json::Error, body: bytes::Bytes },
            GetTokenError { source: azure_core::errors::AzureError },
        }
    }
}
