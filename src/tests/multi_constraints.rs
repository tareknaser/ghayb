use crate::{circuit, constraint, prove, verify, witness, FieldElement, PRIME};

#[test]
fn multiple_constraints_test() {
    let c = circuit! {
        [x, y] -> [out],
        constraints = [
            constraint!((w@Witness + x@Input) -> tmp@Witness),
            constraint!((tmp@Witness * y@Input) -> out@Output)
        ]
    };

    let w = witness! {
        w = 2,
        x = 3,
        y = 4
    };

    let proof = prove!(c, w);

    let is_ok = verify!(
        c,
        [FieldElement::new(3, PRIME), FieldElement::new(4, PRIME)],
        proof
    );

    println!("Multiple constraints proof verified: {}", is_ok);
    assert!(is_ok);
}
