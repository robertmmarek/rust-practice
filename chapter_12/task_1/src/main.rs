use std::thread;


fn main() {
    let a = thread::spawn(|| {
        while (1==1) {
            println!("Hello ");
            thread::sleep_ms(100);
        }
    });
    let b = thread::spawn(|| {
        while (1==1) {
            println!("World!");
            thread::sleep_ms(100);
        }
    });

    while(1==1){
        thread::sleep_ms(200);
    }
}
