mod conway;
use conway::Conway;
use std::time::Duration;
use std::thread;

const ROUNDS: u32 = 100000;


fn search_immortal(rounds: u32, conway: &mut conway::Conway) {
    let mut i: u32 = 1;

    while i <= rounds && conway.is_alive() && conway.is_window_open(){
        conway.evolve();
        conway.print();
        i += 1;

        thread::sleep(Duration::from_millis(500));
    }
}

fn main() {
    let mut conway = Conway::new();

    conway.gen_rnd_conway();
    search_immortal(ROUNDS, &mut conway);

}
