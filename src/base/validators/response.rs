// use super::Validator;
// use crate::base::web::request::Request;
//
// #[derive(Default)]
// pub struct ValidUrlProtocolRequest;
//
// impl IValidator<Request> for ValidUrlProtocolRequest {
//     fn execute(&self, value: &mut Request) -> Result<(), String> {
//         let has_the_protocol_in_begin =
//             regex::Regex::new(r"^((http|https)://)(.+)$").map_err(|e| e.to_string())?;
//
//         if !has_the_protocol_in_begin.is_match(&value.url) {
//             let protocol = "http://".to_string();
//             value.url = protocol + &value.url;
//         }
//
//         Ok(())
//     }
// }
