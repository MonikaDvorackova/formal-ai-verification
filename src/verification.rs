/// A deterministic reference policy checker.
/// This is not a replacement for the SMT solver; use it to cross-check fixtures.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Decision {
    pub risk_approved: bool,
    pub human_approved: bool,
    pub evaluation_passed: bool,
}
impl Decision {
    pub fn permitted(self) -> bool {
        self.risk_approved && self.human_approved && self.evaluation_passed
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn exhaustive_boolean_policy() {
        for mask in 0..8 {
            let d = Decision {
                risk_approved: mask & 1 != 0,
                human_approved: mask & 2 != 0,
                evaluation_passed: mask & 4 != 0,
            };
            assert_eq!(d.permitted(), mask == 7);
        }
    }
}
