pub mod verification;
pub mod temporal;

pub fn approved_before_execute(approved: bool, execute: bool) -> bool {
    !execute || approved
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn cannot_execute_without_approval() {
        assert!(!approved_before_execute(false, true));
        assert!(approved_before_execute(true, true));
    }
}
