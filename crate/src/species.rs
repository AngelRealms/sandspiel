use Cell;
use SandApi;
use Wind;
use EMPTY_CELL;

use std::mem;
use wasm_bindgen::prelude::*;
// use web_sys::console;

fn rand_int(n: i32) -> i32 {
    (js_sys::Math::random() * n as f64) as i32
}

fn rand_dir() -> i32 {
    let i = rand_int(1000);
    (i % 3) - 1
}
fn adjacency_right(dir: (i32, i32)) -> (i32, i32) {
    match dir {
        (0, 1) => (1, 1),
        (1, 1) => (1, 0),
        (1, 0) => (1, -1),
        (1, -1) => (0, -1),
        (0, -1) => (-1, -1),
        (-1, -1) => (-1, 0),
        (-1, 0) => (-1, 1),
        (-1, 1) => (0, 1),
        _ => (0, 0),
    }
}
fn adjacency_left(dir: (i32, i32)) -> (i32, i32) {
    match dir {
        (0, 1) => (-1, 1),
        (1, 1) => (0, 1),
        (1, 0) => (1, 1),
        (1, -1) => (1, 0),
        (0, -1) => (1, -1),
        (-1, -1) => (0, -1),
        (-1, 0) => (-1, -1),
        (-1, 1) => (-1, 0),
        _ => (0, 0),
    }
}
// fn rand_dir_2() -> i32 {
//     let i = rand_int(1000);
//     if (i % 2) == 0 {
//         -1
//     } else {
//         1
//     }
// }
#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Species {
    Empty = 0,
    Wall = 1,
    Sand = 2,
    Water = 3,
    Stone = 13,
    Ice = 9,
    Gas = 4,
    Cloner = 5,
    // Sink = 10,
    Mite = 15,
    Wood = 7,
    Plant = 11,
    Fungus = 18,
    Seed = 19,
    Fire = 6,
    Lava = 8,
    Acid = 12,
    Dust = 14,
    Oil = 16,
    Firework = 17,
}

impl Species {
    pub fn update(&self, cell: Cell, api: SandApi) {
        match self {
            Species::Empty => {}
            Species::Wall => {}
            Species::Sand => update_sand(cell, api),
            Species::Dust => update_dust(cell, api),
            Species::Water => update_water(cell, api),
            Species::Stone => update_stone(cell, api),
            Species::Gas => update_gas(cell, api),
            Species::Cloner => update_cloner(cell, api),
            Species::Firework => update_firework(cell, api),
            Species::Fire => update_fire(cell, api),
            Species::Wood => update_wood(cell, api),
            Species::Lava => update_lava(cell, api),
            Species::Ice => update_ice(cell, api),
            // Species::Snow => update_ice(cell, api),
            //lightning
            // Species::Sink => update_sink(cell, api),
            Species::Plant => update_plant(cell, api),
            Species::Acid => update_acid(cell, api),
            Species::Mite => update_mite(cell, api),
            Species::Oil => update_oil(cell, api),
            Species::Fungus => update_fungus(cell, api),
            Species::Seed => update_seed(cell, api),
        }
    }
}

pub fn update_sand(cell: Cell, mut api: SandApi) {
    let dx = rand_dir();

    let nbr = api.get(0, 1);
    if nbr.species == Species::Empty {
        api.set(0, 0, EMPTY_CELL);
        api.set(0, 1, cell);
    } else if api.get(dx, 1).species == Species::Empty {
        api.set(0, 0, EMPTY_CELL);
        api.set(dx, 1, cell);
    } else if nbr.species == Species::Water
        || nbr.species == Species::Gas
        || nbr.species == Species::Oil
        || nbr.species == Species::Acid
    {
        api.set(0, 0, nbr);
        api.set(0, 1, cell);
    } else {
        api.set(0, 0, cell);
    }
}

