use pyo3::{
    prelude::*,
    pyclass,
    types::{PyCFunction, PyDict, PyFunction, PyTuple},
};

#[pyclass]
struct FlashAPI {}

#[pymethods]
impl FlashAPI {
    #[new]
    fn new() -> Self {
        FlashAPI {}
    }

    fn decorate(&self, py: Python) -> PyResult<&PyCFunction> {
        let f = move |args: &PyTuple, _: Option<&PyDict>| -> PyResult<&PyCFunction> {
            let g = move |args: &PyTuple, kwargs: Option<&PyDict>| {
                // Print when the function is called
            };
            PyCFunction::new_closure(py, None, None, g)
        };
        PyCFunction::new_closure_bound(py, None, None, f)
    }
}

#[pymodule]
fn flashapi(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<FlashAPI>()?;
    Ok(())
}
