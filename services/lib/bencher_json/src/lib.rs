pub mod auth;
pub mod params;
pub mod project;
pub mod report;
pub mod testbed;

pub use auth::{
    JsonLogin,
    JsonSignup,
    JsonUser,
};
pub use params::ResourceId;
pub use project::{
    JsonNewProject,
    JsonProject,
};
pub use report::{
    JsonAdapter,
    JsonBenchmark,
    JsonBenchmarks,
    JsonLatency,
    JsonNewReport,
    JsonReport,
};
pub use testbed::{
    JsonNewTestbed,
    JsonTestbed,
};
