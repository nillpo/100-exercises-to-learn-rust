struct Ticket {
    title: String,
    description: String,
    status: String,
}
enum Status {
    ToDo,
    InProgress,
    Done,
}

impl Status {
    fn from_str(status: &str) -> Option<Self> {
        match status {
            "To-Do" => Some(Self::ToDo),
            "In Progress" => Some(Self::InProgress),
            "Done" => Some(Self::Done),
            _ => None,
        }
    }
}

impl Ticket {
    // TODO: implement the `new` function.
    //  The following requirements should be met:
    //   - Only `To-Do`, `In Progress`, and `Done` statuses are allowed.
    //   - The `title` and `description` fields should not be empty.
    //   - the `title` should be at most 50 bytes long.
    //   - the `description` should be at most 500 bytes long.
    //  The method should panic if any of the requirements are not met.
    //
    // You'll have to use what you learned in the previous exercises,
    // as well as some `String` methods. Use the documentation of Rust's standard library
    // to find the most appropriate options -> https://doc.rust-lang.org/std/string/struct.String.html
    fn new(title: String, description: String, status: String) -> Self {
        Self::validate_title(&title);
        Self::validate_description(&description);
        Self::validate_status(&status);
        Self {
            title,
            description,
            status,
        }
    }

    fn validate_status(status: &String) {
        match Status::from_str(status) {
            Some(_) => {}
            None => panic!("Only `To-Do`, `In Progress`, and `Done` statuses are allowed"),
        };
    }

    fn validate_empty(key: &str, str: &String) {
        if str.is_empty() {
            panic!("{} cannot be empty", key);
        }
    }
    fn validate_string_size(key: &str, str: &String, size: usize) {
        if str.as_bytes().len() > size {
            panic!("{} cannot be longer than {} characters", key, size);
        }
    }
    fn validate_title(str: &String) {
        Self::validate_empty("Title", str);
        Self::validate_string_size("Title", str, 50);
    }

    fn validate_description(str: &String) {
        Self::validate_empty("Description", str);
        Self::validate_string_size("Description", str, 500);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::{overly_long_description, overly_long_title, valid_description, valid_title};

    #[test]
    #[should_panic(expected = "Title cannot be empty")]
    fn title_cannot_be_empty() {
        Ticket::new("".into(), valid_description(), "To-Do".into());
    }

    #[test]
    #[should_panic(expected = "Description cannot be empty")]
    fn description_cannot_be_empty() {
        Ticket::new(valid_title(), "".into(), "To-Do".into());
    }

    #[test]
    #[should_panic(expected = "Title cannot be longer than 50 bytes")]
    fn title_cannot_be_longer_than_fifty_chars() {
        Ticket::new(overly_long_title(), valid_description(), "To-Do".into());
    }

    #[test]
    #[should_panic(expected = "Description cannot be longer than 500 bytes")]
    fn description_cannot_be_longer_than_500_chars() {
        Ticket::new(valid_title(), overly_long_description(), "To-Do".into());
    }

    #[test]
    #[should_panic(expected = "Only `To-Do`, `In Progress`, and `Done` statuses are allowed")]
    fn status_must_be_valid() {
        Ticket::new(valid_title(), valid_description(), "Funny".into());
    }

    #[test]
    fn done_is_allowed() {
        Ticket::new(valid_title(), valid_description(), "Done".into());
    }

    #[test]
    fn in_progress_is_allowed() {
        Ticket::new(valid_title(), valid_description(), "In Progress".into());
    }
}
