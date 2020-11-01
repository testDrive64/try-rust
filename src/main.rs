/*--------------------------------------------------------------------------------------------------------------
 * Copyright (c) Microsoft Corporation. All rights reserved.
 * Licensed under the MIT License. See https://go.microsoft.com/fwlink/?linkid=2090316 for license information.
 *-------------------------------------------------------------------------------------------------------------*/
 extern crate ferris_says;
 extern crate raster;
 use ferris_says::say;
 use std::io::{stdout, BufWriter};
 //use std::fmt;
use std::fmt::{self, Formatter, Display};
//use raster::Color;


struct City {
    name: &'static str,
    lat: f32,
    lon: f32,
}

impl Display for City {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let lat_c = if self.lat >= 0.0 { 'N' } else { 'S' };
        let lon_c = if self.lon >= 0.0 { 'E' } else { 'W' };

        write!(f, "{}: {:.3}°{} {:.3}°{}", self.name, self.lat.abs(), lat_c, self.lon.abs(), lon_c)
    }
}

#[derive(Debug)]
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        print!("RGB ({}, {}, {}) ", self.red, self.green, self.blue);
        write!(f, "0x{:02X?}{:02X?}{:02X?}", self.red, self.green, self.blue)
    }
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
        println!("{}", *color);
    }

    println!("1 +2 = {}", 1u32 + 2);

    println!("1 - 2 = {}", 1i32 - 2);

    println!("true AND false is {}", true && false);
    println!("true OR false is {}", true || false);
    println!("Not true is {}", !true);

    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    println!("1 << 5 is {}", 1u32 << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);

    // Bei großen Zahlen Unterstrich, zur besseren Lesbarkeit benutzen
    println!("One million is written as {}", 1_000_000u32);
}