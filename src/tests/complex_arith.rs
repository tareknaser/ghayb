use crate::{circuit, constraint, prove, verify, witness, FieldElement, PRIME};

#[test]
fn complex_arithmetic_test() {
    let c = circuit! {
        [x] -> [out],
        constraints = [
            constraint!((w1@Witness + w2@Witness) -> tmp1@Witness),
            constraint!((tmp1@Witness * x@Input) -> tmp2@Witness),
            constraint!((tmp2@Witness - w3@Witness) -> out@Output)
        ]
    };

    let w = witness! {
        w1 = 2,
        w2 = 3,
        x = 4,
        w3 = 10
    };

    let proof = prove!(c, w);

    let is_ok = verify!(c, [FieldElement::new(4, PRIME)], proof);

    println!("Complex arithmetic proof verified: {}", is_ok);
    assert!(is_ok);
}
