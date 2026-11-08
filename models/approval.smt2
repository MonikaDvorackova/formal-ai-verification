(set-logic QF_BOOL)
(declare-const approved Bool)
(declare-const execute Bool)
(assert execute)
(assert (not approved))
; Counterexample to policy: execution without approval is satisfiable.
(check-sat)
(get-model)
