use bevy::prelude::*;

struct Position {
    x: f32,
    y: f32,
}

fn print_position_system(query: Query<&Transform>) {
    for transform in query.iter() {
        println!("position: {:?}", transform.translation);
    }
}

struct Entity(u64);

fn hello_world() {
    println!("hello world!");
}

fn main() {
    App::build().add_system(hello_world.system()).run();
}
