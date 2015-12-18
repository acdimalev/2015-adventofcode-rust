use std::io::{self, BufRead};

type Range = (usize, usize);

type LightStateFn = Fn(LightState) -> LightState;

#[derive(Clone, Copy, PartialEq)]
enum LightState {
    Off,
    On,
}

struct Grid {
    lights: [LightState; 1000 * 1000],
}

impl Grid {
    fn new() -> Grid {
        Grid{ lights: [LightState::Off; 1000 * 1000] }
    }
    fn count_on(&self) -> usize {
        self.lights.iter().filter(|&x| *x == LightState::On).count()
    }
    fn section(&mut self, xrange: Range, yrange: Range) -> GridSection {
        GridSection {
            grid: self,
            xrange: xrange,
            yrange: yrange,
        }
    }
}

struct GridSection<'a> {
    grid: &'a mut Grid,
    xrange: Range,
    yrange: Range,
}

trait Light {
    // inner functions produce a ton of overhead
    // avoid dynamic dispatch so this can be optimized out ?

    // fn apply(&mut self, op: &LightStateFn);

    fn apply<F>(&mut self, op: F) where
        F: Fn(LightState) -> LightState;
}

impl<'a> Light for GridSection<'a> {
    fn apply<F>(&mut self, op: F) where
        F: Fn(LightState) -> LightState
    {
        for x in (self.xrange.0)..(self.xrange.1) {
        for y in (self.yrange.0)..(self.yrange.1) {
            let light = &mut self.grid.lights[1000 * y + x];
            *light = op(*light);
        }}
    }
}

fn off(_: LightState) -> LightState {
    LightState::Off
}

fn on(_: LightState) -> LightState {
    LightState::On
}

fn toggle(state: LightState) -> LightState {
    match state {
        LightState::Off => LightState::On,
        LightState::On => LightState::Off,
    }
}

#[test]
fn test_grid() {
    let examples: Vec<(Range, Range, Box<LightStateFn>, usize)> = vec![
        ((0, 1000), (0, 1000), Box::new(on), 1000000),
        ((0, 1000), (0, 1), Box::new(toggle), 1000),
        ((499, 501), (499, 501), Box::new(off), 0),
    ];

    for (xrange, yrange, op, reference) in examples.into_iter() {
        let mut grid = Grid::new();

        grid.section(xrange, yrange).apply(&*op);

        assert_eq!(grid.count_on(), reference);
    }
}

mod parse {
    extern crate combine;
    use self::combine::{
        Parser, ParserExt,
        choice, string, many1, digit, token, value, try
    };

    #[derive(Clone, PartialEq, Debug)]
    pub enum Action {
        TurnOn,
        TurnOff,
        Toggle,
    }

    #[derive(PartialEq, Debug)]
    pub struct Coord(pub usize, pub usize);

    #[derive(PartialEq, Debug)]
    pub struct Instruction {
        pub action: Action,
        pub coord1: Coord,
        pub coord2: Coord,
    }

    pub fn instruction(from_string: &str) -> Instruction {
        let number = || many1(digit())
            .and_then(|s: String| s.parse());

        let coord = || (
            number().skip(token(',')),
            number(),
        ).map(|(x, y)| Coord(x, y));

        let action = || choice([
            try(string("turn on")).with(value(Action::TurnOn)),
            try(string("turn off")).with(value(Action::TurnOff)),
            try(string("toggle")).with(value(Action::Toggle)),
        ]);

        let instruction = || (
            action().skip(token(' ')),
            coord().skip(string(" through ")),
            coord(),
        ).map(|(action, coord1, coord2)| Instruction {
            action: action,
            coord1: coord1,
            coord2: coord2,
        });

        instruction().parse(from_string).unwrap().0
    }

    #[test]
    fn test() {
        let examples = vec![
            (
                "turn on 0,0 through 999,999",
                Instruction {
                    action: Action::TurnOn,
                    coord1: Coord(0, 0),
                    coord2: Coord(999, 999),
                },
            ),
            (
                "toggle 0,0 through 999,0",
                Instruction {
                    action: Action::Toggle,
                    coord1: Coord(0, 0),
                    coord2: Coord(999, 0),
                },
            ),
            (
                "turn off 499,499 through 500,500",
                Instruction {
                    action: Action::TurnOff,
                    coord1: Coord(499, 499),
                    coord2: Coord(500, 500),
                },
            ),
        ];

        for (input, reference) in examples.into_iter() {
            println!("parsing input: {}", input);

            let parsed_instruction = instruction(input);

            assert_eq!(parsed_instruction, reference);
        }
    }
}

fn apply_instruction_to_grid(instruction: parse::Instruction, grid: &mut Grid) {
    let xrange = (instruction.coord1.0, instruction.coord2.0 + 1);
    let yrange = (instruction.coord1.1, instruction.coord2.1 + 1);

    let mut grid_section = grid.section(xrange, yrange);

    match instruction.action {
        parse::Action::TurnOn => grid_section.apply(on),
        parse::Action::TurnOff => grid_section.apply(off),
        parse::Action::Toggle => grid_section.apply(toggle),
    };
}

fn main() {
    let stdin = io::stdin();
    let lines: Result<Vec<_>, _> = stdin.lock().lines().collect();
    let instructions = lines.unwrap().into_iter()
        .map(|line| parse::instruction(&line));

    let mut grid = Grid::new();

    for instruction in instructions {
        apply_instruction_to_grid(instruction, &mut grid);
    }

    println!("{}", grid.count_on());
}

#[test]
fn test() {
    let examples = vec![
        ("turn on 0,0 through 999,999", 1000000),
        ("toggle 0,0 through 999,0", 1000),
        ("turn off 499,499 through 500,500", 0),
    ];

    for (instruction, reference) in examples.into_iter() {
        let mut grid = Grid::new();

        let parsed_instruction = parse::instruction(instruction);

        apply_instruction_to_grid(parsed_instruction, &mut grid);

        assert_eq!(grid.count_on(), reference);
    }
}
