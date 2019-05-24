mod conway;
use conway::Conway;

const ROUNDS: u32 = 1000;

fn search_immortal(rounds: u32,conway:&mut conway::Conway) {
    let mut i: u32 = 1;

    println!("Initial Status:");
    println!("------------------------------------");
    conway.print();

    while i <= rounds && conway.is_alive() {
        conway.evolve();
        i += 1;
    }

    if conway.is_alive() {
        println!("This could be an immortal:");
        conway.print();
    } else {
        println!("The colony died after {} rounds! :(", i);
    }
    
}

fn main() {
    let mut conway = Conway::new();

    conway.gen_rnd_conway();
    search_immortal(ROUNDS, &mut conway);

}
