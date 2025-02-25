use serde::Serialize;
use utoipa::ToSchema;

#[derive(Serialize, Debug, Clone, Copy, ToSchema)]
pub struct Build {
    pub version: &'static str,
    pub git_sha: &'static str,
    pub build_date: &'static str,
}

pub static BUILD: Build = Build {
    version: env!("CARGO_PKG_VERSION"),
    git_sha: env!("VERGEN_GIT_SHA"),
    build_date: env!("VERGEN_BUILD_TIMESTAMP"),
};
