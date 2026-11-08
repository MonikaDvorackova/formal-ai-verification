//! Safety monitor for ordered governance events.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Event { EvaluationPassed, RiskApproved, HumanApproved, Deploy, Reset }
#[derive(Debug, Default, Clone, Copy)]
pub struct Monitor { evaluation: bool, risk: bool, human: bool }
impl Monitor {
    pub fn apply(&mut self, event: Event) -> Result<(), &'static str> {
        match event {
            Event::Reset => *self = Self::default(),
            Event::EvaluationPassed => self.evaluation = true,
            Event::RiskApproved => self.risk = true,
            Event::HumanApproved => self.human = true,
            Event::Deploy if !(self.evaluation && self.risk && self.human) =>
                return Err("deployment blocked: missing approval"),
            Event::Deploy => (),
        }
        Ok(())
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn deployment_requires_all_gates_and_reset_revokes() {
        let mut m=Monitor::default();
        assert!(m.apply(Event::Deploy).is_err());
        for e in [Event::EvaluationPassed, Event::RiskApproved, Event::HumanApproved] { m.apply(e).unwrap(); }
        assert!(m.apply(Event::Deploy).is_ok());
        m.apply(Event::Reset).unwrap();
        assert!(m.apply(Event::Deploy).is_err());
    }
}
