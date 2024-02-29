use log::{error, info};
use std::{env, io::Error, process::Command};

pub fn change_directory(path: &str) -> Result<(), Error> {
    dbg!(path);
    if let Err(err) = env::set_current_dir(path) {
        error!("Failed to change directory: {}", err);
        return Err(err.into());
    }
    info!("Changed directory to {}", path);
    Ok(())
}

pub fn restart_container(container_name: &str) -> Result<(), Error> {
    info!("Restarting container {}", container_name);
    if let Err(err) = Command::new("docker")
        .args(&["restart", container_name])
        .output()
    {
        error!("Failed to restart container {}: {}", container_name, err);
        return Err(err.into());
    }
    info!("Restarted container {}", container_name);
    Ok(())
}

pub fn pull_from_github() -> Result<(), Error> {
    info!("Pulling from GitHub");
    if let Err(err) = Command::new("git").args(&["pull"]).output() {
        error!("Failed to pull from GitHub: {}", err);
        return Err(err.into());
    }
    info!("Pulled from GitHub");
    Ok(())
}
