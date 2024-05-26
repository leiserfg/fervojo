use railroad::DEFAULT_CSS;
use railroad_dsl::compile;
use wasm_minimal_protocol::*;

initiate_protocol!();

#[wasm_func]
pub fn default_css() -> Vec<u8> {
    DEFAULT_CSS.as_bytes().to_vec()
}


#[wasm_func]
pub fn railroad(src: &[u8], css: &[u8]) -> Result<Vec<u8>, String> {
    let src_str = std::str::from_utf8(src).expect("Missing src");
    let css_str = std::str::from_utf8(css).expect("Missing css");

    match compile(src_str, css_str).map(|it| format!("{}", it.diagram)) {
        Ok(out) => Ok(out.as_bytes().to_vec()),
        Err(e) => Err(format!("{}", e)),
    }
}
