use crate::CellID::{Compute, Input};
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};
use std::sync::atomic::{AtomicUsize, Ordering};

/// `InputCellID` is a unique identifier for an input cell.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct InputCellID(usize);

/// `ComputeCellID` is a unique identifier for a compute cell.
/// Values of type `InputCellID` and `ComputeCellID` should not be mutually assignable,
/// demonstrated by the following tests:
///
/// ```compile_fail
/// let mut r = react::Reactor::new();
/// let input: react::ComputeCellID = r.create_input(111);
/// ```
///
/// ```compile_fail
/// let mut r = react::Reactor::new();
/// let input = r.create_input(111);
/// let compute: react::InputCellID = r.create_compute(&[react::CellID::Input(input)], |_| 222).unwrap();
/// ```
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct ComputeCellID(usize);

#[derive(Clone, Copy, Debug, PartialEq, Hash, Eq)]
pub struct CallbackID(usize);

static CALLBACKS_COUNT: AtomicUsize = AtomicUsize::new(0);

impl CallbackID {
    fn new() -> Self {
        CallbackID(CALLBACKS_COUNT.fetch_add(1, Ordering::SeqCst))
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CellID {
    Input(InputCellID),
    Compute(ComputeCellID),
}

impl CellID {
    fn common_id(&self) -> usize {
        match self {
            Input(id) => id.0,
            Compute(id) => id.0,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum RemoveCallbackError {
    NonexistentCell,
    NonexistentCallback,
}

type ComputeFunc<'a, T> = Box<dyn Fn(&[T]) -> T + 'a>;
type Callback<'a, T> = Box<dyn Fn(T) -> () + 'a>;

struct ComputeCellInfo<'a, T> {
    deps: Vec<usize>,
    compute_func: ComputeFunc<'a, T>,
    callbacks: HashMap<CallbackID, Callback<'a, T>>,
}

struct CellInfo<'a, T> {
    value: T,
    required_by: Vec<ComputeCellID>,
    compute_cell_info: Option<ComputeCellInfo<'a, T>>,
}

impl<'a, T> CellInfo<'a, T> {
    fn new_input_cell(value: T) -> Self {
        Self {
            value,
            required_by: Default::default(),
            compute_cell_info: None,
        }
    }

    fn new_compute_cell(value: T, compute_func: ComputeFunc<'a, T>, deps: &[CellID]) -> Self {
        Self {
            value,
            required_by: Default::default(),
            compute_cell_info: Some(ComputeCellInfo {
                deps: deps.iter().map(CellID::common_id).collect(),
                compute_func,
                callbacks: Default::default(),
            }),
        }
    }
}

#[derive(Default)]
pub struct Reactor<'a, T> {
    cells: Vec<CellInfo<'a, T>>,
}

impl<'a, T: Copy + PartialEq> Reactor<'a, T> {
    pub fn new() -> Self {
        Self {
            cells: Default::default(),
        }
    }

    // Creates an input cell with the specified initial value, returning its ID.
    pub fn create_input(&mut self, initial: T) -> InputCellID {
        self.cells.push(CellInfo::new_input_cell(initial));
        InputCellID(self.cells.len() - 1)
    }

    fn check_dependencies<'b>(
        &'_ self,
        dependencies: &'b [CellID],
    ) -> Result<&'b [CellID], CellID> {
        if let Some(&missing) = dependencies
            .iter()
            .find(|id| id.common_id() >= self.cells.len())
        {
            Err(missing)
        } else {
            Ok(dependencies)
        }
    }

    // Creates a compute cell with the specified dependencies and compute function.
    // The compute function is expected to take in its arguments in the same order as specified in
    // `dependencies`.
    // You do not need to reject compute functions that expect more arguments than there are
    // dependencies (how would you check for this, anyway?).
    //
    // If any dependency doesn't exist, returns an Err with that nonexistent dependency.
    // (If multiple dependencies do not exist, exactly which one is returned is not defined and
    // will not be tested)
    //
    // Notice that there is no way to *remove* a cell.
    // This means that you may assume, without checking, that if the dependencies exist at creation
    // time they will continue to exist as long as the Reactor exists.

