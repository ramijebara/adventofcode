fn main() {
    let area = Area {
        x1: 88,
        y1: -157,
        x2: 125,
        y2: -103,
    };

    let mut global_best_y = 0;
    let mut accumulator = 0;

    for i in 1..256 {
        let mut run_best_y = 0;

        for j in -256..256 {

            // initialize state
            let mut x_v = i;
            let mut y_v = j;
            let mut probe = Probe::new();

            loop {
                let p_in_area = probe.in_area(&area);
                let p_over_shoot = probe.over_shoot(&area);

                if p_in_area || p_over_shoot {
                    if p_in_area {
                        accumulator += 1;
                        if run_best_y > global_best_y {
                            global_best_y = run_best_y;
                        }
                    }

                    break;
                }

                if probe.y > run_best_y {
                    run_best_y = probe.y;
                }

                if x_v > 0 {
                    probe.x += x_v;
                    x_v -= 1;
                }

                probe.y += y_v;
                y_v -= 1;
            }
        }
    }

    println!("best y: {}, number of valid initial velocities: {}", global_best_y, accumulator);
}

#[derive(Debug)]
struct Probe {
    x: isize,
    y: isize,
}

impl Probe {
    pub fn new() -> Self {
        Probe { x: 0, y: 0 }
    }

    pub fn in_area(&self, area: &Area) -> bool {
        if self.x >= area.x1 && self.x <= area.x2 && self.y >= area.y1 && self.y <= area.y2 {
            return true;
        }
        false
    }

    pub fn over_shoot(&self, area: &Area) -> bool {
        if self.x > area.x2 || self.y < area.y1 {
            return true;
        }
        false
    }
}

#[derive(Debug)]
struct Area {
    x1: isize,
    y1: isize,
    x2: isize,
    y2: isize,
}
