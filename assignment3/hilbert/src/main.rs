use turtle::{Turtle, Drawing, Size};
use std::cmp::{max, min};

//A → -BF+AFA+FB-
// B → +AF-BFB-FA+
#[derive(Debug)]
enum DerivationElement {
    Variable(MoveVariable),
    Token(MoveToken),
}

#[derive(Debug)]
enum MoveVariable {
    A,
    B,
}

#[derive(Debug, Clone)]
enum MoveToken {
    Left,
    Forward,
    Right,
}

impl MoveToken {
    fn execute(&self, turtle: &mut DecoratedTurtle) {
        match (self) {
            Self::Left => turtle.left(),
            Self::Right => turtle.right(),
            Self::Forward => turtle.step()
        }
    }
}

struct DerivationVariableMap {
    A: Vec<DerivationElement>,
    B: Vec<DerivationElement>,
}

impl DerivationVariableMap {
    fn derive(&self, start: &Vec<DerivationElement>, depth: usize) -> Vec<MoveToken> {
        match depth {
             1 => start.iter().filter_map(|e| {
                match e {
                    DerivationElement::Token(t) => Some(t.clone()),
                    DerivationElement::Variable(_) => None
                }
            }).collect(),
            _ => start.iter().flat_map(|e| {
                match e {
                    DerivationElement::Token(t) => vec![t.clone()],
                    DerivationElement::Variable(v) => match v {
                        MoveVariable::A => self.derive(&self.A, depth - 1),
                        MoveVariable::B => self.derive(&self.B, depth - 1),
                    }
                }
            }).collect()
        }
    }
}

fn parse_derivation(rule: &str) -> Vec<DerivationElement> {
    rule.chars().map(|c| {
        match c {
            '-' => DerivationElement::Token(MoveToken::Left),
            '+' => DerivationElement::Token(MoveToken::Right),
            'F' => DerivationElement::Token(MoveToken::Forward),
            'A' => DerivationElement::Variable(MoveVariable::A),
            'B' => DerivationElement::Variable(MoveVariable::B),
            _ => panic!("unknown token char {}", c)
        }
    }).collect()
}

struct DecoratedTurtle {
    turtle:Turtle,
    step_size:f64
}

enum WindowSizeStrategy {
    Fixed(u32),
    Max,
    Min
}
impl WindowSizeStrategy {
    fn find_size(&self,drawing:&Drawing) -> u32 {
        match self {
            Self::Fixed(length)=>length.clone(),
            Self::Max =>max(drawing.size().width, drawing.size().height),
            Self::Min => min(drawing.size().width, drawing.size().height)
        }
    }
}
impl DecoratedTurtle {
    fn create(mut turtle:Turtle,size_strategy:WindowSizeStrategy,steps:usize) -> DecoratedTurtle {
        let size=size_strategy.find_size(turtle.drawing());

        turtle.set_pen_size(1.0);
        turtle.set_pen_color("black");
        turtle.set_speed("fast");

        turtle.drawing_mut().set_size((size,size));

        let step_size=(size as f64)/((steps+1) as f64);
        let start_pos=-0.5*(size as f64)+0.5*step_size;
        turtle.pen_up();
        turtle.go_to((start_pos,start_pos));
        turtle.set_heading(0.0);
        turtle.pen_down();

        DecoratedTurtle {
            turtle,
            step_size
        }
    }

    fn step(&mut self) {
        self.turtle.forward(self.step_size)
    }

    fn left(&mut self) {
        self.turtle.left(90.0);
    }
    fn right(&mut self) {
        self.turtle.right(90.0);
    }
}

const DEPTH:usize=6;

fn main() {
    turtle::start();
    let mut dec_turtle=DecoratedTurtle::create(Turtle::new(),WindowSizeStrategy::Fixed(500),(1<<DEPTH)-1);
    let derivation_variables = DerivationVariableMap {
        A: parse_derivation("-BF+AFA+FB-"),
        B: parse_derivation("+AF-BFB-FA+"),
	};
    for t in derivation_variables.derive(&derivation_variables.A,DEPTH){
        t.execute(&mut dec_turtle);
    }

}
