use custom_dag::Dag;
use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();
    for _ in 1..1000 {
        let node_id: u32 = rng.gen();
    }
}