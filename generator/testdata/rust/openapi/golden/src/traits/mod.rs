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

use gax::error::Error;

pub(crate) mod dyntraits;

/// Stores sensitive data such as API keys, passwords, and certificates.
/// Provides convenience while improving security.
///
/// # Mocking
///
/// Application developers may use this trait to mock the secretmanager clients.
///
/// Services gain new RPCs routinely. Consequently, this trait gains new methods
/// too. To avoid breaking applications the trait provides a default
/// implementation for each method. These implementations return an error.
pub trait SecretManagerService: std::fmt::Debug + Send + Sync {

    /// Lists information about the supported locations for this service.
    fn list_locations(
        &self,
        _req: crate::model::ListLocationsRequest,
        _options: gax::options::RequestOptions
    ) -> impl std::future::Future<Output = crate::Result<crate::model::ListLocationsResponse>> + Send {
        std::future::ready::<crate::Result<crate::model::ListLocationsResponse>>(Err(Error::other("unimplemented")))
    }

    /// Gets information about a location.
    fn get_location(
        &self,
        _req: crate::model::GetLocationRequest,
        _options: gax::options::RequestOptions
    ) -> impl std::future::Future<Output = crate::Result<crate::model::Location>> + Send {
        std::future::ready::<crate::Result<crate::model::Location>>(Err(Error::other("unimplemented")))
    }

    /// Lists Secrets.
    fn list_secrets(
        &self,
        _req: crate::model::ListSecretsRequest,
        _options: gax::options::RequestOptions
    ) -> impl std::future::Future<Output = crate::Result<crate::model::ListSecretsResponse>> + Send {
        std::future::ready::<crate::Result<crate::model::ListSecretsResponse>>(Err(Error::other("unimplemented")))
    }

    /// Creates a new Secret containing no SecretVersions.
    fn create_secret(
        &self,
        _req: crate::model::CreateSecretRequest,
        _options: gax::options::RequestOptions
    ) -> impl std::future::Future<Output = crate::Result<crate::model::Secret>> + Send {
        std::future::ready::<crate::Result<crate::model::Secret>>(Err(Error::other("unimplemented")))
    }

    /// Lists Secrets.
    fn list_secrets_by_project_and_location(
        &self,
        _req: crate::model::ListSecretsByProjectAndLocationRequest,
        _options: gax::options::RequestOptions
    ) -> impl std::future::Future<Output = crate::Result<crate::model::ListSecretsResponse>> + Send {
        std::future::ready::<crate::Result<crate::model::ListSecretsResponse>>(Err(Error::other("unimplemented")))
    }

    /// Creates a new Secret containing no SecretVersions.
    fn create_secret_by_project_and_location(
        &self,
        _req: crate::model::CreateSecretByProjectAndLocationRequest,
        _options: gax::options::RequestOptions
    ) -> impl std::future::Future<Output = crate::Result<crate::model::Secret>> + Send {
        std::future::ready::<crate::Result<crate::model::Secret>>(Err(Error::other("unimplemented")))
    }

    /// Creates a new SecretVersion containing secret data and attaches
    /// it to an existing Secret.
    fn add_secret_version(
        &self,
        _req: crate::model::AddSecretVersionRequest,
        _options: gax::options::RequestOptions
    ) -> impl std::future::Future<Output = crate::Result<crate::model::SecretVersion>> + Send {
        std::future::ready::<crate::Result<crate::model::SecretVersion>>(Err(Error::other("unimplemented")))
    }

    /// Creates a new SecretVersion containing secret data and attaches
    /// it to an existing Secret.
    fn add_secret_version_by_project_and_location_and_secret(
        &self,
        _req: crate::model::AddSecretVersionRequest,
        _options: gax::options::RequestOptions
    ) -> impl std::future::Future<Output = crate::Result<crate::model::SecretVersion>> + Send {
        std::future::ready::<crate::Result<crate::model::SecretVersion>>(Err(Error::other("unimplemented")))
    }

