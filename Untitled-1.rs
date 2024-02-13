extern crate bollard;

use bollard::container::ListContainersOptions;
use bollard::Docker;

#[tokio::main]
async fn main() -> Result<(), bollard::errors::Error> {
    let docker = Docker::connect_with_local_defaults()?;

    let options = Some(ListContainersOptions::<String> {
        all: true,
        ..Default::default()
    });

    let containers = docker.list_containers(options, None).await?;

    for container in containers {
        println!("ID: {}, Name: {}", container.id, container.names.join(", "));
        println!("Status: {}, State: {}", container.status, container.state);
        println!("Created: {}", container.created);
        println!("Image: {}", container.image);
        println!("Command: {}", container.command.unwrap_or_else(|| "".to_string()));
        println!("-----------------------------------------------------");
    }

    Ok(())
}
