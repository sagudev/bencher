mod codegen {
    #![allow(
        unused_qualifications,
        variant_size_differences,
        clippy::all,
        clippy::cargo,
        clippy::pedantic,
        clippy::restriction
    )]
    include!(concat!(env!("OUT_DIR"), "/codegen.rs"));
}

pub use codegen::*;

macro_rules! from_client {
    ($($name:ident),*) => {
        $(
            impl From<bencher_json::$name> for types::$name  {
                fn from(json: bencher_json::$name) -> Self {
                    Self(json.into())
                }
            }
        )*
    };
}

from_client!(
    BenchmarkName,
    Boundary,
    BranchName,
    Email,
    GitHash,
    Jwt,
    NonEmpty,
    ResourceId,
    SampleSize,
    Slug,
    Url,
    UserName
);

from_client!(
    OrganizationUuid,
    ProjectUuid,
    ReportUuid,
    MetricKindUuid,
    BranchUuid,
    TestbedUuid,
    BenchmarkUuid,
    ThresholdUuid,
    StatisticUuid,
    AlertUuid,
    UserUuid,
    TokenUuid
);

#[cfg(feature = "plus")]
from_client!(CardCvc, CardNumber, ExpirationMonth, ExpirationYear);

// This is a bit of a kludge, but it should always work!
macro_rules! try_from_client {
    ($($name:ident),*) => {
        $(
            impl TryFrom<types::$name> for bencher_json::$name  {
                type Error = serde_json::Error;

                fn try_from(json: types::$name) -> Result<Self, Self::Error> {
                    serde_json::from_value::<Self>(serde_json::json!(json))
                }
            }
        )*
    };
}

try_from_client!(
    JsonEmpty,
    JsonMember,
    JsonMembers,
    JsonAllowed,
    JsonOrganization,
    JsonOrganizations,
    JsonAlerts,
    JsonAlertStats,
    JsonAlert,
    JsonBenchmarks,
    JsonBenchmark,
    JsonBranches,
    JsonBranch,
    JsonMetricKinds,
    JsonMetricKind,
    JsonProjects,
    JsonProject,
    JsonPerf,
    JsonReports,
    JsonReport,
    JsonStatistic,
    JsonTestbeds,
    JsonTestbed,
    JsonThresholds,
    JsonThreshold,
    JsonAuthUser,
    JsonLogin,
    JsonSignup,
    JsonConfig,
    JsonEndpoint,
    JsonApiVersion,
    JsonSpec,
    JsonPing,
    JsonTokens,
    JsonToken,
    JsonUser
);

#[cfg(feature = "plus")]
try_from_client!(JsonPlan, JsonUsage);
