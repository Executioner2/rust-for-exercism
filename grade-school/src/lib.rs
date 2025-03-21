use std::collections::{BTreeMap, HashSet};

pub struct School {
    roster: BTreeMap<u32, Vec<String>>,
    students: HashSet<String>,
}

impl School {
    pub fn new() -> School {
        Self {
            roster: BTreeMap::new(),
            students: HashSet::new(),
        }
    }

    /// Add {student} to the roster for {grade}
    pub fn add(&mut self, grade: u32, student: &str) {
        if self.students.contains(student) {
            return;
        }
        self.students.insert(student.to_string());
        self.roster
            .entry(grade)
            .or_default()
            .push(student.to_string());
        self.roster.get_mut(&grade).unwrap().sort_unstable();
    }

    pub fn grades(&self) -> Vec<u32> {
        self.roster.keys().cloned().collect()
    }

    // If `grade` returned a reference, `School` would be forced to keep a `Vec<String>`
    // internally to lend out. By returning an owned vector of owned `String`s instead,
    // the internal structure can be completely arbitrary. The tradeoff is that some data
    // must be copied each time `grade` is called.
    pub fn grade(&self, grade: u32) -> &[String] {
        self.roster.get(&grade).map_or(&[], |v| &v[..])
    }
}
