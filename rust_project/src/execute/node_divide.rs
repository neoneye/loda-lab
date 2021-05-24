use super::{EvalError, ProgramCache, Node, ProgramState, RegisterIndex, RegisterValue};
use std::collections::HashSet;
use num_bigint::BigInt;
use num_traits::Zero;

enum DivideError {
    InputOutOfRange,
    DivisionByZero,
}

impl From<DivideError> for EvalError {
    fn from(err: DivideError) -> EvalError {
        match err {
            DivideError::InputOutOfRange => EvalError::DivideOutOfRange,
            DivideError::DivisionByZero => EvalError::DivisionByZero
        }
    }
}

fn perform_operation(x: &RegisterValue, y: &RegisterValue) -> Result<RegisterValue,DivideError> {
    let yy: &BigInt = &y.0;
    if yy.bits() >= 32 {
        return Err(DivideError::InputOutOfRange);
    }
    if yy.is_zero() {
        return Err(DivideError::DivisionByZero);
    }
    let xx: &BigInt = &x.0;
    if xx.bits() >= 32 {
        return Err(DivideError::InputOutOfRange);
    }
    Ok(RegisterValue(xx / yy))
}

pub struct NodeDivideRegister {
    target: RegisterIndex,
    source: RegisterIndex,
}

impl NodeDivideRegister {
    pub fn new(target: RegisterIndex, source: RegisterIndex) -> Self {
        Self {
            target: target,
            source: source,
        }
    }
}

impl Node for NodeDivideRegister {
    fn formatted_instruction(&self) -> String {
        format!("div {},{}", self.target, self.source)
    }

    fn eval(&self, state: &mut ProgramState, _cache: &mut ProgramCache) -> Result<(), EvalError> {
        let lhs: &RegisterValue = state.get_register_value_ref(&self.target);
        let rhs: &RegisterValue = state.get_register_value_ref(&self.source);
        let value: RegisterValue = perform_operation(lhs, rhs)?;
        state.set_register_value(self.target.clone(), value);
        Ok(())
    }

    fn accumulate_register_indexes(&self, register_vec: &mut Vec<RegisterIndex>) {
        register_vec.push(self.target.clone());
        register_vec.push(self.source.clone());
    }

    fn live_register_indexes(&self, register_set: &mut HashSet<RegisterIndex>) {
        if register_set.contains(&self.source) {
            register_set.insert(self.target.clone());
        }
    }
}

pub struct NodeDivideConstant {
    target: RegisterIndex,
    source: RegisterValue,
}

impl NodeDivideConstant {
    pub fn new(target: RegisterIndex, source: RegisterValue) -> Self {
        Self {
            target: target,
            source: source,
        }
    }
}

impl Node for NodeDivideConstant {
    fn formatted_instruction(&self) -> String {
        format!("div {},{}", self.target, self.source)
    }

    fn eval(&self, state: &mut ProgramState, _cache: &mut ProgramCache) -> Result<(), EvalError> {
        let lhs: &RegisterValue = state.get_register_value_ref(&self.target);
        let rhs: &RegisterValue = &self.source;
        let value: RegisterValue = perform_operation(lhs, rhs)?;
        state.set_register_value(self.target.clone(), value);
        Ok(())
    }

    fn accumulate_register_indexes(&self, register_vec: &mut Vec<RegisterIndex>) {
        register_vec.push(self.target.clone());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn process(left: i64, right: i64) -> String {
        let result = perform_operation(
            &RegisterValue::from_i64(left),
            &RegisterValue::from_i64(right)
        );
        match result {
            Ok(value) => value.to_string(),
            Err(DivideError::DivisionByZero) => "BOOM-ZERO".to_string(),
            Err(DivideError::InputOutOfRange) => "BOOM-INPUT".to_string()
        }
    }

    #[test]
    fn test_10000() {
        assert_eq!(process(50, 10), "5");
        assert_eq!(process(100, 1), "100");
        assert_eq!(process(10, 2), "5");
        assert_eq!(process(9, 2), "4");
        assert_eq!(process(-1, -1), "1");
        assert_eq!(process(3, -3), "-1");
        assert_eq!(process(-3, 3), "-1");
        assert_eq!(process(-9, 2), "-4");
        assert_eq!(process(-10, 2), "-5");
    }

    #[test]
    fn test_10001_divisionbyzero() {
        assert_eq!(process(100, 0), "BOOM-ZERO");
        assert_eq!(process(-100, 0), "BOOM-ZERO");
    }

    #[test]
    fn test_10002_inputoutofrange() {
        assert_eq!(process(0x7fffffff, 0x7fffffff), "1");
        assert_eq!(process(-0x7fffffff, -0x7fffffff), "1");
        assert_eq!(process(0x80000000, 1), "BOOM-INPUT");
        assert_eq!(process(-0x80000000, 1), "BOOM-INPUT");
        assert_eq!(process(0x80000001, 2), "BOOM-INPUT");
        assert_eq!(process(-0x80000001, 2), "BOOM-INPUT");
        assert_eq!(process(1, 0x7fffffff), "0");
        assert_eq!(process(1, -0x7fffffff), "0");
        assert_eq!(process(1, 0x80000000), "BOOM-INPUT");
        assert_eq!(process(1, -0x80000000), "BOOM-INPUT");
        assert_eq!(process(1, 0x80000001), "BOOM-INPUT");
        assert_eq!(process(1, -0x80000001), "BOOM-INPUT");
    }
}
