
mod redis;

fn main() {
    redis::basics();
    redis::hash();
    redis::list();
    redis::set();
    redis::sorted_set();
}
