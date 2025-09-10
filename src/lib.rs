/// Represents a tail call that can be either a continuation or a completed result.
pub enum TailCall<T> {
    /// A continuation that holds a deferred computation.
    Continue(Box<dyn Fn() -> TailCall<T>>),
    /// A completed computation with a result.
    Done(T),
}

impl<T> TailCall<T> {
    /// Applies the current step and returns the next `TailCall`.
    ///
    /// # Panics
    /// Panics if called on a completed computation.
    pub fn apply(&self) -> TailCall<T> {
        match self {
            TailCall::Continue(f) => f(),
            TailCall::Done(_) => panic!("Cannot apply on completed TailCall"),
        }
    }

    /// Checks whether the computation is completed.
    pub fn is_completed(&self) -> bool {
        matches!(self, TailCall::Done(_))
    }

    /// Returns the result if the computation is completed.
    pub fn result(&self) -> Option<&T> {
        match self {
            TailCall::Done(val) => Some(val),
            _ => None,
        }
    }

    /// Executes the computation by iterating until a completed result is found.
    pub fn invoke(self) -> T {
        let mut current = self;
        loop {
            match current {
                TailCall::Continue(f) => {
                    current = f();
                }
                TailCall::Done(val) => {
                    return val;
                }
            }
        }
    }
}


/// Represents a tail call that can be either a continuation or a completed result.
/// Uses static dispatch by parameterizing over the function type `F`.
pub enum MutableTailCall<T, F>
where
    F: Fn() -> MutableTailCall<T, F>,
{
    /// A continuation that holds a deferred computation.
    Continue(F),
    /// A completed computation with a result.
    Done(T),
}

impl<T, F> MutableTailCall<T, F>
where
    F: Fn() -> MutableTailCall<T, F>,
{
    /// Applies the current step and returns the next `TailCall`.
    ///
    /// # Panics
    /// Panics if called on a completed computation.
    pub fn apply(self) -> MutableTailCall<T, F> {
        match self {
            MutableTailCall::Continue(f) => f(),
            MutableTailCall::Done(_) => panic!("Cannot apply on completed TailCall"),
        }
    }

    /// Checks whether the computation is completed.
    pub fn is_completed(&self) -> bool {
        matches!(self, MutableTailCall::Done(_))
    }

    /// Returns the result if the computation is completed.
    pub fn result(&self) -> Option<&T> {
        match self {
            MutableTailCall::Done(val) => Some(val),
            _ => None,
        }
    }

    /// Executes the computation by iterating until a completed result is found.
    pub fn invoke(mut self) -> T {
        loop {
            match self {
                MutableTailCall::Continue(f) => {
                    self = f();
                }
                MutableTailCall::Done(val) => {
                    return val;
                }
            }
        }
    }
}