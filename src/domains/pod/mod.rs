use async_trait::async_trait;
use bollard::{image::CreateImageOptions, Docker};
use serde::{Deserialize, Serialize};
use serde_json::value::RawValue;
use futures_util::TryStreamExt;

use crate::error::NetherilErr;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Provider {
    Docker,
}

#[derive(Debug, Deserialize)]
pub struct Manifest {
    provider: Provider,
    config: Box<RawValue>,
}

enum Configuration {
    Docker(DockerConfiguration),
}

#[derive(Clone, Deserialize)]
struct DockerConfiguration {
    image: String,
}

impl DockerConfiguration {
    fn image(&self) -> String {
        self.image.clone()
    }
}

fn parse(manifest: Manifest) -> Result<Configuration, NetherilErr> {
    match manifest.provider {
        Provider::Docker => {
            let config = serde_json::from_str(manifest.config.get())
                .map_err(|e| NetherilErr::PodConfigurationError(e.to_string()))?;
            Ok(Configuration::Docker(config))
        }
    }
}

struct DockerRunner {
    config: DockerConfiguration,
    client: Docker,
}

impl DockerRunner {
    fn new(config: DockerConfiguration) -> Result<Self, NetherilErr> {
        let client = Docker::connect_with_local_defaults()
            .map_err(|e| NetherilErr::Runner(e.to_string()))?;

        Ok(Self { config, client })
    }
}

#[async_trait]
impl Runner for DockerRunner {
    async fn run(&mut self) -> Result<(), NetherilErr> {
	let options = CreateImageOptions {
	    from_image: self.config.image(),
	    ..Default::default()
	};

        let ok = self.client
            .create_image(Some(options), None, None)
            .try_collect::<Vec<_>>()
	    .await
            .map_err(|e| NetherilErr::Runner(e.to_string()))?;
	println!("streams: {:?}", ok);

        Ok(())
    }
}

fn create(configuration: Configuration) -> Result<impl Runner, NetherilErr> {
    match configuration {
        Configuration::Docker(config) => DockerRunner::new(config),
    }
}

#[async_trait]
trait Runner {
    async fn run(&mut self) -> Result<(), NetherilErr>;
}

#[cfg(test)]
mod test {
    use super::*;
    use serde_json::json;

    #[test]
    #[should_panic]
    fn it_doesnt_parse_invalid_provider() {
        let raw = json!({
            "provider": "UNKNOWN",
            "config": {
            "a": "b",
            }
        });
        let _manifest: Manifest = serde_json::from_value(raw).unwrap();
    }

    #[test]
    fn it_should_parse_a_valid_docker_configuration() {
        let image_name = "nginx:latest";

        let raw = json!({
            "provider": "DOCKER",
            "config" : {
            "image": image_name,
            },
        });

        let manifest: Manifest = serde_json::from_value(raw).unwrap();

        let Configuration::Docker(pod_configuration) = parse(manifest).unwrap();
        assert_eq!(image_name, pod_configuration.image());
    }

    #[tokio::test]
    async fn it_should_start_a_docker_image() {
        let raw = json!({
            "provider": "DOCKER",
            "config": {
		"image": "nginx:latest",
            }
        });

        let manifest: Manifest = serde_json::from_value(raw).unwrap();
	let config = parse(manifest).unwrap();
	let mut runner = create(config).unwrap();

	runner.run().await.unwrap()
    }
}
