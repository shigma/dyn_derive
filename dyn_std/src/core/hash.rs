use core::hash::{Hash, Hasher};

use crate::Instance;

impl<T: Hash, U> Hash for Instance<T, U> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}
