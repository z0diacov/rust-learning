fn main() {
    const LIMITS: [u32; 3] = [10, 50, 100];
    let mut counters: [u32; 3] = [0, 0, 0];

    let mut log: Vec<u32> = Vec::new();

    counters[1] = 32;

    for i in LIMITS {
        log.push(i);
    }

    println!("{:?}", counters);

    println!("{:?}", log);
    println!("{:#?}", log);

    //LIMITS[0] = 5;
}