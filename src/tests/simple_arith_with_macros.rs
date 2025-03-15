use crate::{circuit, constraint, prove, verify, witness, FieldElement, PRIME};

#[test]
fn simple_zk_example() {
    let c = circuit! {
        [x] -> [out],
        constraints = [
            // This is how to mark signals as Witness, Input (public by default), or Output
            constraint!((w@Witness + x@Input) -> out@Output)
        ]
    };

    let w = witness! {
        w = 5,
        x = 10
    };

    let proof = prove!(c, w);

    let is_ok = verify!(c, [FieldElement::new(10, PRIME)], proof);

    println!("Proof verified: {}", is_ok);
    assert!(is_ok);
}
