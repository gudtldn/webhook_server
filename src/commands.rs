use log::{error, info};
use std::{env, io::Error, process::Command};

pub struct ChangeDirectory {
    previous_directory: String,
}

impl ChangeDirectory {
    pub fn new(target_directory: &str) -> Result<Self, String> {
        // 현재 디렉토리 저장
        let current_directory =
            env::current_dir().map_err(|e| format!("Failed to get current directory: {}", e))?;

        // 타겟 디렉토리로 변경
        env::set_current_dir(target_directory)
            .map_err(|e| format!("Failed to change directory to {}: {}", target_directory, e))?;

        info!("Changed directory to {}", target_directory);

        Ok(Self {
            previous_directory: current_directory.to_string_lossy().to_string(),
        })
    }
}

impl Drop for ChangeDirectory {
    fn drop(&mut self) {
        // 스코프를 벗어날 때 이전 디렉토리로 돌아옴
        env::set_current_dir(&self.previous_directory)
            .expect("Failed to change directory back to the previous one");
        info!("Changed directory back to {}", self.previous_directory);
    }
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
