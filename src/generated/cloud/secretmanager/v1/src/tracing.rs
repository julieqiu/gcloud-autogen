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

/// Implement a [SecretManagerService](crate::client::) decorator
/// for tracing and logging.
#[derive(Clone, Debug)]
pub struct SecretManagerService<T>
where
    T: crate::traits::SecretManagerService + std::fmt::Debug + Send + Sync,
{
    inner: T,
}

impl<T> SecretManagerService<T>
where
    T: crate::traits::SecretManagerService + std::fmt::Debug + Send + Sync,
{
    pub fn new(inner: T) -> Self {
        Self { inner }
    }
}

impl<T> crate::traits::SecretManagerService for SecretManagerService<T>
where
    T: crate::traits::SecretManagerService + std::fmt::Debug + Send + Sync,
{
    #[tracing::instrument(ret)]
    async fn list_secrets(
        &self,
        req: crate::model::ListSecretsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListSecretsResponse> {
        self.inner.list_secrets(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn create_secret(
        &self,
        req: crate::model::CreateSecretRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Secret> {
        self.inner.create_secret(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn add_secret_version(
        &self,
        req: crate::model::AddSecretVersionRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::SecretVersion> {
        self.inner.add_secret_version(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_secret(
        &self,
        req: crate::model::GetSecretRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Secret> {
        self.inner.get_secret(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn update_secret(
        &self,
        req: crate::model::UpdateSecretRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Secret> {
        self.inner.update_secret(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_secret(
        &self,
        req: crate::model::DeleteSecretRequest,
        options: gax::options::RequestOptions,
    ) -> Result<wkt::Empty> {
        self.inner.delete_secret(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_secret_versions(
        &self,
        req: crate::model::ListSecretVersionsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListSecretVersionsResponse> {
        self.inner.list_secret_versions(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_secret_version(
        &self,
        req: crate::model::GetSecretVersionRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::SecretVersion> {
        self.inner.get_secret_version(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn access_secret_version(
        &self,
        req: crate::model::AccessSecretVersionRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::AccessSecretVersionResponse> {
        self.inner.access_secret_version(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn disable_secret_version(
        &self,
        req: crate::model::DisableSecretVersionRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::SecretVersion> {
        self.inner.disable_secret_version(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn enable_secret_version(
        &self,
        req: crate::model::EnableSecretVersionRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::SecretVersion> {
        self.inner.enable_secret_version(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn destroy_secret_version(
        &self,
        req: crate::model::DestroySecretVersionRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::SecretVersion> {
        self.inner.destroy_secret_version(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn set_iam_policy(
        &self,
        req: iam_v1::model::SetIamPolicyRequest,
        options: gax::options::RequestOptions,
    ) -> Result<iam_v1::model::Policy> {
        self.inner.set_iam_policy(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_iam_policy(
        &self,
        req: iam_v1::model::GetIamPolicyRequest,
        options: gax::options::RequestOptions,
    ) -> Result<iam_v1::model::Policy> {
        self.inner.get_iam_policy(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn test_iam_permissions(
        &self,
        req: iam_v1::model::TestIamPermissionsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<iam_v1::model::TestIamPermissionsResponse> {
        self.inner.test_iam_permissions(req, options).await
    }
}

/// Implement a [Locations](crate::client::) decorator
/// for tracing and logging.
#[derive(Clone, Debug)]
pub struct Locations<T>
where
    T: crate::traits::Locations + std::fmt::Debug + Send + Sync,
{
    inner: T,
}

impl<T> Locations<T>
where
    T: crate::traits::Locations + std::fmt::Debug + Send + Sync,
{
    pub fn new(inner: T) -> Self {
        Self { inner }
    }
}

impl<T> crate::traits::Locations for Locations<T>
where
    T: crate::traits::Locations + std::fmt::Debug + Send + Sync,
{
    #[tracing::instrument(ret)]
    async fn list_locations(
        &self,
        req: location::model::ListLocationsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<location::model::ListLocationsResponse> {
        self.inner.list_locations(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_location(
        &self,
        req: location::model::GetLocationRequest,
        options: gax::options::RequestOptions,
    ) -> Result<location::model::Location> {
        self.inner.get_location(req, options).await
    }
}
