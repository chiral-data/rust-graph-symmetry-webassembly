// Copyright 2021 Chiral Ltd.
// Licensed under the Apache-2.0 license (https://opensource.org/licenses/Apache-2.0)
// This file may not be copied, modified, or distributed
// except according to those terms.

//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

extern crate graph_symmetry_wasm_binding;

#[wasm_bindgen_test]
fn test_canon_smiles() {
    let smiles = "COc1ccc(C(=O)C[n+]2c(C)n(Cc3c4c(cc5c3OCC5)OCC4)c3ccccc32)cc1";
    assert_eq!(graph_symmetry_wasm_binding::canon_smiles(smiles), "COc1ccc(cc1)C(=O)C[n+]1c2ccccc2n(Cc2c3CCOc3cc3CCOc23)c1C".to_string());
}

#[wasm_bindgen_test]
fn test_givp() {
    let smiles = "C(C)(C)CCNCCC(C)(C)";
    let avw = graph_symmetry_wasm_binding::smiles_to_atom_vec(smiles);
    let sr = graph_symmetry_wasm_binding::givp(&avw);
    assert_eq!(sr.get_numbering(), vec![11, 4, 4, 8, 6, 9, 6, 8, 11, 4, 4]);
    let orbits_target = vec![vec![0, 8], vec![1, 2, 9, 10], vec![3, 7], vec![4, 6]];
    for i in 0..orbits_target.len() {
        assert_eq!(sr.get_orbits(i), orbits_target[i]);
    }
    assert_eq!(sr.orbits_count(), 4);
}

#[wasm_bindgen_test]
fn test_cnap() {
    let smiles = "C(C)(C)CCNCCC(C)(C)";
    let avw = graph_symmetry_wasm_binding::smiles_to_atom_vec(smiles);
    let sr_givp = graph_symmetry_wasm_binding::givp(&avw);
    let sr_cnap = graph_symmetry_wasm_binding::cnap(&avw, &sr_givp);
    let orbits_target = vec![vec![0, 8], vec![1, 2, 9, 10], vec![3, 7], vec![4, 6]];
    for i in 0..orbits_target.len() {
        assert_eq!(sr_cnap.get_orbits(i), orbits_target[i]);
    }
    assert_eq!(sr_cnap.orbits_count(), 4);
}