pub fn update_dust(cell: Cell, mut api: SandApi) {
    let dx = rand_dir();
    let fluid = api.get_fluid();

    if fluid.pressure > 120 {
        api.set(
            0,
            0,
            Cell {
                species: Species::Fire,
                ra: (150 + (cell.ra / 10)) as u8,
                rb: 0,
                clock: 0,
            },
        );
        api.set_fluid(Wind {
            dx: 0,
            dy: 0,
            pressure: 80,
            density: 5,
        });
        return;
    }

    let nbr = api.get(0, 1);
    if nbr.species == Species::Empty {
        api.set(0, 0, EMPTY_CELL);
        api.set(0, 1, cell);
    } else if nbr.species == Species::Water {
        api.set(0, 0, nbr);
        api.set(0, 1, cell);
    } else if api.get(dx, 1).species == Species::Empty {
        api.set(0, 0, EMPTY_CELL);
        api.set(dx, 1, cell);
    } else {
        api.set(0, 0, cell);
    }
}

pub fn update_stone(cell: Cell, mut api: SandApi) {
    if api.get(-1, -1).species == Species::Stone && api.get(1, -1).species == Species::Stone {
        return;
    }
    let fluid = api.get_fluid();

    if fluid.pressure > 120 && (js_sys::Math::random() > 0.5) {
        api.set(
            0,
            0,
            Cell {
                species: Species::Sand,
                ra: cell.ra,
                rb: 0,
                clock: 0,
            },
        );
        return;
    }

    let nbr = api.get(0, 1);
    let nbr_species = nbr.species;
    if nbr_species == Species::Empty {
        api.set(0, 0, EMPTY_CELL);
        api.set(0, 1, cell);
    } else if nbr_species == Species::Water
        || nbr_species == Species::Gas
        || nbr_species == Species::Oil
        || nbr_species == Species::Acid
    {
        api.set(0, 0, nbr);
        api.set(0, 1, cell);
    } else {
        api.set(0, 0, cell);
    }
}

pub fn update_water(cell: Cell, mut api: SandApi) {
    let dx = rand_dir();
    let below = api.get(0, 1);
    let dx1 = api.get(dx, 1);
    let dx0 = api.get(dx, 0);
    if below.species == Species::Empty || below.species == Species::Oil {
        api.set(0, 0, below);
        api.set(0, 1, cell);
    } else if dx1.species == Species::Empty || dx1.species == Species::Oil {
        api.set(0, 0, dx1);
        api.set(dx, 1, cell);
    } else if api.get(-dx, 1).species == Species::Empty {
        api.set(0, 0, EMPTY_CELL);
        api.set(-dx, 1, cell);
    } else if dx0.species == Species::Empty || dx0.species == Species::Oil {
        api.set(0, 0, dx0);
        api.set(dx, 0, cell);
    } else if api.get(-dx, 0).species == Species::Empty {
        api.set(0, 0, EMPTY_CELL);
        api.set(-dx, 0, cell);
    } else {
        api.set(0, 0, cell);
    }
}

pub fn update_oil(cell: Cell, mut api: SandApi) {
    let rb = cell.rb;
    let dx = rand_dir();
    let dy = rand_dir();

    let mut new_cell = cell;
    let nbr = api.get(dx, dy);
    if rb == 0 && nbr.species == Species::Fire
        || nbr.species == Species::Lava
        || (nbr.species == Species::Oil && nbr.rb > 1 && nbr.rb < 20)
    {
        new_cell = Cell {
            species: Species::Oil,
            ra: cell.ra,
            rb: 50,
            clock: 0,
        };
    }

    if rb > 1 {
        new_cell = Cell {
            species: Species::Oil,
            ra: cell.ra,
            rb: rb - 1,
            clock: 0,
        };
        api.set_fluid(Wind {
            dx: 0,
            dy: 10,
            pressure: 10,
            density: 180,
        });
        if rb % 4 != 0 && nbr.species == Species::Empty && nbr.species != Species::Water {
            api.set(
                dx,
                dy,
                Cell {
                    species: Species::Fire,
                    ra: 20 + rand_int(30) as u8,
                    rb: 0,
                    clock: 0,
                },
            );
        }
        if nbr.species == Species::Water {
            new_cell = Cell {
                species: Species::Oil,
                ra: 50,
                rb: 0,
                clock: 0,
            };
        }
    } else if rb == 1 {
        api.set(
            0,
            0,
            Cell {
                species: Species::Empty,
                ra: cell.ra,
                rb: 90,
                clock: 0,
            },
        );
        return;
    }

    if api.get(0, 1).species == Species::Empty {
        api.set(0, 0, EMPTY_CELL);
        api.set(0, 1, new_cell);
    } else if api.get(dx, 1).species == Species::Empty {
        api.set(0, 0, EMPTY_CELL);
        api.set(dx, 1, new_cell);
    } else if api.get(-dx, 1).species == Species::Empty {
        api.set(0, 0, EMPTY_CELL);
        api.set(-dx, 1, new_cell);
    } else if api.get(dx, 0).species == Species::Empty {
        api.set(0, 0, EMPTY_CELL);
        api.set(dx, 0, new_cell);
    } else if api.get(-dx, 0).species == Species::Empty {
        api.set(0, 0, EMPTY_CELL);
        api.set(-dx, 0, new_cell);
    } else {
        api.set(0, 0, new_cell);
    }
}

