use core::hash::{Hash, Hasher};

use crate::Instance;

impl<T: Hash> Hash for Instance<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}
