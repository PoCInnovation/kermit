use std::env::current_dir;

use testcontainers::{
    ContainerAsync, Image,
    core::{ContainerPort, IntoContainerPort, Mount, WaitFor},
    runners::AsyncRunner,
};

const CONTAINER_MOUNT_PATH: &str = "/alephium-home/.alephium/user.conf";
const EXPOSED_PORT: u16 = 22973;

pub struct AlephiumNodeImage {
    mounts: Vec<Mount>,
    ports: Vec<ContainerPort>,
}

impl AlephiumNodeImage {
    fn new(conf_path: impl Into<String>) -> Self {
        Self {
            mounts: vec![Mount::bind_mount(conf_path, CONTAINER_MOUNT_PATH)],
            ports: vec![EXPOSED_PORT.tcp()],
        }
    }
}

impl Image for AlephiumNodeImage {
    fn name(&self) -> &str {
        "alephium/alephium"
    }

    fn tag(&self) -> &str {
        "latest"
    }

    fn ready_conditions(&self) -> Vec<WaitFor> {
        vec![WaitFor::message_on_stdout("Listening http request on")]
    }

    fn mounts(&self) -> impl IntoIterator<Item = &Mount> {
        &self.mounts
    }

    fn expose_ports(&self) -> &[ContainerPort] {
        &self.ports
    }
}

pub async fn setup_node() -> (ContainerAsync<AlephiumNodeImage>, String) {
    let conf = format!("{}/tests/config/devnet.conf", current_dir().unwrap().display());

    let container = AlephiumNodeImage::new(&conf).start().await.unwrap();

    let host = container.get_host().await.unwrap();
    let port = container
        .get_host_port_ipv4(EXPOSED_PORT.tcp())
        .await
        .unwrap();
    let url = format!("http://{host}:{port}");

    (container, url)
}
