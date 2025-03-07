/// Define a constraint for a circuit
///
/// Usage:
/// ```ignore
///   constraint!((left@Role + right@Role) -> out@Role)
/// ```
///
/// - `Role` can be Input, Witness, or Output
#[macro_export]
macro_rules! constraint {
    (( $l:ident @ $roleL:ident + $r:ident @ $roleR:ident ) -> $o:ident @ $roleO:ident) => {
        $crate::Constraint {
            left: $crate::Signal::$roleL(stringify!($l).to_string()),
            right: $crate::Signal::$roleR(stringify!($r).to_string()),
            output: $crate::Signal::$roleO(stringify!($o).to_string()),
            operation: $crate::Operation::Add,
        }
    };
    (( $l:ident @ $roleL:ident - $r:ident @ $roleR:ident ) -> $o:ident @ $roleO:ident) => {
        $crate::Constraint {
            left: $crate::Signal::$roleL(stringify!($l).to_string()),
            right: $crate::Signal::$roleR(stringify!($r).to_string()),
            output: $crate::Signal::$roleO(stringify!($o).to_string()),
            operation: $crate::Operation::Sub,
        }
    };
    (( $l:ident @ $roleL:ident * $r:ident @ $roleR:ident ) -> $o:ident @ $roleO:ident) => {
        $crate::Constraint {
            left: $crate::Signal::$roleL(stringify!($l).to_string()),
            right: $crate::Signal::$roleR(stringify!($r).to_string()),
            output: $crate::Signal::$roleO(stringify!($o).to_string()),
            operation: $crate::Operation::Mul,
        }
    };
    (( $l:ident @ $roleL:ident == $r:ident @ $roleR:ident ) -> $o:ident @ $roleO:ident) => {
        $crate::Constraint {
            left: $crate::Signal::$roleL(stringify!($l).to_string()),
            right: $crate::Signal::$roleR(stringify!($r).to_string()),
            output: $crate::Signal::$roleO(stringify!($o).to_string()),
            operation: $crate::Operation::Eq,
        }
    };
}

/// Define a circuit with inputs, outputs, and constraints
///
/// Usage:
/// ```ignore
///   circuit! {
///       inputs = [x, y],
///       outputs = [out],
///       constraints = [
///           constraint!((w@Witness + x@Input) -> tmp@Witness),
///           constraint!((tmp@Witness * y@Input) -> out@Output)
///       ]
///   }
/// ```
#[macro_export]
macro_rules! circuit {
    (
        inputs = [ $( $inp:ident ),* $(,)? ],
        outputs = [ $( $out:ident ),* $(,)? ],
        constraints = [ $( $c:expr ),* $(,)? ]
    ) => {
        {
            let _in_signals = vec![
                $(
                    $crate::Signal::Input(stringify!($inp).to_string()),
                )*
            ];

            let _out_signals = vec![
                $(
                    $crate::Signal::Output(stringify!($out).to_string()),
                )*
            ];

            let _constraints = vec![
                $(
                    $c,  // Each $c is a constraint from constraint!(...)
                )*
            ];

            $crate::Circuit::new(_in_signals, _out_signals, _constraints)
        }
    };
}

/// Define a witness (private inputs for a circuit)
///
/// Usage:
/// ```ignore
///    witness! { w = 5, x = 10 }
/// ```
#[macro_export]
macro_rules! witness {
    ( $( $name:ident = $val:expr ),* $(,)? ) => {
        {
            let mut _map = std::collections::HashMap::new();
            $(
                _map.insert(
                    stringify!($name).to_string(),
                    $crate::FieldElement::new($val, $crate::PRIME),
                );
            )*
            $crate::Witness::new(_map)
        }
    };
}

/// Generate a proof using a circuit and a witness
///
/// Usage:
/// ```ignore
///   let proof = prove!(circuit_expr, witness_expr);
/// ```
#[macro_export]
macro_rules! prove {
    ( $circ:expr, $wit:expr ) => {
        $crate::Prover::prove(&$circ, &$wit)
    };
}

/// Verify a proof using a circuit and public inputs
///
/// Usage:
/// ```ignore
///   let is_valid = verify!(circuit_expr, [public inputs...], proof_expr);
/// ```
#[macro_export]
macro_rules! verify {
    ( $circ:expr, [ $( $inp:expr ),* $(,)? ], $proof:expr ) => {
        {
            let _pub_inputs_vec = vec![$($inp),*];
            $crate::Verifier::verify(&$circ, &_pub_inputs_vec, &$proof)
        }
    };
}
