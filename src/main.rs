/*--------------------------------------------------------------------------------------------------------------
 * Copyright (c) Microsoft Corporation. All rights reserved.
 * Licensed under the MIT License. See https://go.microsoft.com/fwlink/?linkid=2090316 for license information.
 *-------------------------------------------------------------------------------------------------------------*/
 extern crate ferris_says;
 use ferris_says::say;
 use std::io::{stdout, BufWriter};
 //use std::fmt;
use std::fmt::{self, Formatter, Display};


struct City {
    name: &'static str,
    lat: f32,
    lon: f32,
}

impl Display for City {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let lat_c = if self.lat >= 0.0 { 'N' } else { 'S' };
        let lon_c = if self.lon >= 0.0 { 'E' } else { 'w' };

        write!(f, "{}: {:.3}°{} {:.3}°{}", self.name, self.lat.abs(), lat_c, self.lon.abs(), lon_c)
    }
}

#[derive(Debug)]
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}
 struct List(Vec<i32>);

 impl fmt::Display for List {
     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
         let vec = &self.0;
         write!(f, "[")?;

         for (count, v) in vec.iter().enumerate() {
             if count != 0 { write!(f, ", ")?; }
             write!(f, "{}", v)?;
         }
         write!(f, "]")
     }
 }
#[derive(Debug)]
struct MinMax(i64, i64);

impl fmt::Display for MinMax {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

 #[derive(Debug)]
struct Point2D {
    x: f64,
    y: f64
}

impl fmt::Display for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "x: {}, y: {}", self.x, self.y)
    }
}

#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8
}

fn main() {
    let minmax = MinMax(0, 14);

    println!("Compare structures:");
    println!("Display: {}", minmax);
    println!("Debug: {:?}", minmax);
    let big_range = MinMax(-300,300);
    let small_range = MinMax(-3, 3);

    println!("The big range is {big} and the small is {small}", small = small_range, big= big_range);

    let point = Point2D { x:3.3, y: 7.2 };

    println!("Compare points:");
    println!("Display: {}", point);
    println!("Debug: {:?}", point);
    let stdout = stdout();
    let name = "Toni";
    let age = 31;
    let toni = Person{ name, age};

    let message = "Caprese, Mageritha, Schinken, Roma, Roma mit Salami";
    let width = message.chars().count();
    let mut writer = BufWriter::new(stdout.lock());

    println!("{:#?}", toni);

    say(message.as_bytes(), width, &mut writer).unwrap();

    let v = List(vec![1, 2, 3]);
    println!("{}", v);


    for city in [
        City { name:"Dublin", lat: 53.347778, lon: -6.259722 },
        City { name: "Osolo", lat: 59.95, lon: 10.75 },
        City { name: "Vancouver", lat: 49.25, lon: -123.1 },
    ].iter() {
        println!("{}", *city);
    }
    for color in [
        Color { red: 128, green: 255, blue: 90 },
        Color { red: 0, green: 3, blue: 254 },
        Color { red: 0, green: 0, blue: 0 },
    ].iter() {
        println!("{:?}", *color);
    }
}