pub fn update_gas(cell: Cell, mut api: SandApi) {
    let dx = rand_dir();
    let dy = rand_dir();
    let nbr = api.get(dx, dy);
    // api.set_fluid(Wind {
    //     dx: 0,
    //     dy: 0,
    //     pressure: 5,
    //     density: 0,
    // });
    if cell.rb == 0 {
        api.set(0, 0, Cell { rb: 5, ..cell });
    }

    if nbr.species == Species::Empty {
        if cell.rb < 3 {
            //single molecule
            api.set(0, 0, EMPTY_CELL);
            api.set(dx, dy, cell);
        } else {
            api.set(0, 0, Cell { rb: 1, ..cell });
            api.set(
                dx,
                dy,
                Cell {
                    rb: cell.rb - 1,
                    ..cell
                },
            );
        }
    } else if (dx != 0 || dy != 0) && nbr.species == Species::Gas && nbr.rb < 4 {
        // if (cell.rb < 2) {
        api.set(0, 0, EMPTY_CELL);
        // }
        api.set(
            dx,
            dy,
            Cell {
                rb: nbr.rb + cell.rb,
                ..cell
            },
        );
    }
}

pub fn update_cloner(cell: Cell, mut api: SandApi) {
    let mut clone_species = unsafe { mem::transmute(cell.rb as u8) };
    let g = api.universe.generation;
    for dx in [-1, 0, 1].iter().cloned() {
        for dy in [-1, 0, 1].iter().cloned() {
            if cell.rb == 0 {
                let nbr_species = api.get(dx, dy).species;
                if nbr_species != Species::Empty
                    && nbr_species != Species::Cloner
                    && nbr_species != Species::Wall
                {
                    clone_species = nbr_species;
                    api.set(
                        0,
                        0,
                        Cell {
                            species: cell.species,
                            ra: 200,
                            rb: clone_species as u8,
                            clock: 0,
                        },
                    );

                    break;
                }
            } else {
                if api.get(dx, dy).species == Species::Empty {
                    api.set(
                        dx,
                        dy,
                        Cell {
                            species: clone_species,
                            ra: 80 + rand_int(30) as u8 + ((g % 127) as i8 - 60).abs() as u8,
                            rb: 0,
                            clock: 0,
                        },
                    );
                    break;
                }
            }
        }
    }
}

