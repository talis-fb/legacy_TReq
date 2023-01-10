use std::sync::{Arc, RwLock};

pub mod requests_active;

use crate::{
    app::{app::InputMode, states::StatesNames},
    input::buffer::{InputBuffer, InputKeyboardBuffer},
};

use self::requests_active::RequestStore;

use super::{
    logs::LogType,
    web::{request::Request, response::Response}, doc::handler::DocReaderHandler,
};

use std::sync::Mutex;

use super::logs::Log;

#[derive(Clone)]
pub struct DataStore {
    // Web
    requests: RequestStore,
    last_response: Arc<Mutex<Response>>,

    // States
    pub current_state: StatesNames,

    // Modes / InputMode
    pub mode: InputMode,
    pub input_buffer: InputKeyboardBuffer,

    // Logs
    pub log: Log,

    // DocReader
    pub doc_reader: Option<DocReaderHandler>,
}

impl DataStore {
    pub fn init(requests: RequestStore) -> Self {
        let last_response = Arc::new(Mutex::new(Response::default()));

        Self {
            requests,
            last_response,
            current_state: StatesNames::Default,
            mode: InputMode::Normal,
            input_buffer: InputKeyboardBuffer::init(),
            log: Log::default(),
            doc_reader: None,
        }
    }

    // Logs
    pub fn set_log(&mut self, log_type: LogType, title: String, detail: String) -> () {
        self.log = Log::default()
            .with_type(log_type)
            .with_title(title)
            .with_detail(detail);
    }
    pub fn set_log_error(&mut self, title: String, detail: String) -> () {
        self.set_log(LogType::Error, title, detail)
    }
    pub fn set_log_warning(&mut self, title: String, detail: String) -> () {
        self.set_log(LogType::Warning, title, detail)
    }
    pub fn set_log_helping(&mut self, title: String, detail: String) -> () {
        self.set_log(LogType::Help, title, detail)
    }
    pub fn set_log_input_mode(&mut self) -> () {
        self.set_log(LogType::InputMode, "INSERT".to_string(), "".to_string())
    }
    pub fn clear_log(&mut self) -> () {
        self.set_log(LogType::Empty, "".to_string(), "".to_string())
    }

    pub fn get_request(&self) -> Arc<Request> {
        Arc::new(self.requests.get_request())
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

    pub fn update_request(&mut self, request: Request) -> () {
        self.requests.update_request(request)
    }

    pub fn save_request(&mut self) -> Result<(), String> {
        self.requests.save_current_request()
    }

    pub fn goto_request(&mut self, index: usize) -> Option<()> {
        self.requests.goto_request(index)
    }

    pub fn goto_next_request(&mut self) -> () {
        self.requests.goto_next_request()
    }
    pub fn goto_prev_request(&mut self) -> () {
        self.requests.goto_prev_request()
    }

    pub fn add_request(&mut self) -> usize {
        self.requests.add_request()
    }

    // Response
    pub fn get_response(&self) -> Arc<Mutex<Response>> {
        self.last_response.clone()
    }

    pub fn get_keys_queue(&self) -> String {
        "ai".to_string()
    }

    pub fn get_mode(&self) -> InputMode {
        self.mode.clone()
    }

    // doc_reader
    pub fn set_doc_reader(&mut self, doc_reader: DocReaderHandler) -> () {
        self.doc_reader = Some(doc_reader)
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn should_add_and_point_to_current_request_corretly() {
//         let mut data_store = DataStore::init(vec![]);
//         assert_eq!(data_store.get_request().name, Request::default().name);
//         assert_eq!(data_store.request_history.len(), 1);
//
//         let mut req2 = Request::default();
//         req2.name = String::from("Req2");
//         data_store.add_request(req2);
//
//         let mut req3 = Request::default();
//         req3.name = String::from("Req3");
//         data_store.add_request(req3);
//
//         assert_eq!(data_store.request_history.len(), 3);
//         assert_eq!(data_store.request_ind(), 2);
//         assert_eq!(data_store.get_request().name, "Req3".to_string());
//     }
//
//     #[test]
//     fn should_jump_and_goto_to_requests() {
//         let mut req0 = Request::default();
//         req0.set_name("Req0");
//
//         let mut req1 = Request::default();
//         req1.set_name("Req1");
//
//         let mut req2 = Request::default();
//         req2.set_name("Req2");
//
//         let mut data_store = DataStore::init(vec![req0, req1, req2]);
//
//         // Init in first
//         assert_eq!(data_store.request_ind(), 0);
//         assert_eq!(data_store.get_request().name, "Req0");
//
//         data_store.goto_request(2);
//         assert_eq!(data_store.request_ind(), 2);
//         assert_eq!(data_store.get_request().name, "Req2");
//
//         data_store.goto_request(1);
//         assert_eq!(data_store.request_ind(), 1);
//         assert_eq!(data_store.get_request().name, "Req1");
//
//         data_store.goto_next_request();
//         assert_eq!(data_store.request_ind(), 2);
//         assert_eq!(data_store.get_request().name, "Req2");
//
//         data_store.goto_prev_request();
//         assert_eq!(data_store.request_ind(), 1);
//         assert_eq!(data_store.get_request().name, "Req1");
//     }
//
//     #[test]
//     fn should_update_and_get_current_request() {
//         let mut req = Request::default();
//         let mut data_store = DataStore::init(vec![req.clone()]);
//
//         req.set_name("New name 1");
//         assert_eq!(data_store.get_request().name, Request::default().name);
//
//         data_store.update_request(req);
//         assert_eq!(data_store.get_request().name, "New name 1".to_string());
//
//         let mut req2 = Request::default();
//         req2.set_name("New name 2");
//         data_store.add_request(req2.clone());
//         assert_eq!(data_store.get_request().name, "New name 2".to_string());
//
//         req2.set_name("New name 2 after alter");
//
//         assert_eq!(data_store.get_request().name, "New name 2".to_string());
//         data_store.update_request(req2);
//         assert_eq!(
//             data_store.get_request().name,
//             "New name 2 after alter".to_string()
//         );
//
//         data_store.goto_request(0);
//         assert_eq!(data_store.get_request().name, "New name 1".to_string());
//     }
//
//     #[test]
//     fn should_next_and_prev_request_return() {
//         let mut first = Request::default();
//         first.set_name("First");
//
//         let mut middle = Request::default();
//         middle.set_name("In Middle");
//
//         let mut last = Request::default();
//         last.set_name("Last");
//
//         let mut data_store = DataStore::init(vec![first, middle, last]);
//
//         assert_eq!(data_store.get_request().name, "First".to_string());
//
//         data_store.goto_next_request();
//         assert_eq!(data_store.get_request().name, "In Middle".to_string());
//
//         data_store.goto_next_request();
//         assert_eq!(data_store.get_request().name, "Last".to_string());
//
//         data_store.goto_next_request();
//         assert_eq!(data_store.get_request().name, "First".to_string());
//
//         // Prev
//         assert_eq!(data_store.get_request().name, "First".to_string());
//
//         data_store.goto_prev_request();
//         assert_eq!(data_store.get_request().name, "Last".to_string());
//
//         data_store.goto_prev_request();
//         assert_eq!(data_store.get_request().name, "In Middle".to_string());
//
//         data_store.goto_prev_request();
//         assert_eq!(data_store.get_request().name, "First".to_string());
//     }
// }
