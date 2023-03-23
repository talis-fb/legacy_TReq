pub mod request;
pub mod response;

pub type Validator<T> = fn(app: &mut T) -> Result<(), String>;

pub struct Validators;

pub struct ValidatorsHandler<'a, T> {
    value: &'a T,
}
impl<'a, T: Clone> ValidatorsHandler<'a, T> {
    pub fn from(value: &'a T) -> Self {
        Self { value }
    }

    pub fn execute<I>(&self, itr: I) -> Result<T, String>
    where
        I: IntoIterator<Item = Validator<T>>,
    {
        let mut result = self.value.clone();

        for validator_fn in itr.into_iter() {
            validator_fn(&mut result)?
        }

        Ok(result)
    }

    pub fn execute_ignoring_errors<I>(&self, itr: I) -> T
    where
        I: IntoIterator<Item = Validator<T>>,
    {
        let mut result = self.value.clone();

        for validator_fn in itr.into_iter() {
            if validator_fn(&mut result).is_err() {
                // do nothing
            }
        }

        result
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
            parameter.push(' ');
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

        let t1 = ValidatorsHandler::from(&value)
            .execute([validator_to_append_two()])
            .unwrap();

        assert_eq!("Mewtwo", t1.as_str());
    }

    #[test]
    fn should_execute_multiples_validators() {
        let value = String::from("Mew");

        let t1 = ValidatorsHandler::from(&value)
            .execute([validator_to_append_space(), validator_to_append_space()])
            .unwrap();

        assert_eq!("Mew  ", t1.as_str());
    }

    #[test]
    fn should_execute_multiples_validators_differently() {
        let value = String::from("Mew");

        let t1 = ValidatorsHandler::from(&value)
            .execute([
                validator_to_append_two(),
                validator_to_append_space(),
                validator_to_append_mew(),
            ])
            .unwrap();

        assert_eq!("Mewtwo mew", t1.as_str());
    }

    // Ignoring errors TEST CASES
    fn validator_to_throw_error() -> Validator<String> {
        |parameter: &mut String| Err("".to_string())
    }

    #[test]
    fn should_execute_multiples_validators_differently_with_ignore_errors() {
        let value = String::from("Mew");

        let t1 = ValidatorsHandler::from(&value)
            .execute_ignoring_errors([
                validator_to_append_two(),
                validator_to_append_space(),
                validator_to_throw_error(),
                validator_to_append_mew(),
                validator_to_append_two(),
                validator_to_throw_error(),
            ]);

        assert_eq!("Mewtwo mewtwo", t1.as_str());
    }
}