pub fn update_firework(cell: Cell, mut api: SandApi) {
    if cell.rb == 0 {
        api.set(
            0,
            0,
            Cell {
                ra: 0,
                rb: 100,
                ..cell
            },
        );
        return;
    }

    let clone_species = if cell.rb != 100 {
        unsafe { mem::transmute(cell.rb as u8) }
    } else {
        Species::Sand
    };

    let sx = rand_dir();
    let sy = rand_dir();
    let sample = api.get(sx, sy);

    if cell.rb == 100
        && sample.species != Species::Empty
        && sample.species != Species::Firework
        && sample.species != Species::Wall
        && sample.species != Species::Cloner
    {
        api.set(
            0,
            0,
            Cell {
                rb: sample.species as u8,
                ..cell
            },
        );
        return;
    } else if cell.rb == 100 && sample.species == Species::Firework && sample.rb != 100 {
        api.set(
            0,
            0,
            Cell {
                rb: sample.rb,
                ..cell
            },
        );
        return;
    }
    let ra = cell.ra;

    if ra == 0 {
        //falling
        let dx = rand_dir();
        let nbr = api.get(0, 1);
        if nbr.species == Species::Empty {
            api.set(0, 0, EMPTY_CELL);
            api.set(0, 1, cell);
        } else if api.get(dx, 1).species == Species::Empty {
            api.set(0, 0, EMPTY_CELL);
            api.set(dx, 1, cell);
        } else if nbr.species == Species::Water
            || nbr.species == Species::Gas
            || nbr.species == Species::Oil
            || nbr.species == Species::Acid
        {
            api.set(0, 0, nbr);
            api.set(0, 1, cell);
        } else {
            api.set(0, 0, cell);
        }
    } else if ra > 5 {
        //rising

        if api.get(0, -2).species == Species::Empty || api.get(0, -2).species == Species::Firework {
            api.set(
                0,
                -2,
                Cell {
                    ra: ra.saturating_sub(1),
                    ..cell
                },
            );
            api.set(0, 0, EMPTY_CELL);
        } else {
            api.set(
                0,
                0,
                Cell {
                    ra: ra.saturating_sub(1),
                    ..cell
                },
            );
        }
        if ra < 10 {
            api.set_fluid(Wind {
                dx: 0,
                dy: 90,
                pressure: 80,
                density: 90,
            });
        }
        return;
    } else {
        api.set_fluid(Wind {
            dx: 0,
            dy: 90,
            pressure: 80,
            density: 90,
        });
        let spawned = Cell {
            species: clone_species,
            ra: 80 + (js_sys::Math::random() * 90.) as u8,
            rb: 0,
            clock: 0,
        };
        api.set(1, 0, spawned);
        api.set(-1, 0, spawned);
        api.set(0, 1, spawned);
        api.set(0, -1, spawned);
        api.set(0, 0, spawned);
        return;
    }

    if sample.species == Species::Fire
        || sample.species == Species::Lava
        || (sample.species == Species::Firework && sample.ra > 5 && sample.rb != 0)
    {
        api.set(
            0,
            0,
            Cell {
                ra: 50 + rand_int(40) as u8,
                ..cell
            },
        );
    }
}
pub fn update_fire(cell: Cell, mut api: SandApi) {
    let ra = cell.ra;
    let mut degraded = cell.clone();
    degraded.ra = ra - (2 + rand_dir()) as u8;

    let dx = rand_dir();
    let dy = rand_dir();
    api.set_fluid(Wind {
        dx: 0,
        dy: 150,
        pressure: 1,
        density: 120,
    });
    if api.get(dx, dy).species == Species::Gas || api.get(dx, dy).species == Species::Dust {
        api.set(
            dx,
            dy,
            Cell {
                species: Species::Fire,
                ra: (150 + (dx + dy) * 10) as u8,
                rb: 0,
                clock: 0,
            },
        );
        api.set_fluid(Wind {
            dx: 0,
            dy: 0,
            pressure: 80,
            density: 40,
        });
    }
    if ra < 5 || api.get(dx, dy).species == Species::Water {
        api.set(0, 0, EMPTY_CELL);
    } else if api.get(dx, -1).species == Species::Empty {
        api.set(0, 0, EMPTY_CELL);
        api.set(dx, -1, degraded);
    } else {
        api.set(0, 0, degraded);
    }
}

pub fn update_lava(cell: Cell, mut api: SandApi) {
    api.set_fluid(Wind {
        dx: 0,
        dy: 10,
        pressure: 0,
        density: 60,
    });
    let dx = rand_dir();
    let dy = rand_dir();

    if api.get(dx, dy).species == Species::Gas || api.get(dx, dy).species == Species::Dust {
        api.set(
            dx,
            dy,
            Cell {
                species: Species::Fire,
                ra: (150 + (dx + dy) * 10) as u8,
                rb: 0,
                clock: 0,
            },
        );
    }
    if api.get(dx, dy).species == Species::Water {
        api.set(
            0,
            0,
            Cell {
                species: Species::Stone,
                ra: (150 + (dx + dy) * 10) as u8,
                rb: 0,
                clock: 0,
            },
        );
        api.set(dx, dy, EMPTY_CELL);
    } else if api.get(0, 1).species == Species::Empty {
        api.set(0, 0, EMPTY_CELL);
        api.set(0, 1, cell);
    } else if api.get(dx, 1).species == Species::Empty {
        api.set(0, 0, EMPTY_CELL);
        api.set(dx, 1, cell);
    } else if api.get(dx, 0).species == Species::Empty {
        api.set(0, 0, EMPTY_CELL);
        api.set(dx, 0, cell);
    } else {
        api.set(0, 0, cell);
    }
}

