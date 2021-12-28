fn main() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        let _y=10;
        println!("The value of x in the inner scope is: {}", x);
    }

    println!("The value of x is: {}", x);
    
    let mul=30.0*11.0;
    println!("The value of mul is: {}", mul);

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    println!("The value of tuple is: {}", tup.1);

    let a: [i32; 5] = [1, 2, 3,4,5];
    println!("The value of array is: {}", a[0]);

    first_function("Test");

    let y={
      let z=1;
      z;
    };

    println!("The value of y is: {}", y);
}

fn first_function(message:&str){
  println!("Hello {}",message);
}