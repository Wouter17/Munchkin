//! Defines the constraints that Pumpkin provides out of the box which can be added to the
//! [`Solver`].
//!
//! A constraint is a relation over variables. In the solver, constraints are enforced through
//! propagators, and therefore constraints can be viewed as a collection of propagators.
//!
//! # Example
//! ```
//! # use pumpkin_lib::constraints;
//! # use pumpkin_lib::Solver;
//! let mut solver = Solver::default();
//!
//! let a = solver.new_bounded_integer(0, 3);
//! let b = solver.new_bounded_integer(0, 3);
//!
//! solver.add_constraint(constraints::equals([a, b], 0)).post();
//! ```
//!
//! # Note
//! At the moment, the API for posting propagators is not yet publicly accessible as it is
//! unstable. Consumers of the Pumpkin library can therefore only define constraints by
//! decomposing them into the constraints that are predefined in the library. Once the
//! propagator API is stabilized, it will become part of the public API.

mod all_different;
mod arithmetic;
mod boolean;
mod circuit;
mod clause;
mod constraint_poster;
mod cumulative;
mod element;

pub use all_different::*;
pub use arithmetic::*;
pub use boolean::*;
pub use circuit::*;
pub use clause::*;
pub use constraint_poster::*;
pub use cumulative::*;
pub use element::*;

use crate::engine::propagation::Propagator;
use crate::propagators::ReifiedPropagator;
use crate::variables::Literal;
use crate::ConstraintOperationError;
use crate::Solver;

/// A [`Constraint`] is a relation over variables. It disqualifies certain partial assignments of
/// making it into a solution of the problem.
///
/// For example, the constraint `a = b` over two variables `a` and `b` only allows assignments to
/// `a` and `b` of the same value, and rejects any assignment where `a` and `b` differ.
pub trait Constraint {
    #[cfg(test)]
    fn post_test(
        self,
        solver: &mut crate::engine::test_helper::TestSolver,
    ) -> Result<Box<dyn Propagator>, crate::basic_types::Inconsistency>;

    /// Add the [`Constraint`] to the [`Solver`].
    ///
    /// This method returns a [`ConstraintOperationError`] if the addition of the [`Constraint`] led
    /// to a root-level conflict.
    fn post(self, solver: &mut Solver) -> Result<(), ConstraintOperationError>;

    /// Add the half-reified version of the [`Constraint`] to the [`Solver`]; i.e. post the
    /// constraint `r -> constraint` where `r` is a reification literal.
    ///
    /// This method returns a [`ConstraintOperationError`] if the addition of the [`Constraint`] led
    /// to a root-level conflict.
    fn implied_by(
        self,
        solver: &mut Solver,
        reification_literal: Literal,
    ) -> Result<(), ConstraintOperationError>;
}

impl<ConcretePropagator> Constraint for ConcretePropagator
where
    ConcretePropagator: Propagator + 'static,
{
    fn post(self, solver: &mut Solver) -> Result<(), ConstraintOperationError> {
        solver.add_propagator(self)
    }

    fn implied_by(
        self,
        solver: &mut Solver,
        reification_literal: Literal,
    ) -> Result<(), ConstraintOperationError> {
        solver.add_propagator(ReifiedPropagator::new(self, reification_literal))
    }

    #[cfg(test)]
    fn post_test(
        self,
        solver: &mut crate::engine::test_helper::TestSolver,
    ) -> Result<Box<dyn Propagator>, crate::basic_types::Inconsistency> {
        solver.new_propagator(self)
    }
}

impl<C: Constraint> Constraint for Vec<C> {
    fn post(self, solver: &mut Solver) -> Result<(), ConstraintOperationError> {
        self.into_iter().try_for_each(|c| c.post(solver))
    }

    fn implied_by(
        self,
        solver: &mut Solver,
        reification_literal: Literal,
    ) -> Result<(), ConstraintOperationError> {
        self.into_iter()
            .try_for_each(|c| c.implied_by(solver, reification_literal))
    }

    #[cfg(test)]
    fn post_test(
        self,
        _solver: &mut crate::engine::test_helper::TestSolver,
    ) -> Result<Box<dyn Propagator>, crate::basic_types::Inconsistency> {
        unimplemented!()
    }
}

/// A [`Constraint`] which has a well-defined negation.
///
/// Having a negation means the [`Constraint`] can be fully reified; i.e., a constraint `C` can be
/// turned into `r <-> C` where `r` is a reification literal.
///
/// For example, the negation of the [`Constraint`] `a = b` is (well-)defined as `a != b`.
pub trait NegatableConstraint: Constraint {
    type NegatedConstraint: NegatableConstraint + 'static;

    fn negation(&self) -> Self::NegatedConstraint;

    /// Add the reified version of the [`Constraint`] to the [`Solver`]; i.e. post the constraint
    /// `r <-> constraint` where `r` is a reification literal.
    ///
    /// This method returns a [`ConstraintOperationError`] if the addition of the [`Constraint`] led
    /// to a root-level conflict.
    fn reify(
        self,
        solver: &mut Solver,
        reification_literal: Literal,
    ) -> Result<(), ConstraintOperationError>
    where
        Self: Sized,
    {
        let negation = self.negation();

        self.implied_by(solver, reification_literal)?;
        negation.implied_by(solver, !reification_literal)
    }
}

/// This implementation is only so that we can return a `todo!` from functions which require returning an `impl Constraint`
impl Constraint for () {
    fn post(self, solver: &mut Solver) -> Result<(), ConstraintOperationError> {
        todo!()
    }

    fn implied_by(
        self,
        solver: &mut Solver,
        reification_literal: Literal,
    ) -> Result<(), ConstraintOperationError> {
        todo!()
    }
    
    #[cfg(test)]
    fn post_test(
        self,
        solver: &mut crate::engine::test_helper::TestSolver,
    ) -> Result<Box<dyn Propagator>, crate::basic_types::Inconsistency> {
        todo!()
    }
}
