#[cfg(test)]
mod file_handler {
    use std::{collections::HashMap, path::Path};
    use treq::{
        base::{
            os::{
                file_facades::{
                    requests::RequestFile, variables::VariablesFile,
                    FileFacade,
                },
                handler::FileHandler,
            },
            web::request::Request,
        },
        utils::custom_types::uuid::UUID,
    };

    // These tests SHOULD RUN IN SEQUENCE
    // These tests SHOULD RUN IN SEQUENCE
    // These tests SHOULD RUN IN SEQUENCE
    //
    // Because of that, this test should be run with follow option...
    // $ cargo test --release -- --test-threads=1 --ignored

    #[test]
    #[ignore]
    fn test1_should_create_env_folders_corretly_when_they_are_not_created() {
        assert_eq!(folder_exists("/home/appuser/.local/share/treq"), false);

        FileHandler::setup_env_folder().unwrap();

        assert_eq!(folder_exists("/home/appuser/.local/share/treq"), true);
        assert_eq!(folder_exists("/home/appuser/.local/share/treq/data"), true);
        assert_eq!(
            folder_exists("/home/appuser/.local/share/treq/requests"),
            true
        );
    }

    #[test]
    #[ignore]
    fn test2_should_not_panic_if_env_folders_already_exist() {
        assert_eq!(folder_exists("/home/appuser/.local/share/treq"), true);
        assert_eq!(folder_exists("/home/appuser/.local/share/treq/data"), true);
        assert_eq!(
            folder_exists("/home/appuser/.local/share/treq/requests"),
            true
        );

        FileHandler::setup_env_folder().unwrap();

        assert_eq!(folder_exists("/home/appuser/.local/share/treq"), true);
        assert_eq!(folder_exists("/home/appuser/.local/share/treq/data"), true);
        assert_eq!(
            folder_exists("/home/appuser/.local/share/treq/requests"),
            true
        );
    }

    #[test]
    #[ignore]
    fn test3_should_create_files_of_file_facades_corretly() {
        assert_eq!(folder_is_empty("/home/appuser/.local/share/treq/data"), true);
        assert_eq!(
            folder_is_empty("/home/appuser/.local/share/treq/requests"),
            true
        );

        let mut handler = FileHandler::default();

        // Request folder -----------------------
        let mut request_1 = Request::default();
        request_1.url = String::from("some_site.com");

        let id_file_request1 = UUID::new();
        let file_request1 =
            RequestFile::create(id_file_request1.clone(), request_1.clone()).unwrap();
        let path_file = format!(
            "/home/appuser/.local/share/treq/requests/{}",
            id_file_request1.value
        );

        let id = handler.add_request(Box::new(file_request1));

        assert_eq!(
            folder_is_empty("/home/appuser/.local/share/treq/requests"),
            false
        );
        assert_eq!(folder_exists(&path_file), true);

        let content_file = handler.get_content_request_file(&id).unwrap();
        assert_eq!(content_file.url, request_1.url);
        assert_eq!(content_file.body, request_1.body);
        assert_eq!(content_file.headers, request_1.headers);

        request_1.url = String::from("another_site.com");
        handler
            .save_content_request_file(&id, request_1.clone())
            .unwrap();

        let content_file = handler.get_content_request_file(&id).unwrap();
        assert_eq!(content_file.url, request_1.url);
        assert_eq!(content_file.body, request_1.body);
        assert_eq!(content_file.headers, request_1.headers);

        // Data folder -----------------------
        let new_variables = HashMap::from([("key1".to_string(), "value1".to_string())]);
        let id_file_variables = UUID::new();
        let file = VariablesFile::create(id_file_variables.value, new_variables).unwrap();

        let key = handler.add_variables(Box::new(file));
        let content_file = handler.get_content_variable_file(&key).unwrap();

        assert_eq!(content_file.get("key1").unwrap(), "value1");
    }

    #[test]
    #[ignore]
    fn test4_should_create_files_corretly_even_when_is_already_files_in_folder() {
        assert_eq!(folder_is_empty("/home/appuser/.local/share/treq/data"), false);
        assert_eq!(
            folder_is_empty("/home/appuser/.local/share/treq/requests"),
            false
        );

        // It's a new instance of FileHandler, but files of last tests are still there
        let mut handler = FileHandler::default();

        let mut new_request = Request::default();
        new_request.url = String::from("also_some_site.com");
        new_request.body = String::from(r#"{ "hello": "world" }"#);

        let id_file_request1 = UUID::new();
        let file_request1 =
            RequestFile::create(id_file_request1.clone(), new_request.clone()).unwrap();
        let path_file = format!(
            "/home/appuser/.local/share/treq/requests/{}",
            id_file_request1.value
        );

        let id = handler.add_request(Box::new(file_request1));

        assert_eq!(folder_exists(&path_file), true);

        let content_file = handler.get_content_request_file(&id).unwrap();
        assert_eq!(content_file.url, new_request.url);
        assert_eq!(content_file.body, new_request.body);
        assert_eq!(content_file.headers, new_request.headers);

        new_request.url = String::from("another_site.com");
        handler
            .save_content_request_file(&id, new_request.clone())
            .unwrap();

        let content_file = handler.get_content_request_file(&id).unwrap();
        assert_eq!(content_file.url, new_request.url);
        assert_eq!(content_file.body, new_request.body);
        assert_eq!(content_file.headers, new_request.headers);
    }

    // ------------
    // Utils
    // ------------

    fn folder_exists(path: &str) -> bool {
        Path::new(path).exists()
    }

    fn folder_is_empty(path: &str) -> bool {
        Path::new(path).read_dir().unwrap().next().is_none()
    }
}
