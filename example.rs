use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();
    let goal = 3;
    let mut count = 0.0;

    for x in 0..100000 {
        let mut int1 = rng.gen_range(1, 7);
        let mut int2 = rng.gen_range(1, 7);
        let mut int3 = rng.gen_range(1, 7);
        if int1 + int2 + int3 == goal{
            count +=1.0
        }
    }
    println!("{} percent", count/1000.0)

}
