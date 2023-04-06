use std::sync::Arc;

pub mod requests;
pub mod environment;
pub mod view;

use crate::base::states::names::StatesNames;
use crate::input::buffer::InputKeyboardBuffer;
use crate::utils::custom_types::uuid::UUID;
use crate::{app::InputMode, config::manager::ConfigManager};

use self::environment::EnvironmentStore;
use self::requests::RequestStore;
use self::view::ViewStore;

use super::{
    doc::handler::DocReaderHandler,
    logs::LogType,
    web::{request::Request, response::Response},
};

use std::sync::Mutex;

use super::logs::Log;

pub struct MainStore {
    // Config
    pub config: ConfigManager,

    // Web
    requests: RequestStore,
    last_response: Arc<Mutex<Response>>,

    // States
    pub current_state: StatesNames,

    // Inputs
    pub mode: InputMode,
    pub input_buffer: InputKeyboardBuffer,

    // Logs
    pub log: Log,

    // DocReader
    pub doc_reader: Option<DocReaderHandler>,

    // Stores
    pub environment: EnvironmentStore,
    pub view: ViewStore,
}

impl MainStore {
    pub fn init(config: ConfigManager) -> Self {
        let last_response = Arc::new(Mutex::new(Response::default()));

        Self {
            requests: RequestStore::init(config.saved_requests.clone()),
            environment: EnvironmentStore::init(config.global_variables.clone()),
            view: ViewStore::init(),
            last_response,
            current_state: StatesNames::Default,
            mode: InputMode::Normal,
            input_buffer: InputKeyboardBuffer::init(),
            log: Log::default(),
            doc_reader: None,
            config,
        }

        // let requests = RequestStore::init(dd.config.saved_requests);
        // dd.requests = requests;
    }

    // Logs
    pub fn set_log(&mut self, log_type: LogType, title: String, detail: String) {
        self.log = Log::default()
            .with_type(log_type)
            .with_title(title)
            .with_detail(detail);
    }
    pub fn set_log_error(&mut self, title: String, detail: String) {
        self.set_log(LogType::Error, title, detail)
    }
    pub fn set_log_warning(&mut self, title: String, detail: String) {
        self.set_log(LogType::Warning, title, detail)
    }
    pub fn set_log_helping(&mut self, title: String, detail: String) {
        self.set_log(LogType::Help, title, detail)
    }
    pub fn set_log_input_mode(&mut self) {
        self.set_log(LogType::InputMode, "INSERT".to_string(), "".to_string())
    }
    pub fn clear_log(&mut self) {
        self.set_log(LogType::Empty, "".to_string(), "".to_string())
    }

    pub fn get_request(&self) -> Arc<Request> {
        Arc::new(self.requests.get_request())
    }

    pub fn get_request_uuid(&self) -> &UUID {
        self.requests.get_request_uuid()
    }

    pub fn get_requests(&self) -> Vec<&Request> {
        self.requests.get_requests()
    }

    pub fn request_ind(&self) -> usize {
        self.requests.request_ind()
    }
    pub fn get_total_requests(&self) -> usize {
        self.requests.get_total_requests()
    }

    pub fn update_request(&mut self, request: Request) {
        self.requests.update_request(request)
    }

    pub fn save_request(&mut self) -> Result<(), String> {
        self.requests.save_current_request()
    }

    pub fn goto_request(&mut self, index: usize) -> Option<()> {
        self.requests.goto_request(index)
    }

    pub fn goto_next_request(&mut self) {
        self.requests.goto_next_request()
    }
    pub fn goto_prev_request(&mut self) {
        self.requests.goto_prev_request()
    }

    pub fn add_request(&mut self) -> usize {
        self.requests.add_request()
    }

    pub fn delete_current_request(&mut self) {
        if let Err(e) = self.requests.delete_current_request() {
            self.log.with_type(LogType::Error).with_detail(e);
        }
    }

    // Response
    pub fn get_response(&self) -> Arc<Mutex<Response>> {
        self.last_response.clone()
    }

    pub fn get_mode(&self) -> InputMode {
        self.mode
    }
}
