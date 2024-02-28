use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct EventData {
    pub push: Vec<Action>,
    pub pull_request: Vec<Action>,
    pub issue: Vec<Action>,
    pub release: Vec<Action>,
    pub watch: Vec<Action>,
    pub fork: Vec<Action>,
    pub member: Vec<Action>,
    pub public: Vec<Action>,
    pub other: Vec<Action>,
}

#[derive(Debug, Deserialize)]
pub struct Action {
    pub docker_container_name: Option<String>,
    pub repository_name: String,
    pub path: String,
}

pub enum Event {
    Push,
    PullRequest,
    Issue,
    Release,
    Watch,
    Fork,
    Member,
    Public,
    Other,
}

impl Event {
    pub fn from_str(event: &str) -> Self {
        match event {
            "push" => Event::Push,
            "pull_request" => Event::PullRequest,
            "issue" => Event::Issue,
            "release" => Event::Release,
            "watch" => Event::Watch,
            "fork" => Event::Fork,
            "member" => Event::Member,
            "public" => Event::Public,
            _ => Event::Other,
        }
    }
}
