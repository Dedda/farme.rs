use itertools::Itertools;
use regex::Regex;

#[derive(Debug)]
pub struct ValidationError {
    pub messages: Vec<String>,
}

pub trait Validator<T> {
    fn validate(&self, value: T) -> Result<(), ValidationError>;
}

pub trait StringCriteria {
    fn validate(&self, value: &str) -> Result<(), String>;
}

pub struct StringLengthCriteria {
    min: Option<usize>,
    max: Option<usize>,
}

impl StringLengthCriteria {
    #[allow(dead_code)]
    pub fn new(min: usize, max: usize) -> Self {
        Self {
            min: Some(min),
            max: Some(max),
        }
    }

    pub fn min(min: usize) -> Self {
        Self {
            min: Some(min),
            max: None,
        }
    }

    #[allow(dead_code)]
    pub fn max(max: usize) -> Self {
        Self {
            min: None,
            max: Some(max),
        }
    }
}

impl StringCriteria for StringLengthCriteria {
    fn validate(&self, value: &str) -> Result<(), String> {
        let len = value.chars().count();
        if let Some(min) = self.min
            && len < min
        {
            return Err(format!(
                "Expected mininum {min} characters but got only {len}."
            ));
        }
        if let Some(max) = self.max
            && len > max
        {
            return Err(format!("Expected maximum {max} characters but got {len}."));
        }
        Ok(())
    }
}

pub struct RequiredCharacterGroupCriteria {
    chars: Vec<char>,
}

impl RequiredCharacterGroupCriteria {
    pub fn new(chars: Vec<char>) -> Self {
        Self { chars }
    }

    pub fn range(min: char, max: char) -> Self {
        Self {
            chars: (min..=max).collect(),
        }
    }
}

impl StringCriteria for RequiredCharacterGroupCriteria {
    fn validate(&self, value: &str) -> Result<(), String> {
        if self.chars.iter().any(|c| value.contains(&c.to_string())) {
            Ok(())
        } else {
            #[allow(unstable_name_collisions)]
            Err(format!(
                "Expected one of `{}` but got `{}`.",
                self.chars.iter().intersperse(&',').collect::<String>(),
                value
            ))
        }
    }
}

pub struct RegexValidator {
    regex: Regex,
}

impl RegexValidator {
    pub fn new(regex: &str) -> Result<Self, regex::Error> {
        Ok(Self {
            regex: Regex::new(regex)?,
        })
    }
}

impl StringCriteria for RegexValidator {
    fn validate(&self, value: &str) -> Result<(), String> {
        if self.regex.is_match(value) {
            Ok(())
        } else {
            Err(format!("`{value}` does not match expected regex."))
        }
    }
}

pub struct StringValidator {
    criteria: Vec<Box<dyn StringCriteria>>,
}

impl StringValidator {
    pub fn new() -> Self {
        Self {
            criteria: Vec::new(),
        }
    }

    pub fn add_criteria<T: StringCriteria + 'static>(&mut self, criteria: T) {
        self.criteria.push(Box::new(criteria));
    }
}

impl Validator<&str> for StringValidator {
    fn validate(&self, value: &str) -> Result<(), ValidationError> {
        let error_messages: Vec<String> = self
            .criteria
            .iter()
            .map(|c| c.validate(value))
            .filter(|res| res.is_err())
            .map(|res| res.unwrap_err())
            .collect();
        if error_messages.is_empty() {
            Ok(())
        } else {
            Err(ValidationError {
                messages: error_messages,
            })
        }
    }
}

pub struct EmailValidator;

impl Validator<&str> for EmailValidator {
    fn validate(&self, value: &str) -> Result<(), ValidationError> {
        RegexValidator::new(include_str!("email_regex.txt"))
            .expect("Cannot parse email regex")
            .validate(value)
            .map_err(|msg| ValidationError {
                messages: vec![msg],
            })
    }
}

pub struct PasswordValidator;

impl Validator<&str> for PasswordValidator {
    fn validate(&self, value: &str) -> Result<(), ValidationError> {
        let mut validator = StringValidator::new();
        validator.add_criteria(StringLengthCriteria::min(8));
        validator.add_criteria(RequiredCharacterGroupCriteria::range('a', 'z'));
        validator.add_criteria(RequiredCharacterGroupCriteria::range('A', 'Z'));
        validator.add_criteria(RequiredCharacterGroupCriteria::range('0', '9'));
        validator.add_criteria(RequiredCharacterGroupCriteria::new(
            "!?.-_#$&".chars().collect(),
        ));
        validator.validate(value)
    }
}

#[cfg(test)]
mod tests {
    mod strings {
        use crate::validation::{
            RequiredCharacterGroupCriteria, StringCriteria, StringLengthCriteria, StringValidator,
            Validator,
        };

        #[test]
        fn string_length() {
            let min_validator = StringLengthCriteria::min(5);
            assert!(min_validator.validate("12345").is_ok());
            assert!(min_validator.validate("ä€`ñ0").is_ok());
            assert_eq!(
                "Expected mininum 5 characters but got only 4.",
                min_validator.validate("ä€`ñ").unwrap_err()
            );

            let max_validator = StringLengthCriteria::max(5);
            assert!(max_validator.validate("12345").is_ok());
            assert!(max_validator.validate("ä€`ñ0").is_ok());
            assert_eq!(
                "Expected maximum 5 characters but got 6.",
                max_validator.validate("ä€`ñ56").unwrap_err()
            );

            let validator = StringLengthCriteria::new(5, 6);
            assert!(validator.validate("12345").is_ok());
            assert!(validator.validate("ä€`ñ06").is_ok());
            assert_eq!(
                "Expected mininum 5 characters but got only 4.",
                min_validator.validate("ä€`ñ").unwrap_err()
            );
            assert_eq!(
                "Expected maximum 6 characters but got 7.",
                validator.validate("ä€`ñ567").unwrap_err()
            );
        }

        #[test]
        fn required_char_group() {
            let chars = RequiredCharacterGroupCriteria::new("abc".chars().collect());
            assert!(chars.validate("adefg").is_ok());
            assert!(chars.validate("bdefg").is_ok());
            assert!(chars.validate("cdefg").is_ok());
            assert_eq!(
                Err("Expected one of `a,b,c` but got `def`.".to_string()),
                chars.validate("def")
            );

            let chars = RequiredCharacterGroupCriteria::range('a', 'c');
            assert!(chars.validate("adefg").is_ok());
            assert!(chars.validate("bdefg").is_ok());
            assert!(chars.validate("cdefg").is_ok());
            assert_eq!(
                Err("Expected one of `a,b,c` but got `def`.".to_string()),
                chars.validate("def")
            );
        }

        #[test]
        fn validator() {
            let mut validator = StringValidator::new();
            validator.add_criteria(RequiredCharacterGroupCriteria::range('0', '9'));
            validator.add_criteria(RequiredCharacterGroupCriteria::range('A', 'Z'));
            validator.add_criteria(RequiredCharacterGroupCriteria::range('a', 'z'));
            validator.add_criteria(StringLengthCriteria::min(10));
            assert!(validator.validate("abcDEF0189").is_ok());
            assert_eq!(validator.validate(".").unwrap_err().messages.len(), 4);
        }
    }
}
