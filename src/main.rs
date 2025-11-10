use flock::bird::Bird;
use raylib::prelude::*;

fn main() {
    let (mut rl, thread) = raylib::init()
        .build();

    let mut birds = vec![];
    for _ in 0..100 {
        let bird = Bird::new(Vector2::new(100.0, 100.0));
        birds.push(bird);
    }

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);

        for bird in &birds {
            bird.draw(&mut d);
        }
    }
}
