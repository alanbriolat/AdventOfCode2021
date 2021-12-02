use std::collections::BTreeMap;

pub type SolutionResult = crate::Result<String>;

pub trait SolutionFn: Fn() -> SolutionResult {}

impl<F: Fn() -> SolutionResult> SolutionFn for F {}

pub trait Solution {
    fn run(&self) -> SolutionResult;
}

impl<F: SolutionFn> Solution for F {
    fn run(&self) -> SolutionResult {
        self()
    }
}

pub struct Runner {
    solutions: BTreeMap<String, Box<dyn Solution>>,
}

impl Runner {
    pub fn add<K: Into<String>>(&mut self, key: K, solution: Box<dyn Solution>) {
        let key = key.into();
        if self.solutions.contains_key(key.as_str()) {
            panic!("solution {:?} already exists", key);
        }
        self.solutions.insert(key, solution);
    }

    pub fn add_fn<K: Into<String>, S: Solution + 'static>(&mut self, key: K, solution: S) {
        self.add(key, Box::new(solution));
    }

    pub fn merge(&mut self, prefix: &str, other: Runner) {
        for (key, solution) in other.solutions.into_iter() {
            self.add(format!("{}{}", prefix, key), solution);
        }
    }

    pub fn list(&self) -> impl Iterator<Item = &'_ str> {
        self.solutions.keys().map(|k| k.as_str())
    }

    pub fn run(&self, key: &str) -> SolutionResult {
        match self.solutions.get(key) {
            Some(solution) => solution.run(),
            None => panic!("no solution {:?}", key),
        }
    }
}

impl<'a> Default for Runner {
    fn default() -> Self {
        Runner {
            solutions: BTreeMap::default(),
        }
    }
}
