use super::{Validator, Validators};
use crate::base::{web::request::Request, stores::environment::EnvironmentStore};

impl Validators {
    pub fn url_protocol_request() -> Validator<Request> {
        let f = |req: &mut Request| {
            let has_the_protocol_in_begin =
                regex::Regex::new(r"^((http|https)://)(.+)$").map_err(|e| e.to_string())?;

            if !has_the_protocol_in_begin.is_match(&req.url) {
                let protocol = "http://".to_string();
                req.url = protocol + &req.url;
            }

            Ok(())
        };

        Box::new(f)
    }

    pub fn url_and_body_template_engine<'a>(environment: &'a EnvironmentStore) -> Validator<Request> {
        let f =  |req: &mut Request| {
            Ok(())
        };

        Box::new(f)
    }
}

#[cfg(test)]
mod tests {
    use super::super::ValidatorsHandler;
    use super::*;

    #[test]
    fn should_ignore_if_it_has_http_or_https() {
        // Http
        let mut req_with_http = Request::default();
        req_with_http.url = String::from("http://url.com");

        let req_final = ValidatorsHandler::from(&req_with_http)
            .execute(vec![Validators::url_protocol_request()])
            .unwrap();
        assert_eq!(req_with_http.url, req_final.url);

        // HttpS
        let mut req_with_https = Request::default();
        req_with_https.url = String::from("https://url.com");

        let req_final = ValidatorsHandler::from(&req_with_https)
            .execute(vec![Validators::url_protocol_request()])
            .unwrap();
        assert_eq!(req_with_https.url, req_final.url);
    }

    #[test]
    fn should_modify_if_not_protocol() {
        let mut req = Request::default();
        req.url = String::from("url.com");

        let req_final = ValidatorsHandler::from(&req)
            .execute(vec![Validators::url_protocol_request()])
            .unwrap();
        assert_eq!("http://url.com".to_string(), req_final.url);
    }
}
