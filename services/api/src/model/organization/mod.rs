use std::str::FromStr;
use std::string::ToString;

use bencher_json::{
    organization::JsonUpdateOrganization, JsonNewOrganization, JsonOrganization, NonEmpty,
    ResourceId, Slug,
};
use bencher_rbac::Organization;
use chrono::Utc;
use diesel::{ExpressionMethods, QueryDsl, Queryable, RunQueryDsl};
use dropshot::HttpError;

use crate::{
    context::{DbConnection, Rbac},
    error::resource_not_found_err,
    model::user::{auth::AuthUser, InsertUser},
    schema::{self, organization as organization_table},
    util::{
        query::{fn_get, fn_get_id, fn_get_uuid},
        resource_id::fn_resource_id,
        slug::unwrap_slug,
        to_date_time,
    },
    ApiError,
};

pub mod member;
pub mod organization_role;

crate::util::typed_id::typed_id!(OrganizationId);
crate::util::typed_uuid::typed_uuid!(OrganizationUuid);

#[derive(diesel::Insertable)]
#[diesel(table_name = organization_table)]
pub struct InsertOrganization {
    pub uuid: OrganizationUuid,
    pub name: String,
    pub slug: String,
    pub created: i64,
    pub modified: i64,
}

impl InsertOrganization {
    pub fn from_json(conn: &mut DbConnection, organization: JsonNewOrganization) -> Self {
        let JsonNewOrganization { name, slug } = organization;
        let slug = unwrap_slug!(conn, name.as_ref(), slug, organization, QueryOrganization);
        let timestamp = Utc::now().timestamp();
        Self {
            uuid: OrganizationUuid::new(),
            name: name.into(),
            slug,
            created: timestamp,
            modified: timestamp,
        }
    }

    pub fn from_user(insert_user: &InsertUser) -> Self {
        let timestamp = Utc::now().timestamp();
        Self {
            uuid: OrganizationUuid::new(),
            name: insert_user.name.clone(),
            slug: insert_user.slug.clone(),
            created: timestamp,
            modified: timestamp,
        }
    }
}

fn_resource_id!(organization);

#[derive(Debug, Clone, Queryable, diesel::Identifiable)]
#[diesel(table_name = organization_table)]
pub struct QueryOrganization {
    pub id: OrganizationId,
    pub uuid: OrganizationUuid,
    pub name: String,
    pub slug: String,
    pub subscription: Option<String>,
    pub license: Option<String>,
    pub created: i64,
    pub modified: i64,
}

#[cfg(feature = "plus")]
pub struct LicenseUsage {
    pub entitlements: u64,
    pub usage: u64,
}

impl QueryOrganization {
    fn_get!(organization);
    fn_get_id!(organization, OrganizationId);
    fn_get_uuid!(organization, OrganizationId, OrganizationUuid);

    pub fn from_resource_id(
        conn: &mut DbConnection,
        organization: &ResourceId,
    ) -> Result<Self, HttpError> {
        schema::organization::table
            .filter(resource_id(organization)?)
            .first::<QueryOrganization>(conn)
            .map_err(resource_not_found_err!(Organization, organization.clone()))
    }

    #[cfg(feature = "plus")]
    pub fn get_subscription(
        conn: &mut DbConnection,
        organization: &ResourceId,
    ) -> Result<Option<bencher_billing::SubscriptionId>, HttpError> {
        let organization = Self::from_resource_id(conn, organization)?;

        Ok(if let Some(subscription) = &organization.subscription {
            Some(bencher_billing::SubscriptionId::from_str(subscription).map_err(|e| {
                crate::error::issue_error(
                    http::StatusCode::INTERNAL_SERVER_ERROR,
                    "Failed to parse subscription ID",
                    &format!("Failed to parse subscription ID ({subscription}) for organization ({organization:?})"),
                    e,
                )
            })?)
        } else {
            None
        })
    }

