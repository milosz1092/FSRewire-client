#[derive(PartialEq)]
pub enum AppStatus {
    Neutral,
    Running,
    Warning,
    Error,
}

pub struct AppState {
    pub status: AppStatus,
    pub msg_text: String,
}

impl AppState {
    pub fn new() -> Self {
        AppState {
            status: AppStatus::Neutral,
            msg_text: "Checking...".to_string(),
        }
    }
}
