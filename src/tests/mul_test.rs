use crate::{circuit, constraint, prove, verify, witness, FieldElement, PRIME};

#[test]
fn multiplication_test() {
    let c = circuit! {
        [a] -> [out],
        constraints = [
            constraint!((w@Witness * a@Input) -> out@Output)
        ]
    };

    let w = witness! {
        w = 4,
        a = 3
    };

    let proof = prove!(c, w);

    let is_ok = verify!(c, [FieldElement::new(3, PRIME)], proof);

    println!("Multiplication proof verified: {}", is_ok);
    assert!(is_ok);
}
