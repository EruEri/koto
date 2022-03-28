use pyo3::{prelude::*, types::PyModule};

pub(crate) fn show_image(url : &str, artist : Option<&str>) -> Option<()>{
    pyo3::prepare_freethreaded_python();
    let code = include_str!("../Python/show_image.py");
    Python::with_gil(|py| {
        let activators = PyModule::from_code(py, code, "show_image", "show_image");
        match activators {
            Err(e) => {print!("Error {:?}", e); None},
            Ok(module) => {
                let show_image = module.getattr("show_image").ok()?;
                show_image.call1((url, artist.unwrap_or("") )).ok().map(|_| ())
            },
            
        }
    })
    //let code = include_str!("/Users/ndiaye/Documents/Lang/Rust/Cargo/exec/koto/Python/show_image.py");

}