/*
    i/o abstraction layer for reading human-readable cell parameter files
    unimplemented: writing or exporting reports or 3D models (i.e. via openscad)

    Copyright (C) 2020 Paul Hansel

    This program is free software: you can redistribute it and/or modify
    it under the terms of the GNU Affero General Public License as
    published by the Free Software Foundation, either version 3 of the
    License, or (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU Affero General Public License for more details.

    You should have received a copy of the GNU Affero General Public License
    along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/

use crate::*;

extern crate ron;
use ron::de::from_reader;
use std::fs::File;

// struct purely for reading input RON files
#[derive(Clone, Debug, Deserialize)]
pub struct InputModule {
    // these are necessary for basic use
    shape: String,
    input_type: String,
    chem: String,
    series: i32,
    parallel: i32,
    dims: [f32; 3],
    mass: f32,
    termination: String,
    vmin: f32,
    vmax: f32,
    // nominal voltage
    vnom: f32,
    // capacity in Ah
    q: f32,
    // resistance nominal max
    rnom: f32,
    // continuous maximum current according to manufacturer
    max_current_continuous: f32,
    // specific heat
    // kJ/kg-K or J/g-k (equal)
    specific_heat: f32,
    // to 80% SoH at 1C charge/discharge
    cycle_life: f32,
    // KELVIN
    temp_max: f32,
    temp_min: f32,
}

// call these with io::read_module(bar) etc.
pub fn read_module(filename: &str) -> Module {
    let f = File::open(filename).expect("Failed to open file.");
    let input: InputModule = match from_reader(f) {
        Ok(x) => x,
        Err(e) => {
            println!("Failed to load input file: {}", e);
            std::process::exit(1);
        }
    };
    return io::input_module_to_module(input);
}

// accepts and destroys input module to create full module
fn input_module_to_module(m: InputModule) -> Module {
    return Module {
        shape: match m.shape.as_str() {
            "cylinder" => Shape::Cylinder,
            "prism" => Shape::Prism,
            _ => Shape::Other,
        },
        input_type: match m.input_type.as_str() {
            "cell" => ModType::Cell,
            "module" => ModType::Module,
            _ => ModType::Module,
        },
        chem: match m.chem.to_lowercase().as_str() {
            "lmo" => Chem::LMO,
            "nmc" => Chem::NMC,
            "nca" => Chem::NCA,
            "lfp" | "LiFePO4" => Chem::LFP,
            _ => Chem::Other,
        },
        series: m.series,
        parallel: m.parallel,
        dims: m.dims,
        mass: m.mass,
        termination: match m.termination.as_str() {
            "axial" => Term::Axial,
            "end" => Term::End,
            _ => Term::Other,
        },
        vmin: m.vmin,
        vmax: m.vmax,
        vnom: m.vnom,
        q: m.q,
        rnom: m.rnom,
        max_current_continuous: m.max_current_continuous,
        specific_heat: m.specific_heat,
        cycle_life: m.cycle_life,
        temp_max: m.temp_max,
        temp_min: m.temp_min,
    };
}

pub fn write_module(module: &InputModule, filename: &str) {
    // let val = Value::from_str(module).expect("Failed to deserialize");
    // let mut ser = serde_json::Serializer::pretty(std::)
}
