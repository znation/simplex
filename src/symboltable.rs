pub struct SymbolTable {

}

impl SymbolTable {
    pub fn new() -> SymbolTable {
        return SymbolTable {}
    }    
}

impl Default for SymbolTable {
    fn default() -> Self {
        Self::new()
    }
}
