use sigmaos_tools_omacalc::Calculator;

fn main() {
    let mut calc = Calculator::new();
    if let Ok(res) = calc.evaluate("10 + 20 * 2") {
        println!("Result: {}", res);
    }
}
