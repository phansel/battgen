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

    demo_from_filename(leaf_mod_file, 48, 1);

    println!("~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~");
    println!("making a pack from 200Ah cells....");

    let lfp_200_mod_file = "./examples/cells/lfp_200ah.ron";
    
    demo_from_filename(lfp_200_mod_file, 96, 1);

    let lfp_202_mod_file = "./examples/cells/lfp_202ah.ron";

    demo_from_filename(lfp_202_mod_file, 96, 1);

    let tesla_cell_file = "./examples/cells/tesla_21700.ron";

    demo_from_filename(tesla_cell_file, 96, 10);

/*
    println!("~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~");
    let lfp_75_mod_file = "./examples/cells/lfp_75ah.ron";

    demo_from_filename(lfp_75_mod_file, 96, 1);


    println!("~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~");

    demo_from_filename(lfp_75_mod_file, 96, 2);
*/
}





pub fn demo_from_filename(fname: &str, s: i32, p: i32) {
    
    println!("{} {} {}S{}P", "Generating demo from module given".green(), fname, s, p);
        
    let dmod = read_module(fname);

    dmod.print_mechanical();
    dmod.print_mass();
    dmod.print_electrical_nominal();
    dmod.print_topology();


    let newbat2 = Battery::new_from(dmod, s, p);

    newbat2.print_topology();
    newbat2.print_voltage();
    newbat2.print_ah();

    println!("{} {} kWh", "Nominal pack capacity: ".blue(), newbat2.get_kwh_nominal());

    // this is broken. feel free to fix it.
    println!("{} {} m3", "Leaf pack volume: ".red(), newbat2.get_min_volume_packed());
}
