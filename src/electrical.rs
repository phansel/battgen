/*
    electrical design layer.
    primary focus; electrical design isn't optional.

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

const MAX_PARALLEL_SUB: i32 = 25;
const MAX_SERIES_SUB: i32 = 25;

// struct actually used throughout the program
#[derive(Copy, Clone, Debug, Deserialize)]
pub struct Module {
    // these are necessary for basic use
    pub shape: Shape,
    pub input_type: ModType,
    pub chem: Chem,
    pub series: i32,
    pub parallel: i32,
    pub dims: [f32; 3],
    pub mass: f32,
    pub termination: Term,
    pub vmin: f32,
    pub vmax: f32,
    // nominal voltage
    pub vnom: f32,
    // capacity in Ah
    pub q: f32,
    // resistance nominal max ohms
    pub rnom: f32,
    // continuous maximum current in amps according to manufacturer
    pub max_current_continuous: f32,
    // specific heat
    // kJ/kg-K or J/g-k (equal)
    pub specific_heat: f32,
    // to 80% SoH at 1C charge/discharge
    pub cycle_life: f32,
    // KELVIN
    pub temp_max: f32,
    pub temp_min: f32,
}

// A battery is made of arbitrary arrays of modules (only in series for now);
// they could be different shapes or voltages.
// Allows for the design of chemically series-hybrid packs. parallel hybrid not supported yet.
pub struct Battery {
    pub module_array: Vec<ModuleArray>,
    // other thermal, mechanical, electrical characteristics; unimplementedd
    pub e_params: Option<ElectricalParams>,
    pub m_params: Option<MechanicalParams>,
    pub t_params: Option<ThermalParams>,
}

// how many copies of each unique cell type should be present in each pack
// and how each should be arranged.
#[derive(Copy, Clone, Debug)]
pub struct ModuleArray {
    pub module: Module,
    // series,parallel
    pub series: i32,
    pub parallel: i32,
}

// The cell's physical shape; sorry, hexagons aren't supported yet.
// flow batteries aren't supported, obviously.
#[derive(Copy, Clone, Debug, Deserialize)]
pub enum Shape {
    Prism,
    Cylinder,
    Other,
}

// how the cell is terminated, for design (mech) tools
#[derive(Copy, Clone, Debug, Deserialize)]
pub enum Term {
    End,
    Axial,
    Other,
}

// Cell means a single chemical cell. these aren't strictly enforced (yet).
#[derive(Copy, Clone, Debug, Deserialize)]
pub enum ModType {
    Cell,
    Module,
}

// Actual chemistry of cell; these are arbitrarily chosen and not guaranteed
// to represent anything in the real world.
#[derive(Copy, Clone, Debug, Deserialize)]
pub enum Chem {
    NMC,
    LFP,
    LMO,
    NCA,
    LTO,
    NiMH,
    Other,
}

// the main cell/module data structure. gigafragile against addition/removal of fields.
impl Module {
    // returns pack voltage in volts
    pub fn get_voltage(self) -> f32 {
        return self.vnom;
    }

    // returns pack charge capacity in Ah
    pub fn get_ah(self) -> f32 {
        return self.q;
    }

    // returns pack energy capacity in kWh
    pub fn get_kwh_nominal(self) -> f32 {
        return self.get_ah() * self.get_voltage() / 1000_f32;
    }

    // returns cell count of module (it might be 4 cells welded together)
    pub fn get_cell_count(self) -> i32 {
        return self.parallel * self.series;
    }

    // such math
    pub fn print_possible_submodules(self) {
        println!("Pack is {}S{}P", self.series, self.parallel);
        println!("Modules in series for easier alignment/placement:");
        for x in 1..MAX_SERIES_SUB {
            if ((self.series % x) == 0) {
                println!(
                    "{} modules of {}S ({}/{}V) possible",
                    x,
                    self.series / x,
                    (self.series / x) as f32 * 3.7,
                    (self.series / x) as f32 * 4.1
                );
            }
        }
        println!("Modules in parallel for future expandability:");
        for x in 1..MAX_PARALLEL_SUB {
            if ((self.parallel % x) == 0) {
                println!(
                    "{} modules of {}P in parallel possible",
                    x,
                    self.parallel / x
                );
            }
        }
    }

    // returns internal resistance of this unit as a function of state of charge.
    // arguments: soc f32 between {0,1}
    // formula is somewhat arbitrary right now, but looks approximately right.
    // parameters from measured data (fitted to some basis function) would be better.
    pub fn get_ir_dc(&self, soc: &f32) -> f32 {
        let r = 1.0 / (4.5 * (0.1 * soc + 0.2)) * self.rnom;
        return r;
    }
}

// module array is a 2D network of a single Module.
impl ModuleArray {
    pub fn new(m: Module, s: i32, p: i32) -> ModuleArray {
        ModuleArray {
            module: m,
            series: s,
            parallel: p,
        }
    }

    // get topology of full pack down to cell-level. probably broken, somehow.
    pub fn get_topology(&self) -> (i32, i32) {
        let s = self.series * self.module.series;
        let p = self.parallel * self.module.parallel;
        return (s, p);
    }

    // this isn't exactly user-facing, but it shouldn't be here.
    pub fn print_topology(&self) {
        let a = self.get_topology();
        println!("ModuleArray topology: {}S{}P", a.0, a.1);
    }

    // returns pack voltage in V
    pub fn get_voltage(&self) -> f32 {
        return self.module.vnom * self.series as f32;
    }

    // returns pack charge capacity in Ah
    pub fn get_ah(&self) -> f32 {
        return self.module.q * self.parallel as f32;
    }

    // returns pack energy capacity in kWh
    pub fn get_kwh_nominal(&self) -> f32 {
        return self.get_ah() * self.get_voltage() / 1000_f32;
    }

    pub fn get_module_count(&self) -> i32 {
        let a = self.series;
        let b = self.parallel;
        return a * b;
    }

    pub fn get_cell_count(&self) -> i32 {
        let a = self.get_topology();
        return a.0 * a.1;
    }

    pub fn get_ir_dc(&self, soc: &f32) -> f32 {
        let r = self.module.get_ir_dc(soc) * (self.series / self.parallel) as f32;
        return r;
    }
}

// most electric vehicle batteries can be modeled by this without any hacks.
impl Battery {
    pub fn new() -> Battery {
        let mut ma: Vec<ModuleArray> = Vec::new();
        Battery {
            module_array: ma,
            e_params: None,
            m_params: None,
            t_params: None,
        }
    }

    pub fn new_from(m: Module, s: i32, p: i32) -> Battery {
        let mut mav: Vec<ModuleArray> = Vec::new();
        let mut ma = ModuleArray::new(m, s, p);
        mav.push(ma);
        Battery {
            module_array: mav,
            e_params: None,
            m_params: None,
            t_params: None,
        }
    }

    // get topology of full pack down to cell-level
    // returns minimum P value; if it's (5S3P)(5S2P), you'll get 10S2P.
    // probably broken.
    pub fn get_topology(&self) -> (i32, i32) {
        let (mut s, mut p) = (0, self.module_array[0].parallel);
        let l = self.module_array.len();
        for i in 0..l {
            let topo_i = self.module_array[i].get_topology();
            s += topo_i.0;
            if topo_i.1 <= p {
                p = topo_i.1;
            }
        }
        return (s, p);
    }

    pub fn print_topology(&self) {
        let a = self.get_topology();
        println!("{} {}S{}P", "Battery minimum topology: ".purple(), a.0, a.1);
        println!("Note: this does not account for cell voltages.");
    }

    // returns pack voltage in V
    pub fn get_voltage(&self) -> f32 {
        let mut v: f32 = 0.0;
        let l = self.module_array.len();
        for i in 0..l {
            v += self.module_array[i].get_voltage();
        }
        return v;
    }

    pub fn print_voltage(&self) {
        println!("Pack voltage: {}V", self.get_voltage());
    }

    // returns minimum pack charge capacity in Ah
    pub fn get_ah(&self) -> f32 {
        let mut q: f32 = 10000000000.0;
        let l = self.module_array.len();
        for i in 0..l {
            let qi = self.module_array[i].get_ah();
            if qi < q {
                q = qi;
            }
        }
        return q;
    }

    pub fn print_ah(&self) {
        println!("Pack capacity: {}Ah", self.get_ah());
    }

    // returns pack minimum accessible energy capacity in kWh
    pub fn get_kwh_nominal(&self) -> f32 {
        return self.get_ah() * self.get_voltage() / 1000_f32;
    }

    pub fn get_module_count(&self) -> i32 {
        let a = self.get_topology();
        let b = self.module_array[0].get_topology();
        let x = a.0 / b.0;
        let y = a.1 / b.1;
        return x * y;
    }

    pub fn get_cell_count(&self) -> i32 {
        let a = self.get_topology();
        return a.0 * a.1;
    }

    pub fn get_ir_dc(&self, soc: &f32) -> f32 {
        let mut r: f32 = 0.0;
        for i in 0..self.module_array.len() {
            r += self.module_array[i].get_ir_dc(soc);
        }
        return r;
    }
}

// returns vmin, vmax, vnom for a given chemistry.
// hardcoding this table is a stopgap solution.
pub fn defaults_from_chem(chem: &Chem) -> (f32, f32, f32, f32, f32, f32, f32, f32) {
    let (vmin, vnom, vmax, cycle_life) = match chem {
        Chem::NMC => (3.0, 3.7, 4.2, 700.0),
        Chem::NCA => (3.0, 3.7, 4.3, 1000.0),
        Chem::LFP => (3.0, 3.2, 3.6, 2000.0),
        Chem::LMO => (3.2, 3.8, 4.2, 700.0),
        Chem::LTO => (1.8, 2.3, 2.8, 5000.0),
        Chem::NiMH => (1.0, 1.2, 1.3, 1000.0),
        Chem::Other => (0.0, 0.0, 0.0, 0.0),
    };
    // assume 2C maximum (dis)charge rate
    let current_max_C = 2.0;
    let (specific_heat, temp_max, temp_min) = (800.0, -20.0, 60.0);
    return (
        vmin,
        vnom,
        vmax,
        cycle_life,
        current_max_C,
        specific_heat,
        temp_max,
        temp_min,
    );
}

// Electrical parameters of the pack as a whole
// unimplemented.
#[derive(Copy, Clone, Debug)]
pub struct ElectricalParams {
    // peak input or output current (A). compare to pack discharge expectations?
    // most packs can deliver more than their officially rated current...
    // at some chemistry-dependent cost to their cycle life.
    peak_current: f32,
    // state of health
    soh: f32,
}
