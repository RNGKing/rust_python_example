use pyo3::prelude::*;
use std::io::Cursor;
use image::io::Reader as ImageReader;
use pyo3::exceptions::PyTypeError;
use bardecoder;
use qrcode_generator::QrCodeEcc;
use std::error::Error;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

#[pyfunction]
fn process_image_from_bytes(bytes: &[u8]) -> PyResult<String> {
    match ImageReader::new(Cursor::new(bytes)).with_guessed_format(){
        Ok(reader) => {
            process_data_from_reader(reader)
        }
        Err(_) => Err(PyErr::new::<PyTypeError, _>("Failed to get data reader"))
    }
}

fn process_data_from_reader(reader: ImageReader<Cursor<&[u8]>>) -> PyResult<String> {
    match reader.decode(){
        Ok(data) => {
            let decoder = bardecoder::default_decoder();
            let results = decoder.decode(&data);
            match results.into_iter().nth(0){
                Some(output_res) => {
                    match output_res {
                        Ok(output) => Ok(output),
                        Err(err) => Err(PyErr::new::<PyTypeError, _>(err.to_string()))
                    }
                },
                None => Ok("NO DATA FOUND".to_string())
            }
            
        },
        Err(_) => Err(PyErr::new::<PyTypeError, _>("Failed to decode image")),
    }
}



#[pyfunction]
fn create_qr_code_from_string(input: String, location: String) -> PyResult<String>{
    match qrcode_generator::to_png_to_file(input, QrCodeEcc::Low, 1024, location)
    {
        Err(err) => Err(PyErr::new::<PyTypeError,_>(err.to_string())),
        Ok(_) => Ok("Success!".to_string())
    }
}

/// A Python module implemented in Rust.
#[pymodule]
fn qr_code_parser(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_function(wrap_pyfunction!(process_image_from_bytes, m)?)?;
    m.add_function(wrap_pyfunction!(create_qr_code_from_string,m)?)?;
    Ok(())
}