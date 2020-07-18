/*
    battgen: EV-focused battery generator tooling
    proof of concept; don't use this in production.
    MKS except for Ah

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

extern crate clap;
use clap::{App, Arg, SubCommand};

extern crate serde;
use serde::{Deserialize, Serialize};

extern crate ron;
use ron::value::Value;

use colored::*;

use std::env;
use std::{collections::HashMap, fs::File};

// local imports
mod electrical;
use electrical::*;

mod mechanical;
use mechanical::*;

mod thermal;
use thermal::*;

mod io;
use io::*;

mod demo;
use demo::*;

const PI: f32 = (3.1415926 as f32);

/*
What does bg do? It takes these things:
- Cell characteristics interactively (unimpl) or in some format (.ron) (implemented)
bg --create-cell-interactive
> Cell mass:
> Cell dimensions (m,m,m):
.....
bg --input-cell m50_21700.json
> returns characteristics formatted nicely

bg --input-cell m50_21700.json --topology 96S50P (-t remains unimplemented)
> prints formatted overview and resistances, etc

bg --input-cell m50_21700.json --topology 4S4P --balance-stability
> prints formatted overview and resistances
> prints balance current needed during charge, discharge, storage
    > some chemistries may not need balancing at all
> prints charge/discharge series and parallel stability;
> prints expected lifetime given battery output/input requirements
*/

fn main() {
    let matches = App::new("battgen")
        .version("0.1")
        .author("Paul Hansel <paul.hansel@colorado.edu>")
        .about("EV-focused battery generator tooling")
        .arg(
            Arg::with_name("interactive")
                .short("k")
                .long("interactive")
                .help("Enables interactive cell+pack design mode"),
        )
        .arg(
            Arg::with_name("v")
                .short("v")
                .multiple(true)
                .help("Sets the level of output verbosity"),
        )
        .arg(
            Arg::with_name("input")
                .short("i")
                .long("input-cell")
                .value_name("/path/to/input_file.ron")
                .help("Takes an input cell file")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("topology")
                .short("t")
                .long("topology")
                .value_name("XSYP")
                .help("Takes the desired 2D module topology (in XSYP format)")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("demo")
                .short("d")
                .long("demo")
                .help("Print demonstration pack analysis"),
        )
        .get_matches();

    let default_cell = format!(
        "{}/examples/cells/m50t_21700.ron",
        env!("CARGO_MANIFEST_DIR")
    );

    // check if input cell file provided; if not, use m50_21700 model's filename
    let input_file = matches.value_of("input").unwrap_or(&default_cell);

    // read file into battgen-internal data structures
    let m = read_module(&input_file);

    println!("Using input file: {}", input_file);

    // do they want to generate a new pack? if not, it's still 1S1P (i.e. a lone cell)
    let default_topo = "1S.1P";
    let mut topology = matches.value_of("topology").unwrap_or(default_topo);
    println!("Topology provided: {}", topology); 
    let mut topo_arr: Vec<&str> = topology.split("S").collect();
    let topo_s: &str = topo_arr[0];
    let topo_p: Vec<&str> = topo_arr[1].split("P").collect();

    let topo_sn: i32 = topo_s.parse().unwrap_or(1);
    let topo_pn: i32 = topo_p[0].parse().unwrap_or(1);

    println!("Topology parsed as: {}S {}P", topo_sn, topo_pn);

    // actually print out what the use asked for.
    match matches.occurrences_of("v") {
        0 => m.print_electrical_nominal(),
        1 => {
            m.print_electrical_nominal();
            m.print_overview_ev();
        }
        2 => {
            m.print_overview();
            m.print_overview_ev();
            m.print_mechanical();
            m.print_electrical_nominal();
        }
        // you're the one who typed -vvv.
        3 | _ => {
            m.print_overview();
            m.print_overview_ev();
            m.print_mechanical();
            m.print_electrical_nominal();
            m.print_overview();
            m.print_overview_ev();
            m.print_mechanical();
            m.print_electrical_nominal();
        }
    };

    demo_from_filename(input_file, topo_sn, topo_pn);

    // print out the Leaf demo if user asked for it.
    match matches.occurrences_of("demo") {
        0 => {}
        1 | _ => {
            demo();
        }
    };

    println!("Done.");
}

// user-facing print-out functions in the cmdline
impl Module {
    pub fn print_overview_ev(self) {
        self.print_mechanical();
        self.print_electrical_nominal();
        self.print_mass();
        self.print_topology();
    }

    pub fn print_overview(self) {
        self.print_mechanical();
        self.print_electrical_nominal();
        self.print_mass();
        self.print_topology();
    }

    pub fn print_mechanical(self) {
        println!("Volume of cells in module: {} m3", self.get_volume());
        println!(
            "Volume with packing efficiency: {} m3",
            self.get_min_volume_packed()
        );
    }

    pub fn print_mass(self) {
        println!("Module cell mass: {} kg", self.get_mass_kg());
    }

    pub fn print_topology(self) {
        println!("Module topology: {}S{}P", self.series, self.parallel);
    }

    pub fn print_electrical_nominal(self) {
        println!(
            "Module nominal characteristics: {}V, {}Ah, {}kWh",
            self.get_voltage(),
            self.get_ah(),
            self.get_kwh_nominal()
        );
    }
}
