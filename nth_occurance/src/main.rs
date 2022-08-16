mod enough_is_enough;
fn main() {

    let arr = [1,2,2,3,4,5,3,7,2,9];
let v = enough_is_enough::enough(&arr, 2);

println!("{:?}", v);
}

