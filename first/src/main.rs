fn main() {
    println!("First dude!");
    println!("this is interested");

    let number = 22;
    println!("This is the number {}", number);

    let li = (500, 6.4, 1);

    let (_w, y, _z) = li;

    println!("Values {}", y);

    let a = [1, 2, 3, 4, 5];
    let _index = 3;

    // let element = a[index];

    println!("The value of element is: {}", a[1]);
    function(6,6)

}

// Simple function with parameters
fn function(number_one: i32, number_two: i32){
    if number_one == number_two{
        println!("Yes")
    } else {
        println!("No")
    }
}
