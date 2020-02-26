# battgen: EV-focused battery generator tooling

Note: this is an experimental project and should not be used without a full understanding of battery design fundamentals (if it's used at all).

# Try it out:

1. [Install rustup and rust-stable].
2. Build the project with "cargo build".
2. Run with "cargo run" or "./target/debug/battgen -demo".

[Install rustup and rust-stable]: https://doc.rust-lang.org/book/ch01-01-installation.html

# Motives
People will eventually get bored of designing batteries; they'll turn the job over to something. Fortunately, the things that matter in battery design are approachably concrete and can be hardcoded.

# What does the future of automated battery design look like?

A guess:

Zeroth step: Modularization
	- Define physical space available for full pack or individual modules
	- Define physical cell types
	- Voltage and capacity requirements
	- Evaluate physical constraints on pack volume/mass
	- Generate selection of possible modules

Second step: Synthesis
	- Generate range of pack possibilities given specific cell types
	- Estimate capacity and lifetime at different discharge rates

Third step: Implementation
	- Spatial packing
        - Cooling channel or plate implementation
        - Mounting bracket and/or injection-molded enclosure synthesis
	- Busbar synthesis - node-based router/solver/relaxer on 2D plane
	- Cell-level fusing addition

3.5th step: CAD implementation
	- Run implementation directions in CAD software via abstraction layer to create rendering

Fourth step: Analysis
	- Thermal characteristics
	- Structural analysis of pressures on cells
	- Calculate fire and failure risk

Fifth step: Revisit second and third step until suitable pack defined

Sixth step: Module housekeeping
	- Implement BMS and create manufacturing automation and assembly procedures
	- Silicon design is out of scope; 

Seventh step: Virtual module assembly
	- Render and calculate overall volumetric efficiency from real CAD
	- Generate arbitrary enclosure wrapper and characterize strength, fire resistance, thermal etc.

Eight step: repetition, but now try to optimize for cost and complexity.


# License
This project is licensed under the GNU Affero General Public License. A text copy of the AGPL can be found here under LICENSE.

