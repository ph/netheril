use serde::Serialize;

#[derive(Serialize, Debug, Clone, Copy)]
pub struct Build {
    version: &'static str,
    git_sha: &'static str,
    build_date: &'static str,
}

pub static BUILD: Build = Build {
    version: env!("CARGO_PKG_VERSION"),
    git_sha: env!("VERGEN_GIT_SHA"),
    build_date: env!("VERGEN_BUILD_TIMESTAMP"),
};
