pub mod request;
pub mod response;

// pub trait IValidator<T: Clone> {
//     fn execute(&self, value: &mut T) -> Result<(), String>;
// }
//
// pub struct ValidatorsHandler<T> {
//     value: T,
// }
// impl<T: Clone> ValidatorsHandler<T> {
//     fn from(value: T) -> Self {
//         Self { value }
//     }
//     fn execute(&self, itr: Vec<Box<dyn IValidator<T>>>) -> Result<T, String> {
//         let mut fold = self.value.clone();
//
//         for cb in itr.into_iter() {
//             let res = cb.execute(&mut fold);
//             if let Err(e) = res {
//                 return Err(e);
//             }
//         }
//
//         Ok(fold)
//     }
// }
// -------------------------------------------

pub type Validator<T> = fn(app: &mut T) -> Result<(), String>;

pub struct Validators;

pub struct ValidatorsHandler<T> {
    value: T,
}
impl<T: Clone> ValidatorsHandler<T> {
    pub fn from(value: T) -> Self {
        Self { value }
    }
    pub fn execute(&self, itr: Vec<Validator<T>>) -> Result<T, String> {
        let mut result = self.value.clone();

        for validator_fn in itr.into_iter() {
            let res = validator_fn(&mut result);
            if let Err(e) = res {
                return Err(e);
            }
        }

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn validator_to_append_mew() -> Validator<String> {
        |parameter: &mut String| -> Result<(), String> {
            parameter.push_str("mew");
            Ok(())
        }
    }

    fn validator_to_append_two() -> Validator<String> {
        |parameter: &mut String| -> Result<(), String> {
            parameter.push_str("two");
            Ok(())
        }
    }

    fn validator_to_append_space() -> Validator<String> {
        |parameter: &mut String| -> Result<(), String> {
            parameter.push_str(" ");
            Ok(())
        }
    }

    fn validator_to_pop_str() -> Validator<String> {
        |parameter: &mut String| {
            parameter.pop();
            Ok(())
        }
    }

    #[test]
    fn should_execute_a_single_validator() {
        let value = String::from("Mew");

        let t1 = ValidatorsHandler::from(value.clone())
            .execute(vec![validator_to_append_two()])
            .unwrap();

        assert_eq!("Mewtwo", t1.as_str());
    }

    #[test]
    fn should_execute_multiples_validators() {
        let value = String::from("Mew");

        let t1 = ValidatorsHandler::from(value.clone())
            .execute(vec![
                validator_to_append_space(),
                validator_to_append_space(),
            ])
            .unwrap();

        assert_eq!("Mew  ", t1.as_str());
    }

    #[test]
    fn should_execute_multiples_validators_differently() {
        let value = String::from("Mew");

        let t1 = ValidatorsHandler::from(value.clone())
            .execute(vec![
                validator_to_append_two(),
                validator_to_append_space(),
                validator_to_append_mew(),
            ])
            .unwrap();

        assert_eq!("Mewtwo mew", t1.as_str());
    }
}
