// Question 1
use std::io;

// Question 2
fn main() 
{
    // User input height in cm
    let mut height = String::new();
    println!("Enter Height in cm=> ");
    io::stdin().read_line(&mut height).expect("Failed to read line");

    // conversion of height(string) to float
    let height_in_cm: f32 = height.trim().parse().expect("Please Type a Number");

    // user input weight in kg
    let mut weight = String::new();
    println!("Enter Weight in kg=> ");
    io::stdin().read_line(&mut weight).expect("Failed to read line");

    // conversion of weight(string) to float
    let weight_in_kg: f32 = weight.trim().parse().expect("Please Type a Number");

    // convert height(cm) into meter
    let height_in_m : f32 = height_in_cm / 100.0;

    // calculating BMI
    let bmi: f32 = weight_in_kg / (height_in_m * height_in_m);
    println!("Your BMI=> {}", bmi);

    if bmi < 18.5
    {
        println!("Under-Weight");
    }
    else if bmi >= 18.5 && bmi <= 25.0
    {
        println!("Normal Weight");
    }
    else
    {
        println!("Over-Weight");
    }
    ////////////////////////////////////////////////////////////
    
    let mut index = 1;

    while index <= 10
    {
        if index == 3 || index == 7 || index == 10
        {
            println!("Special Security Check!  at Number=> {}", index);
        }
        else
        {
            println!("Number=> {}", index);
        }
        index += 1;
    }
}
