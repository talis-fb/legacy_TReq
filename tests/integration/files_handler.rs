#[cfg(test)]
mod file_handler {
    use std::path::Path;
    use treq::{
        base::{
            os::{
                file_facades::{requests::RequestFile, FileFacade},
                handler::FileHandler,
            },
            web::request::Request,
        },
        utils::custom_types::uuid::UUID,
    };

    // These tests SHOULD RUN IN SEQUENCE
    // These tests SHOULD RUN IN SEQUENCE
    // These tests SHOULD RUN IN SEQUENCE

    #[test]
    #[ignore]
    fn test1_should_create_folders_corretly_when_they_are_not_created() {
        assert_eq!(folder_exists("/home/talis/.local/share/treq"), false);

        FileHandler::setup_env_folder().unwrap();

        assert_eq!(folder_exists("/home/talis/.local/share/treq"), true);
        assert_eq!(folder_exists("/home/talis/.local/share/treq/data"), true);
        assert_eq!(
            folder_exists("/home/talis/.local/share/treq/requests"),
            true
        );
    }

    #[test]
    #[ignore]
    fn test2_should_not_panic_if_folder_already_exist() {
        assert_eq!(folder_exists("/home/talis/.local/share/treq"), true);
        assert_eq!(folder_exists("/home/talis/.local/share/treq/data"), true);
        assert_eq!(
            folder_exists("/home/talis/.local/share/treq/requests"),
            true
        );

        FileHandler::setup_env_folder().unwrap();

        assert_eq!(folder_exists("/home/talis/.local/share/treq"), true);
        assert_eq!(folder_exists("/home/talis/.local/share/treq/data"), true);
        assert_eq!(
            folder_exists("/home/talis/.local/share/treq/requests"),
            true
        );
    }

    #[test]
    #[ignore]
    fn test3_should_create_files_corretly_by() {
        FileHandler::setup_env_folder().unwrap();

        assert_eq!(folder_is_empty("/home/talis/.local/share/treq/data"), true);
        assert_eq!(
            folder_is_empty("/home/talis/.local/share/treq/requests"),
            true
        );

        let mut handler = FileHandler::default();

        let mut request_1 = Request::default();
        request_1.url = String::from("some_site.com");

        let id_file_request1 = UUID::new();
        let file_request1 = RequestFile::create(id_file_request1.clone(), request_1).unwrap();
        let path_file = format!(
            "/home/talis/.local/share/treq/requests/{}",
            id_file_request1.clone().value
        );

        handler.add_request(Box::new(file_request1));

        assert_eq!(
            folder_is_empty("/home/talis/.local/share/treq/requests"),
            false
        );
        assert_eq!(folder_exists(&path_file), true);
    }

    // Utils

    fn folder_exists(path: &str) -> bool {
        Path::new(path).exists()
    }

    fn folder_is_empty(path: &str) -> bool {
        Path::new(path).read_dir().unwrap().next().is_none()
    }
}
