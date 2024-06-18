use std::collections::{HashMap, HashSet};

use ruff_diagnostics::{Diagnostic, Violation};
use ruff_macros::{derive_message_formats, violation};
use ruff_python_ast::{comparable::ComparableExpr, BoolOp, CmpOp, Expr, ExprBoolOp, ExprCompare};
use ruff_text_size::Ranged;

use crate::checkers::ast::Checker;

/// ## What it does
/// Checks for comparisons that can be simplified with chaining.
///
/// ## Why is this bad?
/// Multiple comparison expressions can be replaced with a single chained comparison.
/// Simplifying comparisons with chaining can make code more readable and concise.
///
/// ## Example
/// ```python
/// if a < b and b < c:
///     pass
/// ```
///
/// Use instead:
/// ```python
/// if a < b < c:
///     pass
/// ```
///
/// ## References
/// - [Python Documentation: Comparison Expressions](https://docs.python.org/3/reference/expressions.html#comparisons)
#[violation]
pub struct SeparatedComparisons;

impl Violation for SeparatedComparisons {
    #[derive_message_formats]
    fn message(&self) -> String {
        format!("Simplify comparisons with chaining")
    }
}

/// PLR1716
pub(crate) fn separated_comparisons(checker: &mut Checker, expr_bool: &ExprBoolOp) {
    let ExprBoolOp {
        op,
        values,
        range: _,
    } = expr_bool;

    // we only care about 'and' exprs
    if op != &BoolOp::And || values.len() < 2 {
        return;
    }

    // for each variable or constant expr, keep track of which
    // comparison exprs it appears in and whether it was lt or gt
    let mut map: HashMap<ComparableExpr, (HashSet<usize>, HashSet<usize>)> = HashMap::new();

    for (vi, v) in values.iter().enumerate() {
        let Expr::Compare(expr_comp) = v else {
            // we only care about compare exprs
            continue;
        };
        let ExprCompare {
            left,
            ops,
            comparators,
            range: _,
        } = expr_comp;

        let mut left = left.as_ref();
        // iterate through each consecutive pair
        for (op, right) in ops.iter().zip(comparators.iter()) {
            let is_lt = match op {
                CmpOp::Lt | CmpOp::LtE => true,
                CmpOp::Gt | CmpOp::GtE => false,
                _ => continue, // we only care about lt and gt
            };

            for (operand, is_left) in [(left, true), (right, false)] {
                match operand {
                    Expr::Name(_) => {}
                    _ if operand.is_literal_expr() => {}
                    _ => continue, // we only care about variables and constants
                }
                let operand = ComparableExpr::from(operand); // turn it into something hashable

                let (lower, upper) = map.entry(operand).or_default();
                (if is_lt == is_left { lower } else { upper }).insert(vi);
            }

            left = right;
        }
    }

    // iterate through all variable and constant exprs
    for (lower, upper) in map.values() {
        let num_shared = lower.intersection(upper).count();
        let num_lower = lower.len();
        let num_upper = upper.len();

        // there is at least one comparison expr for which it is
        // minimal, and at least one comparison expr for which it
        // is maximal
        if num_shared < num_lower && num_shared < num_upper {
            let diagnostic = Diagnostic::new(SeparatedComparisons, expr_bool.range());
            checker.diagnostics.push(diagnostic);
            break;
        }
    }
}
