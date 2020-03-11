use lazy_static::lazy_static;
use rand::{thread_rng, Rng};
use std::collections::HashSet;
use std::iter::repeat_with;
use std::sync::Mutex;

#[derive(Default)]
struct Name(&'static str);

enum Error {
    AlreadyInUse,
}

lazy_static! {
    static ref NAMES_IN_USE: Mutex<HashSet<String>> = Default::default();
}

impl Name {
    fn new(name: String) -> Result<Name, Error> {
        let mut guard = NAMES_IN_USE.lock().unwrap();
        if guard.insert(name.clone()) {
            Ok(Name(unsafe {
                // Safely: this lifetime upcast to 'static is safe, because the static HashSet owns
                // strings, so `&'static str` can't become dangling.
                &*(guard.get(&name).unwrap() as *const String)
            }))
        } else {
            Err(Error::AlreadyInUse)
        }
    }
}

impl Drop for Name {
    fn drop(&mut self) {
        NAMES_IN_USE.lock().unwrap().remove(self.0);
    }
}

#[derive(Default)]
pub struct Robot {
    name: Name,
}

impl Robot {
    pub fn new() -> Self {
        Robot {
            name: Robot::generate_name(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name.0
    }

    pub fn reset_name(&mut self) {
        self.name = Robot::generate_name();
    }

    fn generate_name() -> Name {
        loop {
            if let Ok(name) = Name::new(
                repeat_with(|| thread_rng().gen_range(b'A', b'Z' + 1) as char)
                    .take(2)
                    .chain(repeat_with(|| thread_rng().gen_range(b'0', b'9' + 1) as char).take(3))
                    .collect(),
            ) {
                break name;
            }
        }
    }
}