pub fn update_wood(cell: Cell, mut api: SandApi) {
    let rb = cell.rb;

    let dx = rand_dir();
    let dy = rand_dir();
    let nbr_species = api.get(dx, dy).species;
    if rb == 0 && nbr_species == Species::Fire || nbr_species == Species::Lava {
        api.set(
            0,
            0,
            Cell {
                species: Species::Wood,
                ra: cell.ra,
                rb: 90,
                clock: 0,
            },
        );
    }

    if rb > 1 {
        api.set(
            0,
            0,
            Cell {
                species: Species::Wood,
                ra: cell.ra,
                rb: rb - 1,
                clock: 0,
            },
        );

        if rb % 4 == 0 && nbr_species == Species::Empty {
            api.set(
                dx,
                dy,
                Cell {
                    species: Species::Fire,
                    ra: 30 + rand_int(60) as u8,
                    rb: 0,
                    clock: 0,
                },
            )
        }
        if nbr_species == Species::Water {
            api.set(
                0,
                0,
                Cell {
                    species: Species::Wood,
                    ra: 50,
                    rb: 0,
                    clock: 0,
                },
            )
        }
    } else if rb == 1 {
        api.set(
            0,
            0,
            Cell {
                species: Species::Empty,
                ra: cell.ra,
                rb: 90,
                clock: 0,
            },
        );
    }
}
pub fn update_ice(cell: Cell, mut api: SandApi) {
    let dx = rand_dir();
    let dy = rand_dir();
    let i = rand_int(100);

    let fluid = api.get_fluid();

    if fluid.pressure > 120 && (js_sys::Math::random() > 0.5) {
        api.set(
            0,
            0,
            Cell {
                species: Species::Water,
                ra: cell.ra,
                rb: 0,
                clock: 0,
            },
        );
        return;
    }

    let nbr_species = api.get(dx, dy).species;
    if nbr_species == Species::Fire || nbr_species == Species::Lava {
        api.set(
            0,
            0,
            Cell {
                species: Species::Water,
                ra: cell.ra,
                rb: cell.rb,
                clock: 0,
            },
        );
    } else if nbr_species == Species::Water && i < 7 {
        api.set(
            dx,
            dy,
            Cell {
                species: Species::Ice,
                ra: cell.ra,
                rb: cell.rb,
                clock: 0,
            },
        );
    }
}

