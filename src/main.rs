use std::env;
use std::fs;

/*
They say rust is a great language for compilers
They say rust is not a great language for shit compilers

Me knowing it's bad but still making a shit compiler in rust:
https://www.youtube.com/watch?v=dQw4w9WgXcQ
 */

fn main() {
    let file_contents = get_file_contents();
    let mut convertable_array = Vec::new();
    let splitted_array = file_contents.split(" ").collect::<Vec<&str>>();
    let splitted_array_length = splitted_array.len();

    //we need an even amount of stans to represent a byte because stan is 4 letters
    if splitted_array_length %2 != 0
    {
        panic!("You must have an even amount of Stans!");
    }


    let mut counter = 0;

    for _i in 0..splitted_array_length/2
    {
        if !splitted_array[counter].to_lowercase().eq("stan") || !splitted_array[counter+1].to_lowercase().eq("stan")
        {
            panic!("Nothing else allowed other than stan!!!!")
        }

        //concatenate the 2 stans to represent a byte
        let together = splitted_array[counter].to_string()+splitted_array[counter+1];
        convertable_array.push(together);

        counter+=2;
    }

    let bytes: Vec<u8> = array_to_bytes(convertable_array);

    //print out the output
    for byte in bytes {
        print!("{}", byte as char);
    }
}

fn array_to_bytes(convertable_array: Vec<String>) -> Vec<u8>
{
    let mut return_array: Vec<u8> = Vec::new();

    let mut current_multiplier = 1;
    let mut current_number: u8 = 0;
    for element in convertable_array
    {
        for char in element.chars().rev().collect::<String>().chars()
        {
            if char.is_uppercase()
            {
                current_number += current_multiplier as u8;
            }
            current_multiplier*=2;
        }

        return_array.push(current_number as u8);
        current_number = 0;
        current_multiplier = 1;
    }

    return return_array;
}

fn get_file_contents() -> String
{
    let args: Vec<String> = env::args().collect();
    let filename = args.get(1).expect("You have to supply a file name!");

    let contents = fs::read_to_string(filename)
        .expect("Should have been able to read the file");

    return contents;
}