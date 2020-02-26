/*
    demonstration; run with "battgen -d" or "battgen --demo"
    shows basic capabilities of tool.

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

pub fn demo() {
    println!("Running demo...");

    let leaf_mod_file = "./examples/cells/leaf_2012.ron";

    println!("~~~~~~~Recreating 2012 Nissan LEAF pack:~~~~~~~~");

    let leafm = read_module(&leaf_mod_file);

    leafm.print_mechanical();
    leafm.print_mass();
    leafm.print_electrical_nominal();
    leafm.print_topology();

    println!("~~~~~~~~~~");

    // leafbat consumes leafm, leafm is no longer publicly accessible after here!
    let leafbat = Battery::new_from(leafm, 48, 1);

    leafbat.print_topology();
    leafbat.print_voltage();
    leafbat.print_ah();

    println!("Nominal pack capacity: {} kWh", leafbat.get_kwh_nominal());

    // this is broken. feel free to fix it.
    println!("Leaf pack volume: {} m3", leafbat.get_min_volume_packed());
}
