use std::mem::size_of;

static mut COUNTER: u32 = 0; // Mutable static variable

fn increment_counter() {
    unsafe{COUNTER += 1;} // This line is unsafe!
}

fn circleproperties(radius: f64) -> (f64, f64) {
    let area = std::f64::consts:: PI * radius.powi(2);
    let circumference = 2.0 * std::f64::consts::PI*radius;
    (area, circumference)
}

fn main() {
    let check:u32 = 20;

    // need to keep mut incase you wannt to change the value of run later !!
    let mut r = 20;

    println!("Value is {}", check);

    r = 30;

    println!("Value is {}", r);

    println!("Hello, world!");
   
    println!("bool:        {} bytes", size_of::<bool>());
    println!("char:        {} bytes", size_of::<char>());
    println!("i8:          {} bytes", size_of::<i8>());
    println!("i16:         {} bytes", size_of::<i16>());
    println!("i32:         {} bytes", size_of::<i32>());
    println!("i64:         {} bytes", size_of::<i64>());
    println!("u8:          {} bytes", size_of::<u8>());
    println!("u16:         {} bytes", size_of::<u16>());
    println!("u32:         {} bytes", size_of::<u32>());
    println!("u64:         {} bytes", size_of::<u64>());
    println!("f32:         {} bytes", size_of::<f32>());
    println!("f64:         {} bytes", size_of::<f64>());
    println!("usize:       {} bytes", size_of::<usize>());
    println!("isize:       {} bytes", size_of::<isize>());

    // start for const values safing
    increment_counter();

    unsafe {
        println!("{}", COUNTER);
    }
    // end for const values safing

    // start changing the value address
    let x = "praveen";

    let y = &x;

    println!("value: {}", x);

    let x = 3.14;
    println!("2-value: {}", x);
    
    println!("value: {}", *y);
    // end - changing the value address

    // start - type casting
    let pi:f64=3.1432;
    let checkpi:u32 = pi as u32;

    println!("value : {}", checkpi);
    // end - type casting
    
    //start tuple
    let my_tuple:(i32, f64, String) = (10, 3.14, "Hellow".to_string());

    println!("tuple.0 {}", my_tuple.0);
    println!("tuple.1 {}", my_tuple.1);
    println!("tuple.2 {}", my_tuple.2);

    let(x,y,z) = my_tuple;
    
    println!("tuple.0 {}", x);
    println!("tuple.1 {}", y);
    println!("tuple.2 {}", z);
    
    let(area,circumference) = circleproperties(5.0);

    println!("area {} circumference {} ", area, circumference);

    let mut another_tuple = (1, vec![2,3,4]);
    another_tuple.1.push(5);

    println!("modified: {:?}", another_tuple);
    //end tuple
    
    //start array
    let arr = [0;5];
    let mynumbers:[u32;5] = [1,2,3,4,5];

    println!("{:?}", arr);
    println!("{:#?}", mynumbers);

    for number in mynumbers {
        println!("{}", number);
    }

    println!("{}",mynumbers[0]);

    //end array
}
