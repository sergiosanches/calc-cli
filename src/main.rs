use std::env::{args, Args};

// simple CLI calculator
// testing iterators, env args, pattern matching, return types

fn main() {
  let mut i_args: Args = args();

  i_args.nth(0); // skip the executable name

  // validate 3 args: num op num
  let (args_num, _) = i_args.size_hint();
  if args_num != 3 {
    println!("Please use 3 arguments: number operator number");
    return;
  } else {
    println!("Calculating...");
  }

  let first_num: f64 = i_args.nth(0).unwrap().parse::<f64>().unwrap();
  let operator: char = i_args.nth(0).unwrap().chars().next().unwrap();
  let second_num: f64 = i_args.nth(0).unwrap().parse::<f64>().unwrap();

  println!("{:?}", operate(operator, first_num, second_num));
}

fn operate(operator: char, first: f64, second: f64) -> f64 {
  match operator {
    '+' => first + second,
    '-' => first - second,
    '/' => first / second,
    '*' | 'x' | 'X' => first * second,
    _ => panic!("Invalid operator!")
  }
}