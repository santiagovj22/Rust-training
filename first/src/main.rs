fn main() {
    println!("First dude!");
    println!("this is interested");

    let number = 22;
    println!("THis is the number {}", number);

    let _li = (500, 6.4, 1);

    let (_w, _y, _z) = _li;

    println!("Values {}", _y);

    let a = [1, 2, 3, 4, 5];
    let index = 3;

    let element = a[index];

    println!("The value of element is: {}", a[1]);
}