    /// Gets metadata for a given Secret.
    fn get_secret(
        &self,
        _req: crate::model::GetSecretRequest,
        _options: gax::options::RequestOptions
    ) -> impl std::future::Future<Output = crate::Result<crate::model::Secret>> + Send {
        std::future::ready::<crate::Result<crate::model::Secret>>(Err(Error::other("unimplemented")))
    }

    /// Deletes a Secret.
    fn delete_secret(
        &self,
        _req: crate::model::DeleteSecretRequest,
        _options: gax::options::RequestOptions
    ) -> impl std::future::Future<Output = crate::Result<crate::model::Empty>> + Send {
        std::future::ready::<crate::Result<crate::model::Empty>>(Err(Error::other("unimplemented")))
    }

    /// Updates metadata of an existing Secret.
    fn update_secret(
        &self,
        _req: crate::model::UpdateSecretRequest,
        _options: gax::options::RequestOptions
    ) -> impl std::future::Future<Output = crate::Result<crate::model::Secret>> + Send {
        std::future::ready::<crate::Result<crate::model::Secret>>(Err(Error::other("unimplemented")))
    }

    /// Gets metadata for a given Secret.
    fn get_secret_by_project_and_location_and_secret(
        &self,
        _req: crate::model::GetSecretByProjectAndLocationAndSecretRequest,
        _options: gax::options::RequestOptions
    ) -> impl std::future::Future<Output = crate::Result<crate::model::Secret>> + Send {
        std::future::ready::<crate::Result<crate::model::Secret>>(Err(Error::other("unimplemented")))
    }

    /// Deletes a Secret.
    fn delete_secret_by_project_and_location_and_secret(
        &self,
        _req: crate::model::DeleteSecretByProjectAndLocationAndSecretRequest,
        _options: gax::options::RequestOptions
    ) -> impl std::future::Future<Output = crate::Result<crate::model::Empty>> + Send {
        std::future::ready::<crate::Result<crate::model::Empty>>(Err(Error::other("unimplemented")))
    }

    /// Updates metadata of an existing Secret.
    fn update_secret_by_project_and_location_and_secret(
        &self,
        _req: crate::model::UpdateSecretByProjectAndLocationAndSecretRequest,
        _options: gax::options::RequestOptions
    ) -> impl std::future::Future<Output = crate::Result<crate::model::Secret>> + Send {
        std::future::ready::<crate::Result<crate::model::Secret>>(Err(Error::other("unimplemented")))
    }

    /// Lists SecretVersions. This call does not return secret
    /// data.
    fn list_secret_versions(
        &self,
        _req: crate::model::ListSecretVersionsRequest,
        _options: gax::options::RequestOptions
    ) -> impl std::future::Future<Output = crate::Result<crate::model::ListSecretVersionsResponse>> + Send {
        std::future::ready::<crate::Result<crate::model::ListSecretVersionsResponse>>(Err(Error::other("unimplemented")))
    }

    /// Lists SecretVersions. This call does not return secret
    /// data.
    fn list_secret_versions_by_project_and_location_and_secret(
        &self,
        _req: crate::model::ListSecretVersionsByProjectAndLocationAndSecretRequest,
        _options: gax::options::RequestOptions
    ) -> impl std::future::Future<Output = crate::Result<crate::model::ListSecretVersionsResponse>> + Send {
        std::future::ready::<crate::Result<crate::model::ListSecretVersionsResponse>>(Err(Error::other("unimplemented")))
    }

    /// Gets metadata for a SecretVersion.
    ///
    /// `projects/_*_/secrets/_*_/versions/latest` is an alias to the most recently
    /// created SecretVersion.
    fn get_secret_version(
        &self,
        _req: crate::model::GetSecretVersionRequest,
        _options: gax::options::RequestOptions
    ) -> impl std::future::Future<Output = crate::Result<crate::model::SecretVersion>> + Send {
        std::future::ready::<crate::Result<crate::model::SecretVersion>>(Err(Error::other("unimplemented")))
    }

    /// Gets metadata for a SecretVersion.
    ///
    /// `projects/_*_/secrets/_*_/versions/latest` is an alias to the most recently
    /// created SecretVersion.
    fn get_secret_version_by_project_and_location_and_secret_and_version(
        &self,
        _req: crate::model::GetSecretVersionByProjectAndLocationAndSecretAndVersionRequest,
        _options: gax::options::RequestOptions
    ) -> impl std::future::Future<Output = crate::Result<crate::model::SecretVersion>> + Send {
        std::future::ready::<crate::Result<crate::model::SecretVersion>>(Err(Error::other("unimplemented")))
    }

