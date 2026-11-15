use sigmaos_tools_omacalc::Calculator;

#[test]
fn test_basic_arithmetic_aaa() {
    // Arrange
    let mut calc = Calculator::new();
    // Act
    let res = calc.evaluate("10 + 5").unwrap();
    // Assert
    assert_eq!(res, 15.0);
}

#[test]
fn test_division_by_zero_aaa() {
    // Arrange
    let mut calc = Calculator::new();
    // Act
    let res = calc.evaluate("10 / 0");
    // Assert
    assert!(res.is_err());
}

#[test]
fn test_history_tracking_aaa() {
    // Arrange
    let mut calc = Calculator::new();
    // Act
    calc.evaluate("2 * 3").unwrap();
    calc.evaluate("4 + 4").unwrap();
    // Assert
    assert_eq!(calc.history_len(), 2);
    assert_eq!(calc.last_result(), Some(8.0));
}
