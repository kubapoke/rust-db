use std::cmp::Ordering;
use crate::database::record::Record;
use crate::database::value::{compare_value_intermediate_value, IntermediateValue};
use crate::errors::Error;

pub enum CompOp {
    Eq,
    Neq,
    Leq,
    Lt,
    Geq,
    Gt
}

pub trait Evaluable {
    fn evaluate(&self, r: &Record) -> Result<bool, Error>;
}

pub struct Comparison {
    pub field: String,
    pub op: CompOp,
    pub constant: IntermediateValue,
}

impl Comparison {
    pub fn new(field: String, op: CompOp, constant: IntermediateValue) -> Self {
        Self { field, op, constant }
    }
}

impl Evaluable for Comparison {
    fn evaluate(&self, r: &Record) -> Result<bool, Error> {
        let field_value = r.values.get(&self.field)
            .ok_or_else(|| Error::MissingFieldError(format!("Missing field '{}' during evaluation", self.field)))?;

        let comp_result = compare_value_intermediate_value(field_value, &self.constant)?;

        let result = match self.op {
            CompOp::Eq => comp_result == Ordering::Equal,
            CompOp::Neq => comp_result != Ordering::Equal,
            CompOp::Leq => comp_result != Ordering::Greater,
            CompOp::Lt => comp_result == Ordering::Less,
            CompOp::Geq => comp_result != Ordering::Less,
            CompOp::Gt => comp_result == Ordering::Greater,
        };

        Ok(result)
    }
}

pub struct ComparisonAnd {
    left: Box<AnyEvaluable>,
    right: Box<AnyEvaluable>,
}

impl ComparisonAnd {
    pub fn new(left: AnyEvaluable, right: AnyEvaluable) -> Self {
        Self { left: Box::new(left), right: Box::new(right) }
    }
}

impl Evaluable for ComparisonAnd {
    fn evaluate(&self, r: &Record) -> Result<bool, Error> {
        Ok(self.left.evaluate(r)? && self.right.evaluate(r)?)
    }
}

pub struct ComparisonOr {
    left: Box<AnyEvaluable>,
    right: Box<AnyEvaluable>,
}

impl ComparisonOr {
    pub fn new(left: AnyEvaluable, right: AnyEvaluable) -> Self {
        Self { left: Box::new(left), right: Box::new(right) }
    }
}

impl Evaluable for ComparisonOr {
    fn evaluate(&self, r: &Record) -> Result<bool, Error> {
        Ok(self.left.evaluate(r)? || self.right.evaluate(r)?)
    }
}

pub enum AnyEvaluable {
    Comp(Comparison),
    And(ComparisonAnd),
    Or(ComparisonOr),
}

impl Evaluable for AnyEvaluable {
    fn evaluate(&self, r: &Record) -> Result<bool, Error> {
        match self {
            AnyEvaluable::Comp(c) => c.evaluate(r),
            AnyEvaluable::And(c) => c.evaluate(r),
            AnyEvaluable::Or(c) => c.evaluate(r),
        }
    }
}
