#[macro_export]
macro_rules! vecky {
    ($($x:expr); *) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    }
}

fn main() {
    let v = vecky!(1; 2; 3);
}
