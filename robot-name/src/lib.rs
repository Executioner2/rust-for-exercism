use std::collections::HashSet;
use std::ptr::null_mut;
use std::sync::Mutex;
use rand::Rng;

static mut USED_NAMES: *mut HashSet<String> = null_mut();
static LETTERS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";
static NUMBERS: &[u8] = b"0123456789";
static LOCK: Mutex<()> = Mutex::new(());

pub struct Robot {
    name: String,
}

fn random_name() -> String {
    loop {
        let name: String = (0..2)
            .map(|_| LETTERS[rand::rng().random_range(0..LETTERS.len())] as char)
            .chain((0..3).map(|_| NUMBERS[rand::rng().random_range(0..NUMBERS.len())] as char))
            .collect();
        
        unsafe {
            let _lock = LOCK.lock().unwrap();
            if USED_NAMES.is_null() {
                USED_NAMES = Box::into_raw(Box::new(HashSet::new()));
            }
            if !USED_NAMES.as_mut().unwrap().contains(&name) {
                USED_NAMES.as_mut().unwrap().insert(name.clone());
                return name;
            }
        }
    }
}

fn remove_robot_name(name: &str) {
    unsafe {
        let _lock = LOCK.lock().unwrap();
        if !USED_NAMES.is_null() {
            USED_NAMES.as_mut().unwrap().remove(name);
        }
    }
}

impl Robot {
    pub fn new() -> Self {
        Self {
            name: random_name(),
        }
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    pub fn reset_name(&mut self) {
        remove_robot_name(self.name.as_str());
        self.name = random_name();
    }
}
