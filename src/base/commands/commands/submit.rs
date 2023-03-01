use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use tokio::sync::mpsc::{self};
use tokio::task::JoinHandle;

use crate::base::actions::Actions;
use crate::base::commands::{CommandTrait, CommandType};
use crate::base::web::response::{Response, ResponseStage};
use crate::commands::{Command, Commands};
use crate::App;

impl Commands {
    pub fn submit() -> Command {
        struct S;
        impl CommandTrait for S {
            fn execute(&self, app: &mut App) -> Result<(), String> {
                app.dispatch_submit();
                Ok(())
            }
        }

        Arc::new(Box::new(S {}))
    }

    pub fn cancel_async_submit() -> Command {
        struct S;
        impl CommandTrait for S {
            fn execute(&self, app: &mut App) -> Result<(), String> {
                let client = app.client_web.as_ref().unwrap().clone();
                let request = app.data_store.as_ref().unwrap().get_request();
                let response_data_store = app.data_store.as_ref().unwrap().get_response();

                let mut response = response_data_store.lock().unwrap();
                response.stage = ResponseStage::Cancelled;

                Ok(())
            }
            fn type_running(&self) -> CommandType {
                CommandType::CancelAsync
            }
            fn get_id(&self) -> String {
                String::from("Submit")
            }
        }

        Arc::new(Box::new(S {}))
    }
    pub fn async_submit() -> Command {
        struct S {
            task_running: Arc<Mutex<Option<JoinHandle<Command>>>>,
        }

        // Here get app data and make the call
        impl CommandTrait for S {
            fn execute(&self, app: &mut App) -> Result<(), String> {
                let client = app.client_web.as_ref().unwrap().clone();
                let request = app.data_store.as_ref().unwrap().get_request();
                let response_data_store = app.data_store.as_ref().unwrap().get_response();

                let data_store = app.get_data_store().clone();

                let renderer = app.renderer.as_ref().unwrap().clone();

                let task = tokio::task::spawn(async move {
                    {
                        let mut data = response_data_store.lock().unwrap();
                        data.stage = ResponseStage::Waiting;
                    }

                    let start_time_request = Instant::now();
                    let (close_timer, mut close_timer_listener) = mpsc::channel::<()>(1);

                    let counter_time_task = tokio::task::spawn({
                        let response = response_data_store.clone();
                        let renderer = renderer.clone();
                        async move {
                            let mut interval = tokio::time::interval(Duration::from_millis(100));
                            loop {
                                tokio::select! {
                                    _ = interval.tick() => {
                                        let elapsed_time = start_time_request.elapsed();

                                        {
                                            let mut data = response.lock().unwrap();
                                            data.response_time = elapsed_time.as_secs_f64();
                                        }

                                        renderer.send(Actions::Null).unwrap();
                                        log::info!("{} counter", elapsed_time.as_millis());
                                    }
                                    _ = close_timer_listener.recv() => {
                                        break;
                                    }
                                }
                            }
                        }
                    });

                    log::info!(" ** INIT SUBMIT");
                    let new_response = client.submit((*request).clone()).await;

                    close_timer.send(()).await.unwrap();
                    counter_time_task.await.unwrap();

                    {
                        let mut data = response_data_store.lock().unwrap();
                        *data = new_response.unwrap_or_else(Response::default_internal_error);
                    }

                    // for re-render
                    renderer.send(Actions::Null).unwrap();

                    Commands::do_nothing()
                });

                {
                    let mut t = self.task_running.lock().unwrap();
                    *t = Some(task);
                }

                Ok(())
            }

            fn type_running(&self) -> CommandType {
                CommandType::Async
            }
            fn get_id(&self) -> String {
                String::from("Submit")
            }
            fn take_task(&self) -> Option<tokio::task::JoinHandle<Command>> {
                self.task_running.lock().unwrap().take()
            }
            fn is_task_begin(&self) -> bool {
                self.task_running.lock().unwrap().is_some()
            }
        }

        Arc::new(Box::new(S {
            task_running: Arc::new(Mutex::new(None)),
        }))
    }
}
