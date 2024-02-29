use log::info;
use std::{env, io::Error, process::Command};

pub fn change_directory(path: &str) -> Result<(), Error> {
    env::set_current_dir(path)?;
    info!("Changed directory to {}", path);
    Ok(())
}

pub fn restart_container(container_name: &str) -> Result<(), Error> {
    info!("Restarting container {}", container_name);
    Command::new("docker")
        .args(&["restart", container_name])
        .output()?;
    info!("Restarted container {}", container_name);
    Ok(())
}

pub fn pull_from_github() -> Result<(), Error> {
    info!("Pulling from GitHub");
    Command::new("git").args(&["pull"]).output()?;
    info!("Pulled from GitHub");
    Ok(())
}
