use std::cell::RefCell;
use std::collections::{HashMap, HashSet, VecDeque};
use std::rc::Rc;
use std::sync::atomic::{AtomicUsize, Ordering};

static COUNTER: AtomicUsize = AtomicUsize::new(0);
type Callback<'a, T> = Rc<RefCell<dyn FnMut(T) + 'a>>;
type CallbackId = usize;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct InputCellId {
    id: usize,
    reactor_id: usize,
}

impl InputCellId {
    fn new(id: usize, reactor_id: usize) -> Self {
        Self { id, reactor_id }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ComputeCellId {
    id: usize,
    reactor_id: usize,
}

impl ComputeCellId {
    fn new(id: usize, reactor_id: usize) -> Self {
        Self { id, reactor_id }
    }
}

struct ComputeCell<'a, T> {
    value: T,
    cell_ids: Vec<CellId>, // 一旦确定后，就不会再改变了。可以避免循环依赖
    compute_func: fn(&[T]) -> T,
    callbacks: HashMap<CallbackId, Callback<'a, T>>,
    callback_id_counter: AtomicUsize,
}

impl<'a, T> ComputeCell<'a, T> {
    fn new(value: T, cell_ids: Vec<CellId>, compute_func: fn(&[T]) -> T) -> Self {
        Self {
            value,
            cell_ids,
            compute_func,
            callbacks: HashMap::new(),
            callback_id_counter: AtomicUsize::new(0),
        }
    }

    pub fn add_callback(&mut self, callback: impl FnMut(T) + 'a) -> CallbackId {
        let id = self.callback_id_counter.fetch_add(1, Ordering::Relaxed);
        self.callbacks.insert(id, Rc::new(RefCell::new(callback)));
        id
    }

    pub fn remove_callback(
        &mut self,
        callback_id: &CallbackId,
    ) -> Result<Callback<'a, T>, RemoveCallbackError> {
        self.callbacks
            .remove(callback_id)
            .ok_or(RemoveCallbackError::NonexistentCallback)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum CellId {
    Input(InputCellId),
    Compute(ComputeCellId),
}

#[derive(Debug, PartialEq, Eq)]
pub enum RemoveCallbackError {
    NonexistentCell,
    NonexistentCallback,
}

pub struct Reactor<'a, T> {
    id: usize,
    input_cell: Vec<T>,
    compute_cell: Vec<ComputeCell<'a, T>>,
    dependencies: HashMap<CellId, HashSet<ComputeCellId>>,
}

impl<'a, T: Copy + PartialEq + std::hash::Hash + Eq> Reactor<'a, T> {
    pub fn new() -> Self {
        Self {
            id: COUNTER.fetch_add(1, Ordering::Relaxed),
            input_cell: Vec::new(),
            compute_cell: Vec::new(),
            dependencies: HashMap::new(),
        }
    }

    pub fn create_input(&mut self, initial: T) -> InputCellId {
        self.input_cell.push(initial);
        InputCellId::new(self.input_cell.len() - 1, self.id)
    }

    pub fn create_compute(
        &mut self,
        dependencies: &[CellId],
        compute_func: fn(&[T]) -> T,
    ) -> Result<ComputeCellId, CellId> {
        let values: Vec<T> = self.values(dependencies)?;
        let cc = ComputeCell::new(compute_func(&values), dependencies.to_vec(), compute_func);
        self.compute_cell.push(cc);
        let cc_id = ComputeCellId::new(self.compute_cell.len() - 1, self.id);
        for &dep in dependencies {
            self.dependencies.entry(dep).or_default().insert(cc_id);
        }
        Ok(cc_id)
    }

    fn values(&self, cell_ids: &[CellId]) -> Result<Vec<T>, CellId> {
        cell_ids
            .iter()
            .map(|&id| self.value(id).ok_or(id))
            .collect()
    }

    pub fn value(&self, id: CellId) -> Option<T> {
        match id {
            CellId::Input(cell) => self.input_cell.get(cell.id).copied(),
            CellId::Compute(cell) => self.compute_cell.get(cell.id).map(|cc| cc.value),
        }
    }

    pub fn set_value(&mut self, id: InputCellId, new_value: T) -> bool {
        if self.id != id.reactor_id || id.id >= self.input_cell.len() {
            return false;
        }

        self.input_cell[id.id] = new_value;

        let mut deque = VecDeque::from([CellId::Input(id)]);
        let mut notify = HashSet::new();

        while let Some(id) = deque.pop_front() {
            if let Some(set) = self.dependencies.get(&id) {
                for &cc_id in set {
                    let values = self.values(&self.compute_cell[cc_id.id].cell_ids).unwrap();
                    let cell = &mut self.compute_cell[cc_id.id];
                    let old_value = cell.value;
                    cell.value = (cell.compute_func)(&values);
                    if old_value != cell.value {
                        notify.insert(cc_id.id);
                        deque.push_back(CellId::Compute(cc_id));
                    }
                }
            }
        }

        for cc_id in notify {
            let cell = self.compute_cell.get_mut(cc_id).unwrap();
            cell.callbacks.iter_mut().for_each(|(_, callback)| {
                (*callback.borrow_mut())(cell.value);
            });
        }

        true
    }

    pub fn add_callback(
        &mut self,
        id: ComputeCellId,
        callback: impl FnMut(T) + 'a,
    ) -> Option<CallbackId> {
        self.compute_cell
            .get_mut(id.id)
            .map(|cc| cc.add_callback(callback))
    }

    pub fn remove_callback(
        &mut self,
        cell: ComputeCellId,
        callback: CallbackId,
    ) -> Result<(), RemoveCallbackError> {
        let cc = self
            .compute_cell
            .get_mut(cell.id)
            .ok_or(RemoveCallbackError::NonexistentCell)?;
        cc.remove_callback(&callback)?;
        Ok(())
    }
}
