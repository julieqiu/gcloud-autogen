// Copyright 2025 Google LLC
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

/// Service for creating, configuring, and deleting Cloud Bigtable Instances and
/// Clusters. Provides access to the Instance and Cluster schemas only, not the
/// tables' metadata or data stored in those tables.
///
/// # Mocking
///
/// Application developers may use this trait to mock the bigtableadmin clients.
///
/// Services gain new RPCs routinely. Consequently, this trait gains new methods
/// too. To avoid breaking applications the trait provides a default
/// implementation for each method. These implementations return an error.
pub trait BigtableInstanceAdmin: std::fmt::Debug + Send + Sync {
    /// Create an instance within a project.
    ///
    /// Note that exactly one of Cluster.serve_nodes and
    /// Cluster.cluster_config.cluster_autoscaling_config can be set. If
    /// serve_nodes is set to non-zero, then the cluster is manually scaled. If
    /// cluster_config.cluster_autoscaling_config is non-empty, then autoscaling is
    /// enabled.
    fn create_instance(
        &self,
        _req: crate::model::CreateInstanceRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Gets information about an instance.
    fn get_instance(
        &self,
        _req: crate::model::GetInstanceRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::Instance>> + Send {
        std::future::ready::<crate::Result<crate::model::Instance>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Lists information about instances in a project.
    fn list_instances(
        &self,
        _req: crate::model::ListInstancesRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::ListInstancesResponse>> + Send
    {
        std::future::ready::<crate::Result<crate::model::ListInstancesResponse>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Updates an instance within a project. This method updates only the display
    /// name and type for an Instance. To update other Instance properties, such as
    /// labels, use PartialUpdateInstance.
    fn update_instance(
        &self,
        _req: crate::model::Instance,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::Instance>> + Send {
        std::future::ready::<crate::Result<crate::model::Instance>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Partially updates an instance within a project. This method can modify all
    /// fields of an Instance and is the preferred way to update an Instance.
    fn partial_update_instance(
        &self,
        _req: crate::model::PartialUpdateInstanceRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Delete an instance from a project.
    fn delete_instance(
        &self,
        _req: crate::model::DeleteInstanceRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<wkt::Empty>> + Send {
        std::future::ready::<crate::Result<wkt::Empty>>(Err(Error::other("unimplemented")))
    }

    /// Creates a cluster within an instance.
    ///
    /// Note that exactly one of Cluster.serve_nodes and
    /// Cluster.cluster_config.cluster_autoscaling_config can be set. If
    /// serve_nodes is set to non-zero, then the cluster is manually scaled. If
    /// cluster_config.cluster_autoscaling_config is non-empty, then autoscaling is
    /// enabled.
    fn create_cluster(
        &self,
        _req: crate::model::CreateClusterRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Gets information about a cluster.
    fn get_cluster(
        &self,
        _req: crate::model::GetClusterRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::Cluster>> + Send {
        std::future::ready::<crate::Result<crate::model::Cluster>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Lists information about clusters in an instance.
    fn list_clusters(
        &self,
        _req: crate::model::ListClustersRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::ListClustersResponse>> + Send
    {
        std::future::ready::<crate::Result<crate::model::ListClustersResponse>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Updates a cluster within an instance.
    ///
    /// Note that UpdateCluster does not support updating
    /// cluster_config.cluster_autoscaling_config. In order to update it, you
    /// must use PartialUpdateCluster.
    fn update_cluster(
        &self,
        _req: crate::model::Cluster,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Partially updates a cluster within a project. This method is the preferred
    /// way to update a Cluster.
    ///
    /// To enable and update autoscaling, set
    /// cluster_config.cluster_autoscaling_config. When autoscaling is enabled,
    /// serve_nodes is treated as an OUTPUT_ONLY field, meaning that updates to it
    /// are ignored. Note that an update cannot simultaneously set serve_nodes to
    /// non-zero and cluster_config.cluster_autoscaling_config to non-empty, and
    /// also specify both in the update_mask.
    ///
    /// To disable autoscaling, clear cluster_config.cluster_autoscaling_config,
    /// and explicitly set a serve_node count via the update_mask.
    fn partial_update_cluster(
        &self,
        _req: crate::model::PartialUpdateClusterRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Deletes a cluster from an instance.
    fn delete_cluster(
        &self,
        _req: crate::model::DeleteClusterRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<wkt::Empty>> + Send {
        std::future::ready::<crate::Result<wkt::Empty>>(Err(Error::other("unimplemented")))
    }

    /// Creates an app profile within an instance.
    fn create_app_profile(
        &self,
        _req: crate::model::CreateAppProfileRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::AppProfile>> + Send {
        std::future::ready::<crate::Result<crate::model::AppProfile>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Gets information about an app profile.
    fn get_app_profile(
        &self,
        _req: crate::model::GetAppProfileRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::AppProfile>> + Send {
        std::future::ready::<crate::Result<crate::model::AppProfile>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Lists information about app profiles in an instance.
    fn list_app_profiles(
        &self,
        _req: crate::model::ListAppProfilesRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::ListAppProfilesResponse>> + Send
    {
        std::future::ready::<crate::Result<crate::model::ListAppProfilesResponse>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Updates an app profile within an instance.
    fn update_app_profile(
        &self,
        _req: crate::model::UpdateAppProfileRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Deletes an app profile from an instance.
    fn delete_app_profile(
        &self,
        _req: crate::model::DeleteAppProfileRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<wkt::Empty>> + Send {
        std::future::ready::<crate::Result<wkt::Empty>>(Err(Error::other("unimplemented")))
    }

    /// Gets the access control policy for an instance resource. Returns an empty
    /// policy if an instance exists but does not have a policy set.
    fn get_iam_policy(
        &self,
        _req: iam_v1::model::GetIamPolicyRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<iam_v1::model::Policy>> + Send {
        std::future::ready::<crate::Result<iam_v1::model::Policy>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Sets the access control policy on an instance resource. Replaces any
    /// existing policy.
    fn set_iam_policy(
        &self,
        _req: iam_v1::model::SetIamPolicyRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<iam_v1::model::Policy>> + Send {
        std::future::ready::<crate::Result<iam_v1::model::Policy>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Returns permissions that the caller has on the specified instance resource.
    fn test_iam_permissions(
        &self,
        _req: iam_v1::model::TestIamPermissionsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<iam_v1::model::TestIamPermissionsResponse>> + Send
    {
        std::future::ready::<crate::Result<iam_v1::model::TestIamPermissionsResponse>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Lists hot tablets in a cluster, within the time range provided. Hot
    /// tablets are ordered based on CPU usage.
    fn list_hot_tablets(
        &self,
        _req: crate::model::ListHotTabletsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::ListHotTabletsResponse>> + Send
    {
        std::future::ready::<crate::Result<crate::model::ListHotTabletsResponse>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::traits::Operations
    fn list_operations(
        &self,
        _req: longrunning::model::ListOperationsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::ListOperationsResponse>>
           + Send {
        std::future::ready::<crate::Result<longrunning::model::ListOperationsResponse>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::traits::Operations
    fn get_operation(
        &self,
        _req: longrunning::model::GetOperationRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::traits::Operations
    fn delete_operation(
        &self,
        _req: longrunning::model::DeleteOperationRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<wkt::Empty>> + Send {
        std::future::ready::<crate::Result<wkt::Empty>>(Err(Error::other("unimplemented")))
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::traits::Operations
    fn cancel_operation(
        &self,
        _req: longrunning::model::CancelOperationRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<wkt::Empty>> + Send {
        std::future::ready::<crate::Result<wkt::Empty>>(Err(Error::other("unimplemented")))
    }

    /// Returns the polling policy.
    fn get_polling_policy(
        &self,
        options: &gax::options::RequestOptions,
    ) -> std::sync::Arc<dyn gax::polling_policy::PollingPolicy>;

    /// Returns the polling backoff policy.
    fn get_polling_backoff_policy(
        &self,
        options: &gax::options::RequestOptions,
    ) -> std::sync::Arc<dyn gax::polling_backoff_policy::PollingBackoffPolicy>;
}

/// Service for creating, configuring, and deleting Cloud Bigtable tables.
///
/// Provides access to the table schemas only, not the data stored within
/// the tables.
///
/// # Mocking
///
/// Application developers may use this trait to mock the bigtableadmin clients.
///
/// Services gain new RPCs routinely. Consequently, this trait gains new methods
/// too. To avoid breaking applications the trait provides a default
/// implementation for each method. These implementations return an error.
pub trait BigtableTableAdmin: std::fmt::Debug + Send + Sync {
    /// Creates a new table in the specified instance.
    /// The table can be created with a full set of initial column families,
    /// specified in the request.
    fn create_table(
        &self,
        _req: crate::model::CreateTableRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::Table>> + Send {
        std::future::ready::<crate::Result<crate::model::Table>>(Err(Error::other("unimplemented")))
    }

    /// Creates a new table from the specified snapshot. The target table must
    /// not exist. The snapshot and the table must be in the same instance.
    ///
    /// Note: This is a private alpha release of Cloud Bigtable snapshots. This
    /// feature is not currently available to most Cloud Bigtable customers. This
    /// feature might be changed in backward-incompatible ways and is not
    /// recommended for production use. It is not subject to any SLA or deprecation
    /// policy.
    fn create_table_from_snapshot(
        &self,
        _req: crate::model::CreateTableFromSnapshotRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Lists all tables served from a specified instance.
    fn list_tables(
        &self,
        _req: crate::model::ListTablesRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::ListTablesResponse>> + Send
    {
        std::future::ready::<crate::Result<crate::model::ListTablesResponse>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Gets metadata information about the specified table.
    fn get_table(
        &self,
        _req: crate::model::GetTableRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::Table>> + Send {
        std::future::ready::<crate::Result<crate::model::Table>>(Err(Error::other("unimplemented")))
    }

    /// Updates a specified table.
    fn update_table(
        &self,
        _req: crate::model::UpdateTableRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Permanently deletes a specified table and all of its data.
    fn delete_table(
        &self,
        _req: crate::model::DeleteTableRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<wkt::Empty>> + Send {
        std::future::ready::<crate::Result<wkt::Empty>>(Err(Error::other("unimplemented")))
    }

    /// Restores a specified table which was accidentally deleted.
    fn undelete_table(
        &self,
        _req: crate::model::UndeleteTableRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Creates a new AuthorizedView in a table.
    fn create_authorized_view(
        &self,
        _req: crate::model::CreateAuthorizedViewRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Lists all AuthorizedViews from a specific table.
    fn list_authorized_views(
        &self,
        _req: crate::model::ListAuthorizedViewsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::ListAuthorizedViewsResponse>> + Send
    {
        std::future::ready::<crate::Result<crate::model::ListAuthorizedViewsResponse>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Gets information from a specified AuthorizedView.
    fn get_authorized_view(
        &self,
        _req: crate::model::GetAuthorizedViewRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::AuthorizedView>> + Send {
        std::future::ready::<crate::Result<crate::model::AuthorizedView>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Updates an AuthorizedView in a table.
    fn update_authorized_view(
        &self,
        _req: crate::model::UpdateAuthorizedViewRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Permanently deletes a specified AuthorizedView.
    fn delete_authorized_view(
        &self,
        _req: crate::model::DeleteAuthorizedViewRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<wkt::Empty>> + Send {
        std::future::ready::<crate::Result<wkt::Empty>>(Err(Error::other("unimplemented")))
    }

    /// Performs a series of column family modifications on the specified table.
    /// Either all or none of the modifications will occur before this method
    /// returns, but data requests received prior to that point may see a table
    /// where only some modifications have taken effect.
    fn modify_column_families(
        &self,
        _req: crate::model::ModifyColumnFamiliesRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::Table>> + Send {
        std::future::ready::<crate::Result<crate::model::Table>>(Err(Error::other("unimplemented")))
    }

    /// Permanently drop/delete a row range from a specified table. The request can
    /// specify whether to delete all rows in a table, or only those that match a
    /// particular prefix.
    fn drop_row_range(
        &self,
        _req: crate::model::DropRowRangeRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<wkt::Empty>> + Send {
        std::future::ready::<crate::Result<wkt::Empty>>(Err(Error::other("unimplemented")))
    }

    /// Generates a consistency token for a Table, which can be used in
    /// CheckConsistency to check whether mutations to the table that finished
    /// before this call started have been replicated. The tokens will be available
    /// for 90 days.
    fn generate_consistency_token(
        &self,
        _req: crate::model::GenerateConsistencyTokenRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<crate::model::GenerateConsistencyTokenResponse>,
    > + Send {
        std::future::ready::<crate::Result<crate::model::GenerateConsistencyTokenResponse>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Checks replication consistency based on a consistency token, that is, if
    /// replication has caught up based on the conditions specified in the token
    /// and the check request.
    fn check_consistency(
        &self,
        _req: crate::model::CheckConsistencyRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::CheckConsistencyResponse>> + Send
    {
        std::future::ready::<crate::Result<crate::model::CheckConsistencyResponse>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Creates a new snapshot in the specified cluster from the specified
    /// source table. The cluster and the table must be in the same instance.
    ///
    /// Note: This is a private alpha release of Cloud Bigtable snapshots. This
    /// feature is not currently available to most Cloud Bigtable customers. This
    /// feature might be changed in backward-incompatible ways and is not
    /// recommended for production use. It is not subject to any SLA or deprecation
    /// policy.
    fn snapshot_table(
        &self,
        _req: crate::model::SnapshotTableRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Gets metadata information about the specified snapshot.
    ///
    /// Note: This is a private alpha release of Cloud Bigtable snapshots. This
    /// feature is not currently available to most Cloud Bigtable customers. This
    /// feature might be changed in backward-incompatible ways and is not
    /// recommended for production use. It is not subject to any SLA or deprecation
    /// policy.
    fn get_snapshot(
        &self,
        _req: crate::model::GetSnapshotRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::Snapshot>> + Send {
        std::future::ready::<crate::Result<crate::model::Snapshot>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Lists all snapshots associated with the specified cluster.
    ///
    /// Note: This is a private alpha release of Cloud Bigtable snapshots. This
    /// feature is not currently available to most Cloud Bigtable customers. This
    /// feature might be changed in backward-incompatible ways and is not
    /// recommended for production use. It is not subject to any SLA or deprecation
    /// policy.
    fn list_snapshots(
        &self,
        _req: crate::model::ListSnapshotsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::ListSnapshotsResponse>> + Send
    {
        std::future::ready::<crate::Result<crate::model::ListSnapshotsResponse>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Permanently deletes the specified snapshot.
    ///
    /// Note: This is a private alpha release of Cloud Bigtable snapshots. This
    /// feature is not currently available to most Cloud Bigtable customers. This
    /// feature might be changed in backward-incompatible ways and is not
    /// recommended for production use. It is not subject to any SLA or deprecation
    /// policy.
    fn delete_snapshot(
        &self,
        _req: crate::model::DeleteSnapshotRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<wkt::Empty>> + Send {
        std::future::ready::<crate::Result<wkt::Empty>>(Err(Error::other("unimplemented")))
    }

    /// Starts creating a new Cloud Bigtable Backup.  The returned backup
    /// [long-running operation][google.longrunning.Operation] can be used to
    /// track creation of the backup. The
    /// [metadata][google.longrunning.Operation.metadata] field type is
    /// [CreateBackupMetadata][google.bigtable.admin.v2.CreateBackupMetadata]. The
    /// [response][google.longrunning.Operation.response] field type is
    /// [Backup][google.bigtable.admin.v2.Backup], if successful. Cancelling the
    /// returned operation will stop the creation and delete the backup.
    ///
    /// [google.bigtable.admin.v2.Backup]: crate::model::Backup
    /// [google.bigtable.admin.v2.CreateBackupMetadata]: crate::model::CreateBackupMetadata
    /// [google.longrunning.Operation]: longrunning::model::Operation
    /// [google.longrunning.Operation.metadata]: longrunning::model::Operation::metadata
    /// [google.longrunning.Operation.response]: longrunning::model::Operation::result
    fn create_backup(
        &self,
        _req: crate::model::CreateBackupRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Gets metadata on a pending or completed Cloud Bigtable Backup.
    fn get_backup(
        &self,
        _req: crate::model::GetBackupRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::Backup>> + Send {
        std::future::ready::<crate::Result<crate::model::Backup>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Updates a pending or completed Cloud Bigtable Backup.
    fn update_backup(
        &self,
        _req: crate::model::UpdateBackupRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::Backup>> + Send {
        std::future::ready::<crate::Result<crate::model::Backup>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Deletes a pending or completed Cloud Bigtable backup.
    fn delete_backup(
        &self,
        _req: crate::model::DeleteBackupRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<wkt::Empty>> + Send {
        std::future::ready::<crate::Result<wkt::Empty>>(Err(Error::other("unimplemented")))
    }

    /// Lists Cloud Bigtable backups. Returns both completed and pending
    /// backups.
    fn list_backups(
        &self,
        _req: crate::model::ListBackupsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::ListBackupsResponse>> + Send
    {
        std::future::ready::<crate::Result<crate::model::ListBackupsResponse>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Create a new table by restoring from a completed backup.  The
    /// returned table [long-running operation][google.longrunning.Operation] can
    /// be used to track the progress of the operation, and to cancel it.  The
    /// [metadata][google.longrunning.Operation.metadata] field type is
    /// [RestoreTableMetadata][google.bigtable.admin.v2.RestoreTableMetadata].  The
    /// [response][google.longrunning.Operation.response] type is
    /// [Table][google.bigtable.admin.v2.Table], if successful.
    ///
    /// [google.bigtable.admin.v2.RestoreTableMetadata]: crate::model::RestoreTableMetadata
    /// [google.bigtable.admin.v2.Table]: crate::model::Table
    /// [google.longrunning.Operation]: longrunning::model::Operation
    /// [google.longrunning.Operation.metadata]: longrunning::model::Operation::metadata
    /// [google.longrunning.Operation.response]: longrunning::model::Operation::result
    fn restore_table(
        &self,
        _req: crate::model::RestoreTableRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Copy a Cloud Bigtable backup to a new backup in the destination cluster
    /// located in the destination instance and project.
    fn copy_backup(
        &self,
        _req: crate::model::CopyBackupRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Gets the access control policy for a Table or Backup resource.
    /// Returns an empty policy if the resource exists but does not have a policy
    /// set.
    fn get_iam_policy(
        &self,
        _req: iam_v1::model::GetIamPolicyRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<iam_v1::model::Policy>> + Send {
        std::future::ready::<crate::Result<iam_v1::model::Policy>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Sets the access control policy on a Table or Backup resource.
    /// Replaces any existing policy.
    fn set_iam_policy(
        &self,
        _req: iam_v1::model::SetIamPolicyRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<iam_v1::model::Policy>> + Send {
        std::future::ready::<crate::Result<iam_v1::model::Policy>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Returns permissions that the caller has on the specified Table or Backup
    /// resource.
    fn test_iam_permissions(
        &self,
        _req: iam_v1::model::TestIamPermissionsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<iam_v1::model::TestIamPermissionsResponse>> + Send
    {
        std::future::ready::<crate::Result<iam_v1::model::TestIamPermissionsResponse>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::traits::Operations
    fn list_operations(
        &self,
        _req: longrunning::model::ListOperationsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::ListOperationsResponse>>
           + Send {
        std::future::ready::<crate::Result<longrunning::model::ListOperationsResponse>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::traits::Operations
    fn get_operation(
        &self,
        _req: longrunning::model::GetOperationRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::traits::Operations
    fn delete_operation(
        &self,
        _req: longrunning::model::DeleteOperationRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<wkt::Empty>> + Send {
        std::future::ready::<crate::Result<wkt::Empty>>(Err(Error::other("unimplemented")))
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::traits::Operations
    fn cancel_operation(
        &self,
        _req: longrunning::model::CancelOperationRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<wkt::Empty>> + Send {
        std::future::ready::<crate::Result<wkt::Empty>>(Err(Error::other("unimplemented")))
    }

    /// Returns the polling policy.
    fn get_polling_policy(
        &self,
        options: &gax::options::RequestOptions,
    ) -> std::sync::Arc<dyn gax::polling_policy::PollingPolicy>;

    /// Returns the polling backoff policy.
    fn get_polling_backoff_policy(
        &self,
        options: &gax::options::RequestOptions,
    ) -> std::sync::Arc<dyn gax::polling_backoff_policy::PollingBackoffPolicy>;
}
