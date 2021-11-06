// Copyright 2021 Chiral Ltd.
// Licensed under the Apache-2.0 license (https://opensource.org/licenses/Apache-2.0)
// This file may not be copied, modified, or distributed
// except according to those terms.

mod utils;

use wasm_bindgen::prelude::*;

use graph_symmetry::ext::molecule;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub struct AtomVecWrapper {
    atoms: molecule::AtomVec
}

#[wasm_bindgen]
pub fn smiles_to_atom_vec(smiles: &str) -> AtomVecWrapper {
    AtomVecWrapper { atoms: molecule::smiles_to_atom_vec(smiles) }
}

type Orbits = Vec<Vec<usize>>;

#[inline]
pub fn orbits_to_js_array(orbits: &Orbits) -> js_sys::Array {
    orbits.to_vec().into_iter().map(
        |ob| -> js_sys::Array { ob.into_iter().map(JsValue::from).collect() }
    ).collect() 
}

#[wasm_bindgen]
pub struct GIVPResult {
    orbits: Orbits, 
    numbering: Vec<usize>
}

#[wasm_bindgen]
impl GIVPResult {
    pub fn get_orbits(&self) -> js_sys::Array {
        orbits_to_js_array(&self.orbits)
    }

    pub fn get_numbering(&self) -> Vec<usize> {
        self.numbering.clone()
    }
}

#[wasm_bindgen]
pub fn givp(avw: &AtomVecWrapper) -> GIVPResult {
    let (orbits, numbering) = molecule::symmetry_perception_givp(&avw.atoms);
    GIVPResult { orbits, numbering }
}

#[wasm_bindgen]
pub fn cnap(avw: &AtomVecWrapper, gr: &GIVPResult) -> js_sys::Array {
    orbits_to_js_array(&molecule::symmetry_perception_cnap(&avw.atoms, &gr.orbits, &gr.numbering))
}

#[wasm_bindgen]
pub fn canon_smiles(smiles: &str) -> String {
    molecule::get_canon_smiles(&smiles.to_string())
}
