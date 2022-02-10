use std::collections::HashMap;

use crate::structure::Structure;

pub struct Stdlib {}
impl Stdlib {
    pub fn symbols() -> HashMap<String, Structure> {
        return HashMap::new();
    }
}
