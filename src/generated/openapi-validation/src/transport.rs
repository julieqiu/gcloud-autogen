// Copyright 2024 Google LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     https://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//
// Code generated by sidekick. DO NOT EDIT.

use crate::Result;
#[allow(unused_imports)]
use gax::error::Error;

/// Stores sensitive data such as API keys, passwords, and certificates.
/// Provides convenience while improving security.
#[derive(Clone)]
pub struct SecretManagerService {
    inner: gax::http_client::ReqwestClient,
}

impl std::fmt::Debug for SecretManagerService {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        f.debug_struct("SecretManagerService")
            .field("inner", &self.inner)
            .finish()
    }
}

impl SecretManagerService {
    pub async fn new(config: gax::http_client::ClientConfig) -> Result<Self> {
        let inner = gax::http_client::ReqwestClient::new(config, crate::DEFAULT_HOST).await?;
        Ok(Self { inner })
    }
}

impl crate::traits::SecretManagerService for SecretManagerService {
    /// Lists information about the supported locations for this service.
    async fn list_locations(
        &self,
        req: crate::model::ListLocationsRequest,
        _options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListLocationsResponse> {
        let builder = self
            .inner
            .builder(
                reqwest::Method::GET,
                format!("/v1/projects/{}/locations", req.project),
            )
            .query(&[("alt", "json")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        let builder =
            gax::query_parameter::add(builder, "filter", &req.filter).map_err(Error::other)?;
        let builder =
            gax::query_parameter::add(builder, "pageSize", &req.page_size).map_err(Error::other)?;
        let builder = gax::query_parameter::add(builder, "pageToken", &req.page_token)
            .map_err(Error::other)?;
        self.inner
            .execute(builder, None::<gax::http_client::NoBody>)
            .await
    }

    /// Gets information about a location.
    async fn get_location(
        &self,
        req: crate::model::GetLocationRequest,
        _options: gax::options::RequestOptions,
    ) -> Result<crate::model::Location> {
        let builder = self
            .inner
            .builder(
                reqwest::Method::GET,
                format!("/v1/projects/{}/locations/{}", req.project, req.location),
            )
            .query(&[("alt", "json")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        self.inner
            .execute(builder, None::<gax::http_client::NoBody>)
            .await
    }

    /// Lists Secrets.
    async fn list_secrets(
        &self,
        req: crate::model::ListSecretsRequest,
        _options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListSecretsResponse> {
        let builder = self
            .inner
            .builder(
                reqwest::Method::GET,
                format!("/v1/projects/{}/secrets", req.project),
            )
            .query(&[("alt", "json")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        let builder =
            gax::query_parameter::add(builder, "pageSize", &req.page_size).map_err(Error::other)?;
        let builder = gax::query_parameter::add(builder, "pageToken", &req.page_token)
            .map_err(Error::other)?;
        let builder =
            gax::query_parameter::add(builder, "filter", &req.filter).map_err(Error::other)?;
        self.inner
            .execute(builder, None::<gax::http_client::NoBody>)
            .await
    }

    /// Creates a new Secret containing no SecretVersions.
    async fn create_secret(
        &self,
        req: crate::model::CreateSecretRequest,
        _options: gax::options::RequestOptions,
    ) -> Result<crate::model::Secret> {
        let builder = self
            .inner
            .builder(
                reqwest::Method::POST,
                format!("/v1/projects/{}/secrets", req.project),
            )
            .query(&[("alt", "json")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        let builder =
            gax::query_parameter::add(builder, "secretId", &req.secret_id).map_err(Error::other)?;
        self.inner.execute(builder, Some(req.request_body)).await
    }

    /// Lists Secrets.
    async fn list_secrets_by_project_and_location(
        &self,
        req: crate::model::ListSecretsByProjectAndLocationRequest,
        _options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListSecretsResponse> {
        let builder = self
            .inner
            .builder(
                reqwest::Method::GET,
                format!(
                    "/v1/projects/{}/locations/{}/secrets",
                    req.project, req.location
                ),
            )
            .query(&[("alt", "json")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        let builder =
            gax::query_parameter::add(builder, "pageSize", &req.page_size).map_err(Error::other)?;
        let builder = gax::query_parameter::add(builder, "pageToken", &req.page_token)
            .map_err(Error::other)?;
        let builder =
            gax::query_parameter::add(builder, "filter", &req.filter).map_err(Error::other)?;
        self.inner
            .execute(builder, None::<gax::http_client::NoBody>)
            .await
    }

    /// Creates a new Secret containing no SecretVersions.
    async fn create_secret_by_project_and_location(
        &self,
        req: crate::model::CreateSecretByProjectAndLocationRequest,
        _options: gax::options::RequestOptions,
    ) -> Result<crate::model::Secret> {
        let builder = self
            .inner
            .builder(
                reqwest::Method::POST,
                format!(
                    "/v1/projects/{}/locations/{}/secrets",
                    req.project, req.location
                ),
            )
            .query(&[("alt", "json")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        let builder =
            gax::query_parameter::add(builder, "secretId", &req.secret_id).map_err(Error::other)?;
        self.inner.execute(builder, Some(req.request_body)).await
    }

    /// Creates a new SecretVersion containing secret data and attaches
    /// it to an existing Secret.
    async fn add_secret_version(
        &self,
        req: crate::model::AddSecretVersionRequest,
        _options: gax::options::RequestOptions,
    ) -> Result<crate::model::SecretVersion> {
        let builder = self
            .inner
            .builder(
                reqwest::Method::POST,
                format!(
                    "/v1/projects/{}/secrets/{}:addVersion",
                    req.project, req.secret
                ),
            )
            .query(&[("alt", "json")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        self.inner.execute(builder, Some(req)).await
    }

    /// Creates a new SecretVersion containing secret data and attaches
    /// it to an existing Secret.
    async fn add_secret_version_by_project_and_location_and_secret(
        &self,
        req: crate::model::AddSecretVersionRequest,
        _options: gax::options::RequestOptions,
    ) -> Result<crate::model::SecretVersion> {
        let builder = self
            .inner
            .builder(
                reqwest::Method::POST,
                format!(
                    "/v1/projects/{}/locations/{}/secrets/{}:addVersion",
                    req.project, req.location, req.secret
                ),
            )
            .query(&[("alt", "json")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        self.inner.execute(builder, Some(req)).await
    }

    /// Gets metadata for a given Secret.
    async fn get_secret(
        &self,
        req: crate::model::GetSecretRequest,
        _options: gax::options::RequestOptions,
    ) -> Result<crate::model::Secret> {
        let builder = self
            .inner
            .builder(
                reqwest::Method::GET,
                format!("/v1/projects/{}/secrets/{}", req.project, req.secret),
            )
            .query(&[("alt", "json")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        self.inner
            .execute(builder, None::<gax::http_client::NoBody>)
            .await
    }

    /// Deletes a Secret.
    async fn delete_secret(
        &self,
        req: crate::model::DeleteSecretRequest,
        _options: gax::options::RequestOptions,
    ) -> Result<crate::model::Empty> {
        let builder = self
            .inner
            .builder(
                reqwest::Method::DELETE,
                format!("/v1/projects/{}/secrets/{}", req.project, req.secret),
            )
            .query(&[("alt", "json")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        let builder =
            gax::query_parameter::add(builder, "etag", &req.etag).map_err(Error::other)?;
        self.inner
            .execute(builder, None::<gax::http_client::NoBody>)
            .await
    }

    /// Updates metadata of an existing Secret.
    async fn update_secret(
        &self,
        req: crate::model::UpdateSecretRequest,
        _options: gax::options::RequestOptions,
    ) -> Result<crate::model::Secret> {
        let builder = self
            .inner
            .builder(
                reqwest::Method::PATCH,
                format!("/v1/projects/{}/secrets/{}", req.project, req.secret),
            )
            .query(&[("alt", "json")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        let builder = gax::query_parameter::add(
            builder,
            "updateMask",
            &serde_json::to_value(&req.update_mask).map_err(Error::serde)?,
        )
        .map_err(Error::other)?;
        self.inner.execute(builder, Some(req.request_body)).await
    }

    /// Gets metadata for a given Secret.
    async fn get_secret_by_project_and_location_and_secret(
        &self,
        req: crate::model::GetSecretByProjectAndLocationAndSecretRequest,
        _options: gax::options::RequestOptions,
    ) -> Result<crate::model::Secret> {
        let builder = self
            .inner
            .builder(
                reqwest::Method::GET,
                format!(
                    "/v1/projects/{}/locations/{}/secrets/{}",
                    req.project, req.location, req.secret
                ),
            )
            .query(&[("alt", "json")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        self.inner
            .execute(builder, None::<gax::http_client::NoBody>)
            .await
    }

    /// Deletes a Secret.
    async fn delete_secret_by_project_and_location_and_secret(
        &self,
        req: crate::model::DeleteSecretByProjectAndLocationAndSecretRequest,
        _options: gax::options::RequestOptions,
    ) -> Result<crate::model::Empty> {
        let builder = self
            .inner
            .builder(
                reqwest::Method::DELETE,
                format!(
                    "/v1/projects/{}/locations/{}/secrets/{}",
                    req.project, req.location, req.secret
                ),
            )
            .query(&[("alt", "json")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        let builder =
            gax::query_parameter::add(builder, "etag", &req.etag).map_err(Error::other)?;
        self.inner
            .execute(builder, None::<gax::http_client::NoBody>)
            .await
    }

    /// Updates metadata of an existing Secret.
    async fn update_secret_by_project_and_location_and_secret(
        &self,
        req: crate::model::UpdateSecretByProjectAndLocationAndSecretRequest,
        _options: gax::options::RequestOptions,
    ) -> Result<crate::model::Secret> {
        let builder = self
            .inner
            .builder(
                reqwest::Method::PATCH,
                format!(
                    "/v1/projects/{}/locations/{}/secrets/{}",
                    req.project, req.location, req.secret
                ),
            )
            .query(&[("alt", "json")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        let builder = gax::query_parameter::add(
            builder,
            "updateMask",
            &serde_json::to_value(&req.update_mask).map_err(Error::serde)?,
        )
        .map_err(Error::other)?;
        self.inner.execute(builder, Some(req.request_body)).await
    }

    /// Lists SecretVersions. This call does not return secret
    /// data.
    async fn list_secret_versions(
        &self,
        req: crate::model::ListSecretVersionsRequest,
        _options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListSecretVersionsResponse> {
        let builder = self
            .inner
            .builder(
                reqwest::Method::GET,
                format!(
                    "/v1/projects/{}/secrets/{}/versions",
                    req.project, req.secret
                ),
            )
            .query(&[("alt", "json")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        let builder =
            gax::query_parameter::add(builder, "pageSize", &req.page_size).map_err(Error::other)?;
        let builder = gax::query_parameter::add(builder, "pageToken", &req.page_token)
            .map_err(Error::other)?;
        let builder =
            gax::query_parameter::add(builder, "filter", &req.filter).map_err(Error::other)?;
        self.inner
            .execute(builder, None::<gax::http_client::NoBody>)
            .await
    }

    /// Lists SecretVersions. This call does not return secret
    /// data.
    async fn list_secret_versions_by_project_and_location_and_secret(
        &self,
        req: crate::model::ListSecretVersionsByProjectAndLocationAndSecretRequest,
        _options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListSecretVersionsResponse> {
        let builder = self
            .inner
            .builder(
                reqwest::Method::GET,
                format!(
                    "/v1/projects/{}/locations/{}/secrets/{}/versions",
                    req.project, req.location, req.secret
                ),
            )
            .query(&[("alt", "json")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        let builder =
            gax::query_parameter::add(builder, "pageSize", &req.page_size).map_err(Error::other)?;
        let builder = gax::query_parameter::add(builder, "pageToken", &req.page_token)
            .map_err(Error::other)?;
        let builder =
            gax::query_parameter::add(builder, "filter", &req.filter).map_err(Error::other)?;
        self.inner
            .execute(builder, None::<gax::http_client::NoBody>)
            .await
    }

    /// Gets metadata for a SecretVersion.
    ///
    /// `projects/_*_/secrets/_*_/versions/latest` is an alias to the most recently
    /// created SecretVersion.
    async fn get_secret_version(
        &self,
        req: crate::model::GetSecretVersionRequest,
        _options: gax::options::RequestOptions,
    ) -> Result<crate::model::SecretVersion> {
        let builder = self
            .inner
            .builder(
                reqwest::Method::GET,
                format!(
                    "/v1/projects/{}/secrets/{}/versions/{}",
                    req.project, req.secret, req.version
                ),
            )
            .query(&[("alt", "json")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        self.inner
            .execute(builder, None::<gax::http_client::NoBody>)
            .await
    }

    /// Gets metadata for a SecretVersion.
    ///
    /// `projects/_*_/secrets/_*_/versions/latest` is an alias to the most recently
    /// created SecretVersion.
    async fn get_secret_version_by_project_and_location_and_secret_and_version(
        &self,
        req: crate::model::GetSecretVersionByProjectAndLocationAndSecretAndVersionRequest,
        _options: gax::options::RequestOptions,
    ) -> Result<crate::model::SecretVersion> {
        let builder = self
            .inner
            .builder(
                reqwest::Method::GET,
                format!(
                    "/v1/projects/{}/locations/{}/secrets/{}/versions/{}",
                    req.project, req.location, req.secret, req.version
                ),
            )
            .query(&[("alt", "json")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        self.inner
            .execute(builder, None::<gax::http_client::NoBody>)
            .await
    }

    /// Accesses a SecretVersion. This call returns the secret data.
    ///
    /// `projects/_*_/secrets/_*_/versions/latest` is an alias to the most recently
    /// created SecretVersion.
    async fn access_secret_version(
        &self,
        req: crate::model::AccessSecretVersionRequest,
        _options: gax::options::RequestOptions,
    ) -> Result<crate::model::AccessSecretVersionResponse> {
        let builder = self
            .inner
            .builder(
                reqwest::Method::GET,
                format!(
                    "/v1/projects/{}/secrets/{}/versions/{}:access",
                    req.project, req.secret, req.version
                ),
            )
            .query(&[("alt", "json")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        self.inner
            .execute(builder, None::<gax::http_client::NoBody>)
            .await
    }

    /// Accesses a SecretVersion. This call returns the secret data.
    ///
    /// `projects/_*_/secrets/_*_/versions/latest` is an alias to the most recently
    /// created SecretVersion.
    async fn access_secret_version_by_project_and_location_and_secret_and_version(
        &self,
        req: crate::model::AccessSecretVersionByProjectAndLocationAndSecretAndVersionRequest,
        _options: gax::options::RequestOptions,
    ) -> Result<crate::model::AccessSecretVersionResponse> {
        let builder = self
            .inner
            .builder(
                reqwest::Method::GET,
                format!(
                    "/v1/projects/{}/locations/{}/secrets/{}/versions/{}:access",
                    req.project, req.location, req.secret, req.version
                ),
            )
            .query(&[("alt", "json")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        self.inner
            .execute(builder, None::<gax::http_client::NoBody>)
            .await
    }

    /// Disables a SecretVersion.
    ///
    /// Sets the state of the SecretVersion to
    /// DISABLED.
    async fn disable_secret_version(
        &self,
        req: crate::model::DisableSecretVersionRequest,
        _options: gax::options::RequestOptions,
    ) -> Result<crate::model::SecretVersion> {
        let builder = self
            .inner
            .builder(
                reqwest::Method::POST,
                format!(
                    "/v1/projects/{}/secrets/{}/versions/{}:disable",
                    req.project, req.secret, req.version
                ),
            )
            .query(&[("alt", "json")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        self.inner.execute(builder, Some(req)).await
    }

    /// Disables a SecretVersion.
    ///
    /// Sets the state of the SecretVersion to
    /// DISABLED.
    async fn disable_secret_version_by_project_and_location_and_secret_and_version(
        &self,
        req: crate::model::DisableSecretVersionRequest,
        _options: gax::options::RequestOptions,
    ) -> Result<crate::model::SecretVersion> {
        let builder = self
            .inner
            .builder(
                reqwest::Method::POST,
                format!(
                    "/v1/projects/{}/locations/{}/secrets/{}/versions/{}:disable",
                    req.project, req.location, req.secret, req.version
                ),
            )
            .query(&[("alt", "json")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        self.inner.execute(builder, Some(req)).await
    }

    /// Enables a SecretVersion.
    ///
    /// Sets the state of the SecretVersion to
    /// ENABLED.
    async fn enable_secret_version(
        &self,
        req: crate::model::EnableSecretVersionRequest,
        _options: gax::options::RequestOptions,
    ) -> Result<crate::model::SecretVersion> {
        let builder = self
            .inner
            .builder(
                reqwest::Method::POST,
                format!(
                    "/v1/projects/{}/secrets/{}/versions/{}:enable",
                    req.project, req.secret, req.version
                ),
            )
            .query(&[("alt", "json")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        self.inner.execute(builder, Some(req)).await
    }

    /// Enables a SecretVersion.
    ///
    /// Sets the state of the SecretVersion to
    /// ENABLED.
    async fn enable_secret_version_by_project_and_location_and_secret_and_version(
        &self,
        req: crate::model::EnableSecretVersionRequest,
        _options: gax::options::RequestOptions,
    ) -> Result<crate::model::SecretVersion> {
        let builder = self
            .inner
            .builder(
                reqwest::Method::POST,
                format!(
                    "/v1/projects/{}/locations/{}/secrets/{}/versions/{}:enable",
                    req.project, req.location, req.secret, req.version
                ),
            )
            .query(&[("alt", "json")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        self.inner.execute(builder, Some(req)).await
    }

    /// Destroys a SecretVersion.
    ///
    /// Sets the state of the SecretVersion to
    /// DESTROYED and irrevocably destroys the
    /// secret data.
    async fn destroy_secret_version(
        &self,
        req: crate::model::DestroySecretVersionRequest,
        _options: gax::options::RequestOptions,
    ) -> Result<crate::model::SecretVersion> {
        let builder = self
            .inner
            .builder(
                reqwest::Method::POST,
                format!(
                    "/v1/projects/{}/secrets/{}/versions/{}:destroy",
                    req.project, req.secret, req.version
                ),
            )
            .query(&[("alt", "json")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        self.inner.execute(builder, Some(req)).await
    }

    /// Destroys a SecretVersion.
    ///
    /// Sets the state of the SecretVersion to
    /// DESTROYED and irrevocably destroys the
    /// secret data.
    async fn destroy_secret_version_by_project_and_location_and_secret_and_version(
        &self,
        req: crate::model::DestroySecretVersionRequest,
        _options: gax::options::RequestOptions,
    ) -> Result<crate::model::SecretVersion> {
        let builder = self
            .inner
            .builder(
                reqwest::Method::POST,
                format!(
                    "/v1/projects/{}/locations/{}/secrets/{}/versions/{}:destroy",
                    req.project, req.location, req.secret, req.version
                ),
            )
            .query(&[("alt", "json")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        self.inner.execute(builder, Some(req)).await
    }

    /// Sets the access control policy on the specified secret. Replaces any
    /// existing policy.
    ///
    /// Permissions on SecretVersions are enforced according
    /// to the policy set on the associated Secret.
    async fn set_iam_policy(
        &self,
        req: crate::model::SetIamPolicyRequest,
        _options: gax::options::RequestOptions,
    ) -> Result<crate::model::Policy> {
        let builder = self
            .inner
            .builder(
                reqwest::Method::POST,
                format!(
                    "/v1/projects/{}/secrets/{}:setIamPolicy",
                    req.project, req.secret
                ),
            )
            .query(&[("alt", "json")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        self.inner.execute(builder, Some(req)).await
    }

    /// Sets the access control policy on the specified secret. Replaces any
    /// existing policy.
    ///
    /// Permissions on SecretVersions are enforced according
    /// to the policy set on the associated Secret.
    async fn set_iam_policy_by_project_and_location_and_secret(
        &self,
        req: crate::model::SetIamPolicyRequest,
        _options: gax::options::RequestOptions,
    ) -> Result<crate::model::Policy> {
        let builder = self
            .inner
            .builder(
                reqwest::Method::POST,
                format!(
                    "/v1/projects/{}/locations/{}/secrets/{}:setIamPolicy",
                    req.project, req.location, req.secret
                ),
            )
            .query(&[("alt", "json")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        self.inner.execute(builder, Some(req)).await
    }

    /// Gets the access control policy for a secret.
    /// Returns empty policy if the secret exists and does not have a policy set.
    async fn get_iam_policy(
        &self,
        req: crate::model::GetIamPolicyRequest,
        _options: gax::options::RequestOptions,
    ) -> Result<crate::model::Policy> {
        let builder = self
            .inner
            .builder(
                reqwest::Method::GET,
                format!(
                    "/v1/projects/{}/secrets/{}:getIamPolicy",
                    req.project, req.secret
                ),
            )
            .query(&[("alt", "json")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        let builder = gax::query_parameter::add(
            builder,
            "options.requestedPolicyVersion",
            &req.options_requested_policy_version,
        )
        .map_err(Error::other)?;
        self.inner
            .execute(builder, None::<gax::http_client::NoBody>)
            .await
    }

    /// Gets the access control policy for a secret.
    /// Returns empty policy if the secret exists and does not have a policy set.
    async fn get_iam_policy_by_project_and_location_and_secret(
        &self,
        req: crate::model::GetIamPolicyByProjectAndLocationAndSecretRequest,
        _options: gax::options::RequestOptions,
    ) -> Result<crate::model::Policy> {
        let builder = self
            .inner
            .builder(
                reqwest::Method::GET,
                format!(
                    "/v1/projects/{}/locations/{}/secrets/{}:getIamPolicy",
                    req.project, req.location, req.secret
                ),
            )
            .query(&[("alt", "json")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        let builder = gax::query_parameter::add(
            builder,
            "options.requestedPolicyVersion",
            &req.options_requested_policy_version,
        )
        .map_err(Error::other)?;
        self.inner
            .execute(builder, None::<gax::http_client::NoBody>)
            .await
    }

    /// Returns permissions that a caller has for the specified secret.
    /// If the secret does not exist, this call returns an empty set of
    /// permissions, not a NOT_FOUND error.
    ///
    /// Note: This operation is designed to be used for building permission-aware
    /// UIs and command-line tools, not for authorization checking. This operation
    /// may "fail open" without warning.
    async fn test_iam_permissions(
        &self,
        req: crate::model::TestIamPermissionsRequest,
        _options: gax::options::RequestOptions,
    ) -> Result<crate::model::TestIamPermissionsResponse> {
        let builder = self
            .inner
            .builder(
                reqwest::Method::POST,
                format!(
                    "/v1/projects/{}/secrets/{}:testIamPermissions",
                    req.project, req.secret
                ),
            )
            .query(&[("alt", "json")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        self.inner.execute(builder, Some(req)).await
    }

    /// Returns permissions that a caller has for the specified secret.
    /// If the secret does not exist, this call returns an empty set of
    /// permissions, not a NOT_FOUND error.
    ///
    /// Note: This operation is designed to be used for building permission-aware
    /// UIs and command-line tools, not for authorization checking. This operation
    /// may "fail open" without warning.
    async fn test_iam_permissions_by_project_and_location_and_secret(
        &self,
        req: crate::model::TestIamPermissionsRequest,
        _options: gax::options::RequestOptions,
    ) -> Result<crate::model::TestIamPermissionsResponse> {
        let builder = self
            .inner
            .builder(
                reqwest::Method::POST,
                format!(
                    "/v1/projects/{}/locations/{}/secrets/{}:testIamPermissions",
                    req.project, req.location, req.secret
                ),
            )
            .query(&[("alt", "json")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        self.inner.execute(builder, Some(req)).await
    }
}