    /// Accesses a SecretVersion. This call returns the secret data.
    ///
    /// `projects/_*_/secrets/_*_/versions/latest` is an alias to the most recently
    /// created SecretVersion.
    fn access_secret_version(
        &self,
        _req: crate::model::AccessSecretVersionRequest,
        _options: gax::options::RequestOptions
    ) -> impl std::future::Future<Output = crate::Result<crate::model::AccessSecretVersionResponse>> + Send {
        std::future::ready::<crate::Result<crate::model::AccessSecretVersionResponse>>(Err(Error::other("unimplemented")))
    }

    /// Accesses a SecretVersion. This call returns the secret data.
    ///
    /// `projects/_*_/secrets/_*_/versions/latest` is an alias to the most recently
    /// created SecretVersion.
    fn access_secret_version_by_project_and_location_and_secret_and_version(
        &self,
        _req: crate::model::AccessSecretVersionByProjectAndLocationAndSecretAndVersionRequest,
        _options: gax::options::RequestOptions
    ) -> impl std::future::Future<Output = crate::Result<crate::model::AccessSecretVersionResponse>> + Send {
        std::future::ready::<crate::Result<crate::model::AccessSecretVersionResponse>>(Err(Error::other("unimplemented")))
    }

    /// Disables a SecretVersion.
    ///
    /// Sets the state of the SecretVersion to
    /// DISABLED.
    fn disable_secret_version(
        &self,
        _req: crate::model::DisableSecretVersionRequest,
        _options: gax::options::RequestOptions
    ) -> impl std::future::Future<Output = crate::Result<crate::model::SecretVersion>> + Send {
        std::future::ready::<crate::Result<crate::model::SecretVersion>>(Err(Error::other("unimplemented")))
    }

    /// Disables a SecretVersion.
    ///
    /// Sets the state of the SecretVersion to
    /// DISABLED.
    fn disable_secret_version_by_project_and_location_and_secret_and_version(
        &self,
        _req: crate::model::DisableSecretVersionRequest,
        _options: gax::options::RequestOptions
    ) -> impl std::future::Future<Output = crate::Result<crate::model::SecretVersion>> + Send {
        std::future::ready::<crate::Result<crate::model::SecretVersion>>(Err(Error::other("unimplemented")))
    }

    /// Enables a SecretVersion.
    ///
    /// Sets the state of the SecretVersion to
    /// ENABLED.
    fn enable_secret_version(
        &self,
        _req: crate::model::EnableSecretVersionRequest,
        _options: gax::options::RequestOptions
    ) -> impl std::future::Future<Output = crate::Result<crate::model::SecretVersion>> + Send {
        std::future::ready::<crate::Result<crate::model::SecretVersion>>(Err(Error::other("unimplemented")))
    }

    /// Enables a SecretVersion.
    ///
    /// Sets the state of the SecretVersion to
    /// ENABLED.
    fn enable_secret_version_by_project_and_location_and_secret_and_version(
        &self,
        _req: crate::model::EnableSecretVersionRequest,
        _options: gax::options::RequestOptions
    ) -> impl std::future::Future<Output = crate::Result<crate::model::SecretVersion>> + Send {
        std::future::ready::<crate::Result<crate::model::SecretVersion>>(Err(Error::other("unimplemented")))
    }

    /// Destroys a SecretVersion.
    ///
    /// Sets the state of the SecretVersion to
    /// DESTROYED and irrevocably destroys the
    /// secret data.
    fn destroy_secret_version(
        &self,
        _req: crate::model::DestroySecretVersionRequest,
        _options: gax::options::RequestOptions
    ) -> impl std::future::Future<Output = crate::Result<crate::model::SecretVersion>> + Send {
        std::future::ready::<crate::Result<crate::model::SecretVersion>>(Err(Error::other("unimplemented")))
    }

