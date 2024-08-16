# The NonHumankind Simulation

The NonHumankind is a simulation project that models early civilizations, focusing on individual entities and their interactions with the environment. This project is written in Rust and emphasizes strong typing, a robust logging system, and a plugin-based architecture.

## Individual State Representation

The simulation uses a compact string format to represent the state of each individual. The format is as follows:

`ID:{:05};POS:{:04},{:04};HP:{:03};EN:{:03};DNA:{}`

This string encodes key information about an individual:
- `ID`: A unique 5-digit identifier for the individual.
- `POS`: The individual's position on the map, represented by two 4-digit coordinates (X,Y).
- `HP`: The individual's current hit points, represented by a 3-digit number.
- `EN`: The individual's current energy level, represented by a 3-digit number.
- `DNA`: The individual's genetic code, which determines its traits and capabilities.

This format allows for efficient storage and parsing of individual states, facilitating features such as saving/loading simulation states, logging, and inter-process communication. The fixed-width fields (except for DNA) ensure easy parsing and consistent string lengths, while the DNA field allows for variable-length genetic codes to accommodate future expansions of the genetic system.


## Getting Started

### Prerequisites

- Rust programming language (latest stable version)
- Cargo (Rust's package manager)

### Installation

1. Clone the repository:

```sh
git clone https://github.com/yourusername/the_nonhumankind.git
cd the_nonhumankind
```

3. Build the project:
cargo build


### Running the Simulation

To run the basic simulation example:

```sh
cargo run --example basic_simulation
```
