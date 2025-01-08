use pyo3::{prelude::*, types::PyModule};

use crate::games;

pub fn add_game_modules(py: Python, m: &PyModule, modules: &[(&str, fn(Python, &PyModule) -> PyResult<()>)]) -> PyResult<()> {
    for &(mod_name, init_fn) in modules {
        let module = PyModule::new(py, mod_name)?;
        init_fn(py, module)?;
        m.add_submodule(module)?;
    }
    Ok(())
}

#[pymodule]
fn gamedig(py: Python, m: &PyModule) -> PyResult<()> {
    // Define the game modules and their initialization functions
    let modules: &[(&str, fn(Python, &PyModule) -> PyResult<()>)] = &[
        ("minecraft", games::minecraft::minecraft),
        ("valheim", games::valheim::valheim),
        // Add more modules here
    ];

    // Add the game modules using the function
    add_game_modules(py, m, modules)?;

    Ok(())
}
