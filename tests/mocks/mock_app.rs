// use treq::base::os::{handler::FileHandler, file_facades::MockFileFacade};
// use mockall::{automock, mock, predicate::*};


pub struct MockApp {
    // app: App
}

impl MockApp {
    pub fn init() {
        // let opa = MockFileFacade::new();
        // opa.expect_get_content().

        // let file_handler = FileHandler::default();
        // let file_handler = FileHandler::default();
        // let configs = ConfigManager::init(file_handler);
    }

    pub fn push_action() {
        
    }

    fn run_commands_in_queue() {
        
    }

    fn get_commands_result_queue() {
        
    }

    // Here will live a lot of query files

    fn is_finished() {
        
    }
}

// Functions
// -> Define a MockInputEditor, where you can define a closure which receive 
//      value of buffer and return what "user" would change on it. This is
//      called when InputMode is "Vim"
//

// pub fn hello() {
//     while !app.is_finished {
//         match app.get_mode() {
//             InputMode::Help => {
//                 app.set_new_state(DefaultHelpMode::init());
//             }
//
//             InputMode::Insert => {
//                 app.set_new_state(DefaultEditMode::init());
//             }
//
//             _ => {}
//         }
//
//         // Listen queue of user's events to execute --------------------
//         log::info!("Wainting action....");
//         match action_queue_receiver.recv() {
//             Ok(action_to_exec) => {
//                 log::info!("Action {:?}", action_to_exec);
//
//                 let command = app
//                     .get_command_of_action(action_to_exec)
//                     .unwrap_or(Commands::do_nothing());
//
//                 // Add Command to queue
//                 command_handler.add(command);
//
//                 // exec it
//                 let command_result = command_handler.run(&mut app);
//
//                 if let Err(e) = command_result {
//                     app.get_data_store_mut()
//                         .set_log_error(String::from("COMMAND ERROR"), e.to_string())
//                 }
//             }
//             Err(err) => {
//                 log::error!("Action ERROR");
//                 log::error!("{}", err);
//             }
//         }
//     }
// }