pub fn update_plant(cell: Cell, mut api: SandApi) {
    let rb = cell.rb;

    let mut i = rand_int(100);
    let dx = rand_dir();
    let dy = rand_dir();
    let nbr_species = api.get(dx, dy).species;
    if rb == 0 && nbr_species == Species::Fire || nbr_species == Species::Lava {
        api.set(
            0,
            0,
            Cell {
                species: Species::Plant,
                ra: cell.ra,
                rb: 20,
                clock: 0,
            },
        );
    }
    if nbr_species == Species::Wood {
        let dx = rand_dir();
        let dy = rand_dir();

        let drift = (i % 15) - 7;
        let newra = (cell.ra as i32 + drift) as u8;
        if api.get(dx, dy).species == Species::Empty {
            api.set(
                dx,
                dy,
                Cell {
                    species: Species::Plant,
                    ra: newra,
                    rb: 0,
                    clock: 0,
                },
            );
        }
    }
    if rand_int(100) > 80
        && (nbr_species == Species::Water
            || nbr_species == Species::Fungus
                && (api.get(-dx, dy).species == Species::Empty
                    || api.get(-dx, dy).species == Species::Water
                    || api.get(-dx, dy).species == Species::Fungus))
    {
        i = rand_int(100);
        let drift = (i % 15) - 7;
        let newra = (cell.ra as i32 + drift) as u8;
        api.set(
            dx,
            dy,
            Cell {
                ra: newra,
                rb: 0,
                ..cell
            },
        );
        api.set(-dx, dy, EMPTY_CELL);
    }

    if rb > 1 {
        api.set(
            0,
            0,
            Cell {
                ra: cell.ra,
                rb: rb - 1,
                ..cell
            },
        );

        if nbr_species == Species::Empty {
            api.set(
                dx,
                dy,
                Cell {
                    species: Species::Fire,
                    ra: 20 + rand_int(30) as u8,
                    rb: 0,
                    clock: 0,
                },
            );
        }
        if nbr_species == Species::Water {
            api.set(
                0,
                0,
                Cell {
                    ra: 50,
                    rb: 0,
                    ..cell
                },
            )
        }
    } else if rb == 1 {
        api.set(0, 0, EMPTY_CELL);
    }
    let ra = cell.ra;
    if ra > 50
        && api.get(1, 1).species != Species::Plant
        && api.get(-1, 1).species != Species::Plant
    {
        if api.get(0, 1).species == Species::Empty {
            let i = (js_sys::Math::random() * js_sys::Math::random() * 100.) as i32;
            let dec = rand_int(30) - 20;
            if (i + ra as i32) > 165 {
                api.set(
                    0,
                    1,
                    Cell {
                        ra: (ra as i32 + dec) as u8,
                        ..cell
                    },
                );
            }
        } else {
            api.set(
                0,
                0,
                Cell {
                    ra: (ra - 1) as u8,
                    ..cell
                },
            );
        }
    }
}

pub fn update_seed(cell: Cell, mut api: SandApi) {
    let rb = cell.rb;
    let ra = cell.ra;

    let dx = rand_dir();
    let dy = rand_dir();
    let nbr_species = api.get(dx, dy).species;
    if nbr_species == Species::Fire || nbr_species == Species::Lava {
        api.set(
            0,
            0,
            Cell {
                species: Species::Fire,
                ra: 5,
                rb: 0,
                clock: 0,
            },
        );
        return;
    }

    if rb == 0 {
        //falling

        let dx = rand_dir();
        let nbr_species = api.get(dx, 1).species;
        if nbr_species == Species::Sand
            || nbr_species == Species::Plant
            || nbr_species == Species::Fungus
        {
            api.set(
                0,
                0,
                Cell {
                    rb: (rand_int(253) + 1) as u8,
                    ..cell
                },
            );
            return;
        }

        let nbr = api.get(0, 1);
        if nbr.species == Species::Empty {
            api.set(0, 0, EMPTY_CELL);
            api.set(0, 1, cell);
        } else if api.get(dx, 1).species == Species::Empty {
            api.set(0, 0, EMPTY_CELL);
            api.set(dx, 1, cell);
        } else if nbr.species == Species::Water
            || nbr.species == Species::Gas
            || nbr.species == Species::Oil
            || nbr.species == Species::Acid
        {
            api.set(0, 0, nbr);
            api.set(0, 1, cell);
        } else {
            api.set(0, 0, cell);
        }
    } else {
        if ra > 60 {
            //stem
            let dx = rand_dir();
            if rand_int(100) > 75
                && (api.get(dx, -1).species == Species::Empty
                    || api.get(dx, -1).species == Species::Sand
                    || api.get(dx, -1).species == Species::Seed)
                && api.get(1, -1).species != Species::Plant
                && api.get(-1, -1).species != Species::Plant
            {
                api.set(
                    dx,
                    -1,
                    Cell {
                        ra: (ra as i32 - rand_int(10)) as u8,
                        ..cell
                    },
                );
                api.set(
                    0,
                    0,
                    Cell {
                        species: Species::Plant,
                        ra: 80 + (js_sys::Math::random() * 30.) as u8,
                        rb: 0,
                        clock: 0,
                    },
                )
            }
        } else {
            if ra > 40 {
                //petals
                // let i = rand_int(100);

                let mdx = rand_dir();
                let mdy = rand_dir();
                let (ldx, ldy) = adjacency_left((mdx, mdy));
                let (rdx, rdy) = adjacency_right((mdx, mdy));

                if api.get(mdx, mdy).species == Species::Empty
                    && (api.get(ldx, ldy).species == Species::Empty
                        || api.get(rdx, rdy).species == Species::Empty)
                {
                    let i = (js_sys::Math::random() * js_sys::Math::random() * 100.) as i32;
                    let dec = 10 - rand_int(3);
                    if (i + ra as i32) > 100 {
                        api.set(
                            mdx,
                            mdy,
                            Cell {
                                ra: (ra as i32 - dec) as u8,
                                ..cell
                            },
                        );
                    }
                }
            }
        }
    }
}

