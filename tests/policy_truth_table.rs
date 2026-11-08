#[test]
fn approval_is_conjunctive() {
    for mask in 0..8 {
        let risk = mask & 1 != 0;
        let human = mask & 2 != 0;
        let evaluation = mask & 4 != 0;
        let allow = risk && human && evaluation;
        assert_eq!(allow, mask == 7);
    }
}
