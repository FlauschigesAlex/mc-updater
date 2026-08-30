
pub trait Alias {
    fn get_aliases(&self) -> Vec<&str> {
        return vec![]
    }
    
    fn has_alias(&self, _alias: &str) -> bool {
        return self.get_aliases().iter().any(|alias| alias.to_string() == _alias)
    }
    fn has_alias_ignore_case(&self, _alias: &str) -> bool {
        return self.get_aliases().iter().any(|alias| alias.to_ascii_lowercase() == _alias.to_ascii_lowercase())
    }
}