pub fn update_fungus(cell: Cell, mut api: SandApi) {
    let rb = cell.rb;

    let mut i = rand_int(100);
    let dx = rand_dir();
    let dy = rand_dir();
    let nbr_species = api.get(dx, dy).species;
    if rb == 0 && nbr_species == Species::Fire || nbr_species == Species::Lava {
        api.set(
            0,
            0,
            Cell {
                species: Species::Fungus,
                ra: cell.ra,
                rb: 10,
                clock: 0,
            },
        );
    }
    if nbr_species != Species::Empty
        && nbr_species != Species::Fungus
        && nbr_species != Species::Fire
        && nbr_species != Species::Ice
    {
        let dx = rand_dir();
        let dy = rand_dir();

        let drift = (i % 15) - 7;
        let newra = (cell.ra as i32 + drift) as u8;
        if api.get(dx, dy).species == Species::Empty {
            api.set(
                dx,
                dy,
                Cell {
                    species: Species::Fungus,
                    ra: newra,
                    rb: 0,
                    clock: 0,
                },
            );
        }
    }

    if i > 9
        && nbr_species == Species::Wood
        && api.get(-dx, dy).species == Species::Wood
        && api.get(dx, -dy).species == Species::Wood
        && api.get(dx, dy).ra % 4 != 0
    {
        i = rand_int(100);
        let drift = (i % 15) - 7;
        let newra = (cell.ra as i32 + drift) as u8;
        api.set(
            dx,
            dy,
            Cell {
                ra: newra,
                rb: 0,
                ..cell
            },
        );
    }

    if rb > 1 {
        api.set(
            0,
            0,
            Cell {
                ra: cell.ra,
                rb: rb - 1,
                ..cell
            },
        );
        if nbr_species == Species::Empty {
            api.set(
                dx,
                dy,
                Cell {
                    species: Species::Fire,
                    ra: 10 + rand_int(10) as u8,
                    rb: 0,
                    clock: 0,
                },
            )
        }
        if nbr_species == Species::Water {
            api.set(
                0,
                0,
                Cell {
                    ra: 50,
                    rb: 0,
                    ..cell
                },
            )
        }
    } else if rb == 1 {
        api.set(0, 0, EMPTY_CELL);
    }

    let ra = cell.ra;

    if ra > 120 {
        let mdx = rand_dir();
        let mdy = rand_dir();
        let (ldx, ldy) = adjacency_left((mdx, mdy));
        let (rdx, rdy) = adjacency_right((mdx, mdy));
        if api.get(mdx, mdy).species == Species::Empty
            && api.get(ldx, ldy).species != Species::Fungus
            && api.get(rdx, rdy).species != Species::Fungus
        {
            let i = (js_sys::Math::random() * js_sys::Math::random() * 100.) as i32;
            let dec = 15 - rand_int(20);
            if (i + ra as i32) > 165 {
                api.set(
                    mdx,
                    mdy,
                    Cell {
                        ra: (ra as i32 - dec) as u8,
                        ..cell
                    },
                );
            }
        }
    }
}

// pub fn update_sink(cell: Cell, mut api: SandApi) {
//     let mut dx = rand_dir();
//     let mut dy = rand_dir();

//     // api.set_fluid(Wind {
//     //     dx: 0,
//     //     dy: 0,
//     //     pressure: 5,
//     //     density: 0,
//     // });
//     if api.get(-dx, dy).species != Species::Wall {
//         api.set(0, 0, EMPTY_CELL);
//         api.set(-dx, dy, cell);
//     } else {
//         api.set(0, 0, cell);
//     }
//     // dx = rand_dir();
//     // dy = rand_dir();
//     // if api.get(dx, dy).species != Species::Empty {
//     //     api.set(dx, dy, EMPTY_CELL);
//     //     api.set(0, 0, cell);
//     // }
// }

