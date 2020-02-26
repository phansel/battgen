/*
    placeholder for thermal battery design procedures & data structures
    # things to model:
    - total thermal output of battery at peak + derate, average, minimum.
    - heat power loss through enclosure via surface area and bulk conductivity
    - thermal power density of battery and other statistics


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

// coolant properties in the static sense
// if it's not covered here, it's irrelevant to the thermal design.
#[derive(Copy, Clone, Debug)]
pub struct Coolant {
    density: f32,
    specific_heat: f32,
    freezing_point: f32,
    boiling_point: f32,
    flammable: bool,
    conductive: bool,
    // fill in...
}

// allows for mixtures of different chemical coolants,
// e.g. 50% ethylene glycol, 50% water
pub struct CoolantMix {
    coolants: Vec<Coolant>,
    fractions: Vec<f32>,
}

// general properties of a thermal management system
#[derive(Copy, Clone, Debug)]
pub struct ThermalParams {
    coolant: Coolant,
    flow_rate: f32,
    heat_k: f32,
    // fill in...
}