    /// Destroys a SecretVersion.
    ///
    /// Sets the state of the SecretVersion to
    /// DESTROYED and irrevocably destroys the
    /// secret data.
    fn destroy_secret_version_by_project_and_location_and_secret_and_version(
        &self,
        _req: crate::model::DestroySecretVersionRequest,
        _options: gax::options::RequestOptions
    ) -> impl std::future::Future<Output = crate::Result<crate::model::SecretVersion>> + Send {
        std::future::ready::<crate::Result<crate::model::SecretVersion>>(Err(Error::other("unimplemented")))
    }

    /// Sets the access control policy on the specified secret. Replaces any
    /// existing policy.
    ///
    /// Permissions on SecretVersions are enforced according
    /// to the policy set on the associated Secret.
    fn set_iam_policy(
        &self,
        _req: crate::model::SetIamPolicyRequest,
        _options: gax::options::RequestOptions
    ) -> impl std::future::Future<Output = crate::Result<crate::model::Policy>> + Send {
        std::future::ready::<crate::Result<crate::model::Policy>>(Err(Error::other("unimplemented")))
    }

    /// Sets the access control policy on the specified secret. Replaces any
    /// existing policy.
    ///
    /// Permissions on SecretVersions are enforced according
    /// to the policy set on the associated Secret.
    fn set_iam_policy_by_project_and_location_and_secret(
        &self,
        _req: crate::model::SetIamPolicyRequest,
        _options: gax::options::RequestOptions
    ) -> impl std::future::Future<Output = crate::Result<crate::model::Policy>> + Send {
        std::future::ready::<crate::Result<crate::model::Policy>>(Err(Error::other("unimplemented")))
    }

    /// Gets the access control policy for a secret.
    /// Returns empty policy if the secret exists and does not have a policy set.
    fn get_iam_policy(
        &self,
        _req: crate::model::GetIamPolicyRequest,
        _options: gax::options::RequestOptions
    ) -> impl std::future::Future<Output = crate::Result<crate::model::Policy>> + Send {
        std::future::ready::<crate::Result<crate::model::Policy>>(Err(Error::other("unimplemented")))
    }

    /// Gets the access control policy for a secret.
    /// Returns empty policy if the secret exists and does not have a policy set.
    fn get_iam_policy_by_project_and_location_and_secret(
        &self,
        _req: crate::model::GetIamPolicyByProjectAndLocationAndSecretRequest,
        _options: gax::options::RequestOptions
    ) -> impl std::future::Future<Output = crate::Result<crate::model::Policy>> + Send {
        std::future::ready::<crate::Result<crate::model::Policy>>(Err(Error::other("unimplemented")))
    }

    /// Returns permissions that a caller has for the specified secret.
    /// If the secret does not exist, this call returns an empty set of
    /// permissions, not a NOT_FOUND error.
    ///
    /// Note: This operation is designed to be used for building permission-aware
    /// UIs and command-line tools, not for authorization checking. This operation
    /// may "fail open" without warning.
    fn test_iam_permissions(
        &self,
        _req: crate::model::TestIamPermissionsRequest,
        _options: gax::options::RequestOptions
    ) -> impl std::future::Future<Output = crate::Result<crate::model::TestIamPermissionsResponse>> + Send {
        std::future::ready::<crate::Result<crate::model::TestIamPermissionsResponse>>(Err(Error::other("unimplemented")))
    }

    /// Returns permissions that a caller has for the specified secret.
    /// If the secret does not exist, this call returns an empty set of
    /// permissions, not a NOT_FOUND error.
    ///
    /// Note: This operation is designed to be used for building permission-aware
    /// UIs and command-line tools, not for authorization checking. This operation
    /// may "fail open" without warning.
    fn test_iam_permissions_by_project_and_location_and_secret(
        &self,
        _req: crate::model::TestIamPermissionsRequest,
        _options: gax::options::RequestOptions
    ) -> impl std::future::Future<Output = crate::Result<crate::model::TestIamPermissionsResponse>> + Send {
        std::future::ready::<crate::Result<crate::model::TestIamPermissionsResponse>>(Err(Error::other("unimplemented")))
    }
}

