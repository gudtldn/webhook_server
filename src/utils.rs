use crate::{event_info::EventData, payload::Repository};
use chrono::Local;
use log::LevelFilter;
use std::{
    fs::{File, OpenOptions},
    io::Write,
};

pub fn init_logger() {
    let log_file = match OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .append(true)
        .open("./logs/webhook.log")
    {
        Ok(file) => Box::new(file),
        Err(_) => {
            Box::new(File::create("./logs/webhook.log").expect("Failed to create or open log file"))
        }
    };

    env_logger::Builder::new()
        .target(env_logger::Target::Pipe(log_file))
        .filter(None, LevelFilter::Debug)
        .format(|buf, record| {
            writeln!(
                buf,
                "[{}] {:<5} [{} | {}] >> {}",
                Local::now().format("%Y-%m-%d %H:%M:%S%.3f"),
                record.level(),
                record.file().unwrap_or("unknown"),
                record.line().unwrap_or(0),
                record.args()
            )
        })
        .init();
}

pub fn get_event_data() -> EventData {
    let action_file = File::open("./actions.json").expect("Failed to open actions file");
    serde_json::from_reader(action_file).expect("Failed to parse actions")
}

pub fn find_repositeory_event<'a>(
    event_data: &'a EventData,
    repository: &'a Option<Repository>,
) -> Option<&'a crate::event_info::Action> {
    dbg!(repository.as_ref().unwrap().full_name.as_ref().unwrap());
    event_data.push.iter().find(|action| {
        &action.repository_full_name == repository.as_ref().unwrap().full_name.as_ref().unwrap()
    })
}