    #[cfg(feature = "plus")]
    pub fn get_license(
        conn: &mut DbConnection,
        organization: &ResourceId,
    ) -> Result<Option<(Self, bencher_json::Jwt)>, HttpError> {
        let organization = Self::from_resource_id(conn, organization)?;

        Ok(if let Some(license) = &organization.license {
            let license_jwt = bencher_json::Jwt::from_str(license).map_err(|e| {
                crate::error::issue_error(
                    http::StatusCode::INTERNAL_SERVER_ERROR,
                    "Failed to parse subscription license",
                    &format!("Failed to parse subscription license ({license}) for organization ({organization:?})"),
                    e,
                )
            })?;
            Some((organization, license_jwt))
        } else {
            None
        })
    }

    #[cfg(feature = "plus")]
    pub fn check_license_usage(
        &self,
        conn: &mut DbConnection,
        licensor: &bencher_license::Licensor,
        license: &bencher_json::Jwt,
    ) -> Result<LicenseUsage, HttpError> {
        let token_data = licensor
            .validate_organization(license, self.uuid.into())
            .map_err(crate::error::payment_required_error)?;

        let start_time = i64::try_from(token_data.claims.iat).map_err(|e| {
            crate::error::issue_error(
                http::StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to parse license start time",
                &format!(
                    "Failed to parse license start time ({start}).",
                    start = token_data.claims.iat,
                ),
                e,
            )
        })?;
        let end_time = i64::try_from(token_data.claims.exp).map_err(|e| {
            crate::error::issue_error(
                http::StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to parse license end time",
                &format!(
                    "Failed to parse license end time ({end}).",
                    end = token_data.claims.exp
                ),
                e,
            )
        })?;

        let usage =
            super::project::metric::QueryMetric::usage(conn, self.id, start_time, end_time)?;
        let entitlements = licensor
            .validate_usage(&token_data.claims, usage)
            .map_err(crate::error::payment_required_error)?;

        Ok(LicenseUsage {
            entitlements,
            usage,
        })
    }

    pub fn is_allowed_resource_id(
        conn: &mut DbConnection,
        rbac: &Rbac,
        organization: &ResourceId,
        auth_user: &AuthUser,
        permission: bencher_rbac::organization::Permission,
    ) -> Result<Self, ApiError> {
        let query_organization = QueryOrganization::from_resource_id(conn, organization)?;

        rbac.is_allowed_organization(auth_user, permission, &query_organization)?;

        Ok(query_organization)
    }

    pub fn is_allowed_id(
        conn: &mut DbConnection,
        rbac: &Rbac,
        organization_id: OrganizationId,
        auth_user: &AuthUser,
        permission: bencher_rbac::organization::Permission,
    ) -> Result<Self, ApiError> {
        let query_organization = schema::organization::table
            .filter(schema::organization::id.eq(organization_id))
            .first(conn)
            .map_err(ApiError::from)?;

        rbac.is_allowed_organization(auth_user, permission, &query_organization)?;

        Ok(query_organization)
    }

    pub fn into_json(self) -> Result<JsonOrganization, ApiError> {
        let Self {
            uuid,
            name,
            slug,
            created,
            modified,
            ..
        } = self;
        Ok(JsonOrganization {
            uuid: uuid.into(),
            name: NonEmpty::from_str(&name).map_err(ApiError::from)?,
            slug: Slug::from_str(&slug).map_err(ApiError::from)?,
            created: to_date_time(created).map_err(ApiError::from)?,
            modified: to_date_time(modified).map_err(ApiError::from)?,
        })
    }
}

impl From<&QueryOrganization> for Organization {
    fn from(organization: &QueryOrganization) -> Self {
        Organization {
            id: organization.id.to_string(),
        }
    }
}

#[derive(Debug, Clone, diesel::AsChangeset)]
#[diesel(table_name = organization_table)]
pub struct UpdateOrganization {
    pub name: Option<String>,
    pub slug: Option<String>,
    pub modified: i64,
}

impl From<JsonUpdateOrganization> for UpdateOrganization {
    fn from(update: JsonUpdateOrganization) -> Self {
        let JsonUpdateOrganization { name, slug } = update;
        Self {
            name: name.map(Into::into),
            slug: slug.map(Into::into),
            modified: Utc::now().timestamp(),
        }
    }
}
