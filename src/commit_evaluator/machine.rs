use std::collections::HashMap;

use crate::analyzer::{IdentityKind, SelectedExpressions};
use crate::number::AbstractNumberType;

use super::EvalResult;
use super::{affine_expression::AffineExpression, eval_error::EvalError, FixedData};

/// A machine is a set of witness columns and identities where the columns
/// are used on the righ-hand-side of lookups. It can process plookups.
pub trait Machine {
    // /// Tries to construct a new machine with the given subset of
    // /// witness columns and identities. If the identities do not
    // /// fit the pattern of the machine type, it can return None.
    // fn try_new(
    //     fixed_data: &'a FixedData<'a>,
    //     identities: Vec<&'a Identity>,
    //     witness_names: HashSet<&'a str>,
    // ) -> Option<Box<Self>>;

    /// Process a plookup. Not all values on the LHS need to be available.
    /// Can update internal data.
    /// Only return an error if this machine is able to handle the query and
    /// it results in a constraint failure.
    /// If this is not the right machine for the query, return `None`.
    fn process_plookup(
        &mut self,
        fixed_data: &FixedData,
        kind: IdentityKind,
        left: &[Result<AffineExpression, EvalError>],
        right: &SelectedExpressions,
    ) -> Option<EvalResult>;

    /// Returns the final values of the witness columns.
    fn witness_col_values(
        &mut self,
        fixed_data: &FixedData,
    ) -> HashMap<String, Vec<AbstractNumberType>>;
}
