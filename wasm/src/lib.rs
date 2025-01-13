use chip8_core::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct EmuWasm {
    chip8: Emu,
}

#[wasm_bindgen]
impl EmuWasm {
    #[wasm_bindgen(constructor)]
    pub fn new() -> EmuWasm {
        EmuWasm {
        chip8: Emu::new(),
        }
    }
}