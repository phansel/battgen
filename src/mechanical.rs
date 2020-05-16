/*
    placeholder for mechanical battery design procedures & data structures
    # things to model:
    - pressures on cells in middle of pack
    - thermal expansion and contraction
    - basic oscillatory harmonics , e.g. first-order fundamental frequencies

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

impl Module {
    // create rectangular cell. yes, this is way too many parameters.
    // thermal and cycle life characteristics are assumed from cathode chemistry.
    pub fn new_rec_cell(l: f32, w: f32, h: f32, m: f32, r: f32, q: f32, chem: Chem) -> Module {
        let (vmin, vnom, vmax, cycle_life, current_max_C, specific_heat, temp_max, temp_min) =
            defaults_from_chem(&chem);
        return Module {
            shape: Shape::Prism,
            input_type: ModType::Cell,
            chem: chem,
            series: 1,
            parallel: 1,
            dims: [l, w, h],
            mass: m,
            termination: Term::Axial,
            vmin: vmin,
            vmax: vmax,
            vnom: vnom,
            q: q,
            rnom: r,
            max_current_continuous: current_max_C * q,
            specific_heat: specific_heat,
            cycle_life: cycle_life,
            temp_max: temp_max,
            temp_min: temp_min,
        };
    }

    // create new cylindrical cell from basic params and chemistry.
    pub fn new_cyl_cell(diam: f32, l: f32, m: f32, r: f32, v: f32, q: f32, chem: Chem) -> Module {
        let (vmin, vnom, vmax, cycle_life, current_max_C, specific_heat, temp_max, temp_min) =
            defaults_from_chem(&chem);
        return Module {
            shape: Shape::Cylinder,
            input_type: ModType::Cell,
            chem: chem,
            series: 1,
            parallel: 1,
            dims: [diam, l, 0.0],
            mass: m,
            termination: Term::Axial,
            vmin: vmin,
            vmax: vmax,
            vnom: vnom,
            q: q,
            rnom: r,
            max_current_continuous: current_max_C * q,
            specific_heat: specific_heat,
            cycle_life: cycle_life,
            temp_max: temp_max,
            temp_min: temp_min,
        };
    }

    // returns volume in cubic meters
    pub fn get_volume(self) -> f32 {
        let vol: f32 = match self.shape {
            Shape::Cylinder => PI * (self.dims[0] / 2.0 * self.dims[0] / 2.0) * self.dims[1],
            Shape::Prism => self.dims[0] * self.dims[1] * self.dims[2],
            _ => 0.0,
        };
        return vol;
    }

    // returns mass in kilograms.
    pub fn get_mass_kg(self) -> f32 {
        return self.mass;
    }

    // basic cylindrical/prismatic packing parameters.
    pub fn get_min_volume_packed(self) -> f32 {
        let packing_eff: f32 = match self.shape {
            Shape::Cylinder => 0.90,
            Shape::Prism => 0.98,
            Shape::Other => 1.0,
        };
        return (1.0 / packing_eff) * self.get_volume();
    }
}

// Battery is a series of module arrays in series only.
// Module arrays are an nxm array of modules only.
impl Battery {
    pub fn get_min_volume_packed(&self) -> f32 {
        let l = self.module_array.len();
        let mut vol: f32 = 0.0;
        for i in 0..l - 1 {
            println!("self volume: {}", self.module_array[i].module.get_min_volume_packed());
            let vp = self.module_array[i].module.get_min_volume_packed();
            println!("self count: {}", self.module_array[i].get_cell_count());
            let cc = self.module_array[i].get_cell_count() as f32;
            vol += vp * cc;
        }
        return vol;
    }
}

// Mechanical design requirements of the pack as a whole
#[derive(Copy, Clone, Debug)]
pub struct MechanicalParams {
    // linear and angular xyz
    peak_accel: [f32; 6],
    // fill in...
}
