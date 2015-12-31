use std::str::FromStr;

type Value = u16;

#[derive(Debug,PartialEq)]
struct Wire(String);

#[derive(Debug,PartialEq)]
enum Signal {
    Value(Value),
    Wire(Wire),
}

#[derive(Debug,PartialEq)]
enum Operation {
    Value(Signal),
    Not(Wire),
    And(Signal, Wire),
    Or(Wire, Wire),
    LShift(Wire, Value),
    RShift(Wire, Value),
}

#[derive(Debug,PartialEq)]
struct Instruction(Operation, Wire);

mod parse {
    extern crate combine;
    use self::combine::*;
    use self::combine::primitives::{Stream, Consumed, Error};
    use std::str::FromStr;

    #[cfg(test)]
    use std::fs::File;

    #[cfg(test)]
    use std::io::{BufRead, BufReader};

    fn wire<I>(input: State<I>) -> ParseResult<::Wire, I> where
        I: Stream<Item=char>,
    {
        many1(lower())
        .map(|id: String| ::Wire(id))
        .parse_state(input)
    }

    fn signal_value<I>(input: State<I>) -> ParseResult<u16, I> where
        I: Stream<Item=char>,
    {
        many1(digit())
        .and_then(|s: String| s.parse())
        .map(|i: u16| i)
        .parse_state(input)
    }

    fn operation<I>(input: State<I>) -> ParseResult<::Operation, I> where
        I: Stream<Item=char>,
    {
        let signal_value = || parser(signal_value)
            .map(|value: ::Value| value);

        let wire = || parser(wire);

        let signal = ||
            try(signal_value().map(|value| ::Signal::Value(value)))
            .or(wire().map(|wire| ::Signal::Wire(wire)));

        let op_signal = signal()
            .map(|signal| ::Operation::Value(signal));

        let op_not = string("NOT ").with(wire())
            .map(|wire| ::Operation::Not(wire));

        let op_and = (
            signal().skip(string(" AND ")),
            wire(),
        ).map(|(a, b)| ::Operation::And(a, b));

        let op_or = (
            wire().skip(string(" OR ")),
            wire(),
        ).map(|(a, b)| ::Operation::Or(a, b));

        let op_lshift = (
            wire().skip(string(" LSHIFT ")),
            signal_value(),
        ).map(|(a, b)| ::Operation::LShift(a, b));

        let op_rshift = (
            wire().skip(string(" RSHIFT ")),
            signal_value(),
        ).map(|(a, b)| ::Operation::RShift(a, b));

        let result = try(op_not).parse_state(input.clone());
        if result.is_ok() { return result; }

        let result = try(op_and).parse_state(input.clone());
        if result.is_ok() { return result; }

        let result = try(op_or).parse_state(input.clone());
        if result.is_ok() { return result; }

        let result = try(op_lshift).parse_state(input.clone());
        if result.is_ok() { return result; }

        let result = try(op_rshift).parse_state(input.clone());
        if result.is_ok() { return result; }

        // all operations start with a signal
        // so match a bare signal last

        let result = try(op_signal).parse_state(input.clone());
        if result.is_ok() { return result; }

        Err(Consumed::Empty(
            ParseError::new(
                input.position.clone(),
                Error::Expected("operation".into()),
            )
        ))
    }

    impl FromStr for ::Instruction {
        type Err = String;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let wire = || parser(wire);

            let operation = || parser(operation);

            let instruction = || (
                operation().skip(string(" -> ")),
                wire(),
            ).map(|(operation, wire)|
                ::Instruction(operation, wire)
            );

            // FIXME -- should not ignore excess input

            instruction()
            .parse(s)
            .map(|(instruction, _)| instruction)
            .map_err(|e| format!("{}", e))
        }
    }

    #[test]
    fn test() {
        // FIXME -- this is not a strong validity test

        let f = File::open("data/day7.txt").unwrap();
        let lines = BufReader::new(f).lines();
        let _: Vec<_> = lines
            .map(Result::unwrap)
            .map(|p| ::Instruction::from_str(&p))
            .map(Result::unwrap)
            .map(|p| println!("{:?}", p))
            .collect();
    }
}

#[test]
fn test() {
    let instructions = vec![
        "1 -> a",
        "NOT a -> b",
        "NOT b -> c",
        "a AND b -> d",
    ];

    let parsed_instructions: Vec<_> = instructions.into_iter()
        .map(|p| ::Instruction::from_str(&p))
        .map(Result::unwrap).collect();

    println!("{:?}", parsed_instructions);

    panic!();
}

fn main() {
}
