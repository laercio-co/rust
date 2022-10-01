fn main() {
    let mut n: (u32, u32) = (0, 1);
    for i in 0..20 {
        if i > 0 {
            n = (n.1, n.0 + n.1);
        }
        println!("F{} - {}", i, n.0);
    }

    // let mut n2: u32 = 0;
    // let mut n1: u32 = 1;
    // let mut n: u32 = 0;

    // for i in 0..20 {
    //     println!("F{} - {}", i, n);
    //     n = 1;
    //     if i > 0 {
    //         n = n1 + n2;
    //         n2 = n1;
    //         n1 = n;
    //     }
    // }
}