    pub fn create_compute<F: Fn(&[T]) -> T + 'a>(
        &mut self,
        dependencies: &[CellID],
        compute_func: F,
    ) -> Result<ComputeCellID, CellID> {
        let dependencies = self.check_dependencies(dependencies)?;
        let compute_func = Box::new(compute_func) as ComputeFunc<T>;
        let value = compute_func(
            dependencies
                .iter()
                .map(|&c| self.value(c).unwrap())
                .collect::<Vec<_>>()
                .as_ref(),
        );
        let cur_id = ComputeCellID(self.cells.len());
        self.cells.push(CellInfo::new_compute_cell(
            value,
            compute_func,
            dependencies,
        ));
        dependencies
            .iter()
            .for_each(|id| self.cells[id.common_id()].required_by.push(cur_id));
        Ok(cur_id)
    }

    // Retrieves the current value of the cell, or None if the cell does not exist.
    //
    // You may wonder whether it is possible to implement `get(&self, id: CellID) -> Option<&Cell>`
    // and have a `value(&self)` method on `Cell`.
    //
    // It turns out this introduces a significant amount of extra complexity to this exercise.
    // We chose not to cover this here, since this exercise is probably enough work as-is.
    pub fn value(&self, id: CellID) -> Option<T> {
        self.cells.get(id.common_id()).map(|info| info.value)
    }

    // Sets the value of the specified input cell.
    //
    // Returns false if the cell does not exist.
    //
    // Similarly, you may wonder about `get_mut(&mut self, id: CellID) -> Option<&mut Cell>`, with
    // a `set_value(&mut self, new_value: T)` method on `Cell`.
    //
    // As before, that turned out to add too much extra complexity.
    pub fn set_value(&mut self, id: InputCellID, new_value: T) -> bool {
        self.cells
            .get_mut(id.0)
            .map(|info| {
                if info.value != new_value {
                    info.value = new_value;
                    info.required_by.iter().cloned().map(Reverse).collect()
                } else {
                    BinaryHeap::new()
                }
            })
            .map(|required_by| {
                let mut q = required_by;
                while let Some(Reverse(id)) = q.pop() {
                    let new_value = {
                        let ComputeCellInfo {
                            deps, compute_func, ..
                        } = self.cells[id.0].compute_cell_info.as_ref().unwrap();
                        compute_func(
                            deps.iter()
                                .map(|&id| self.cells[id].value)
                                .collect::<Vec<_>>()
                                .as_ref(),
                        )
                    };

                    let CellInfo {
                        value,
                        required_by,
                        compute_cell_info,
                    } = &mut self.cells[id.0];
                    if *value != new_value {
                        *value = new_value;
                        required_by.iter().for_each(|&id| q.push(Reverse(id)));
                        compute_cell_info
                            .as_ref()
                            .unwrap()
                            .callbacks
                            .values()
                            .for_each(|cb| cb(*value));
                    }
                }
            })
            .is_some()
    }

    // Adds a callback to the specified compute cell.
    //
    // Returns the ID of the just-added callback, or None if the cell doesn't exist.
    //
    // Callbacks on input cells will not be tested.
    //
    // The semantics of callbacks (as will be tested):
    // For a single set_value call, each compute cell's callbacks should each be called:
    // * Zero times if the compute cell's value did not change as a result of the set_value call.
    // * Exactly once if the compute cell's value changed as a result of the set_value call.
    //   The value passed to the callback should be the final value of the compute cell after the
    //   set_value call.
    pub fn add_callback<F: Fn(T) -> () + 'a>(
        &mut self,
        id: ComputeCellID,
        callback: F,
    ) -> Option<CallbackID> {
        self.compute_cell_info_mut(id).map(|info| {
            let callback_id = CallbackID::new();
            info.callbacks.insert(callback_id, Box::new(callback) as _);
            callback_id
        })
    }

    fn compute_cell_info_mut(&mut self, id: ComputeCellID) -> Option<&mut ComputeCellInfo<'a, T>> {
        self.cells
            .get_mut(id.0)
            .and_then(|info| info.compute_cell_info.as_mut())
    }

    // Removes the specified callback, using an ID returned from add_callback.
    //
    // Returns an Err if either the cell or callback does not exist.
    //
    // A removed callback should no longer be called.
    pub fn remove_callback(
        &mut self,
        cell: ComputeCellID,
        callback: CallbackID,
    ) -> Result<(), RemoveCallbackError> {
        self.compute_cell_info_mut(cell)
            .ok_or(RemoveCallbackError::NonexistentCell)
            .and_then(|info| {
                info.callbacks
                    .remove(&callback)
                    .ok_or(RemoveCallbackError::NonexistentCallback)
            })
            .and(Ok(()))
    }
}
