use super::{Validator, Validators};
use crate::base::web::response::Response;
use serde_json::Value;

impl Validators {
    pub fn set_pretty_json_response() -> Validator<Response> {
        let f = |res: &mut Response| {
            let json_obj: Value = serde_json::from_str(&res.body).map_err(|e| e.to_string())?;
            let prety_body = serde_json::to_string_pretty(&json_obj).map_err(|e| e.to_string())?;
            res.body = prety_body;
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
    fn should_format_string_of_response_body() {
        let mut response = Response::default();
        response.body = r#"{ "StatusCode": "200", "SomeMessenger": "Here's something interesting", "Notes": ["Here live anothers interesting things"] }"#.to_string();

        let response_final = ValidatorsHandler::from(&response)
            .execute(vec![Validators::set_pretty_json_response()])
            .unwrap();

        assert_eq!(
            response_final.body,
            r#"{
  "Notes": [
    "Here live anothers interesting things"
  ],
  "SomeMessenger": "Here's something interesting",
  "StatusCode": "200"
}"#
            .to_string()
        );
    }
}
