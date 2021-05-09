extern "C" {
    fn printf(fmt: &str);
}

static mut NUMBER: i32 = 0;

fn main() {
    unsafe {
        printf("Hello world!\n");
        NUMBER = 5;
        println!("number: {}", NUMBER);
    }
}
