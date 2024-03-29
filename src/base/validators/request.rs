use std::collections::HashMap;

use super::{Validator, Validators};
use crate::base::web::request::Request;

use tera::{Context, Tera};

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

    pub fn url_and_body_template_engine<'a>(
        variables: &HashMap<String, String>,
    ) -> Validator<Request> {
        let mut context = Context::new();

        variables.iter().for_each(|(k, v)| {
            context.insert(k, v);
        });

        let f = move |req: &mut Request| -> Result<(), String> {
            let mut tera = Tera::default();

            let url_rendered = tera
                .render_str(&req.url, &context)
                .map_err(|e| e.to_string())?;

            let body_rendered = tera
                .render_str(&req.body, &context)
                .map_err(|e| e.to_string())?;

            req.url = url_rendered;
            req.body = body_rendered;

            Ok(())
        };

        Box::new(f)
    }

    pub fn headers_template_engine<'a>(variables: &HashMap<String, String>) -> Validator<Request> {
        let mut context = Context::new();

        variables.iter().for_each(|(k, v)| {
            context.insert(k, v);
        });

        let f = move |req: &mut Request| -> Result<(), String> {
            let mut tera = Tera::default();

            let headers: HashMap<_, _> = req
                .headers
                .clone()
                .into_iter()
                .map(|(key, value)| {
                    let value_rendered =
                        tera.render_str(&value, &context).map_err(|e| e.to_string());

                    (key, value_rendered.unwrap_or(value))
                })
                .collect();

            req.headers = headers;

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

    #[test]
    fn should_tera_work_in_template() {
        let mut req = Request::default();
        req.url = String::from("url.com/{{ route }}");
        req.body = String::from("Hello {{ name }}, go to {{ route | upper }} page");

        let req_final = ValidatorsHandler::from(&req)
            .execute(vec![Validators::url_and_body_template_engine(
                &HashMap::from([
                    ("route".to_string(), "user".to_string()),
                    ("name".to_string(), "James Dev".to_string()),
                ]),
            )])
            .unwrap();

        assert_eq!(req_final.url, "url.com/user");
        assert_eq!(req_final.body, "Hello James Dev, go to USER page");
    }
}