pub fn update_acid(cell: Cell, mut api: SandApi) {
    let dx = rand_dir();

    let ra = cell.ra;
    let mut degraded = cell.clone();
    degraded.ra = ra - 60;
    // i = rand_int(100);
    if degraded.ra < 80 {
        degraded = EMPTY_CELL;
    }
    if api.get(0, 1).species == Species::Empty {
        api.set(0, 0, EMPTY_CELL);
        api.set(0, 1, cell);
    } else if api.get(dx, 0).species == Species::Empty {
        api.set(0, 0, EMPTY_CELL);
        api.set(dx, 0, cell);
    } else if api.get(-dx, 0).species == Species::Empty {
        api.set(0, 0, EMPTY_CELL);
        api.set(-dx, 0, cell);
    } else {
        if api.get(0, 1).species != Species::Wall && api.get(0, 1).species != Species::Acid {
            api.set(0, 0, EMPTY_CELL);
            api.set(0, 1, degraded);
        } else if api.get(dx, 0).species != Species::Wall && api.get(dx, 0).species != Species::Acid
        {
            api.set(0, 0, EMPTY_CELL);
            api.set(dx, 0, degraded);
        } else if api.get(-dx, 0).species != Species::Wall
            && api.get(-dx, 0).species != Species::Acid
        {
            api.set(0, 0, EMPTY_CELL);
            api.set(-dx, 0, degraded);
        } else if api.get(0, -1).species != Species::Wall
            && api.get(0, -1).species != Species::Acid
            && api.get(0, -1).species != Species::Empty
        {
            api.set(0, 0, EMPTY_CELL);
            api.set(0, -1, degraded);
        } else {
            api.set(0, 0, cell);
        }
    }
}

pub fn update_mite(cell: Cell, mut api: SandApi) {
    let mut i = rand_int(100);
    let mut dx = 0;
    if cell.ra < 20 {
        dx = (cell.ra as i32) - 1;
    }
    let mut dy = 1;
    let mut mite = cell.clone();

    if cell.rb > 10 {
        // /
        mite.rb = mite.rb.saturating_sub(1);
        dy = -1;
    } else if cell.rb > 1 {
        // \
        mite.rb = mite.rb.saturating_sub(1);
    } else {
        // |
        dx = 0;
    }
    let nbr = api.get(dx, dy);

    let sx = (i % 3) - 1;
    i = rand_int(1000);
    let sy = (i % 3) - 1;
    let sample = api.get(sx, sy).species;
    if sample == Species::Fire
        || sample == Species::Lava
        || sample == Species::Water
        || sample == Species::Oil
    {
        api.set(0, 0, EMPTY_CELL);
        return;
    }
    if (sample == Species::Plant || sample == Species::Wood || sample == Species::Seed) && i > 800 {
        api.set(0, 0, EMPTY_CELL);
        api.set(sx, sy, cell);

        return;
    }
    if sample == Species::Dust {
        api.set(sx, sy, if i > 800 { cell } else { EMPTY_CELL });
    }

    if nbr.species == Species::Empty {
        api.set(0, 0, EMPTY_CELL);
        api.set(dx, dy, mite);
    } else if dy == 1 && i > 800 {
        i = rand_int(100);
        let mut ndx = (i % 3) - 1;
        if i < 6 {
            //switch direction
            ndx = dx;
        }

        mite.ra = (1 + ndx) as u8;
        mite.rb = 10 + (i % 10) as u8; //hop height

        api.set(0, 0, mite);
    } else {
        if api.get(-1, 0).species == Species::Mite
            && api.get(1, 0).species == Species::Mite
            && api.get(0, -1).species == Species::Mite
        {
            api.set(0, 0, EMPTY_CELL);
        } else {
            if api.get(0, 1).species == Species::Ice {
                if api.get(dx, 0).species == Species::Empty {
                    api.set(0, 0, EMPTY_CELL);
                    api.set(dx, 0, mite);
                }
            } else {
                api.set(0, 0, mite);
            }
        }
    }
}
