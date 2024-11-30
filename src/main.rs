use std::collections::HashSet;
use std::io;

fn main() {
    let mut string1 = String::new();
    let mut string2 = String::new();

    println!("Ingrese la primera cadena:");
    io::stdin()
        .read_line(&mut string1)
        .expect("Error al leer la primera cadena");
    println!("Ingrese la segunda cadena:");
    io::stdin()
        .read_line(&mut string2)
        .expect("Error al leer la segunda cadena");

    // Eliminar espacios y saltos de línea al final
    let string1 = string1.trim();
    let string2 = string2.trim();

    // Comparación básica
    if string1 == string2 {
        println!("Las cadenas son iguales.");
    } else {
        println!("Las cadenas son diferentes.");
        if string1.contains(string2) {
            println!("La segunda cadena es una subsecuencia de la primera.");
        } else if string2.contains(string1) {
            println!("La primera cadena es una subsecuencia de la segunda.");
        }
    }

    // Análisis de caracteres
    let chars1: HashSet<char> = string1.chars().collect();
    let chars2: HashSet<char> = string2.chars().collect();

    // Caracteres comunes y únicos
    let comunes: HashSet<_> = chars1.intersection(&chars2).collect();
    let unicos1: HashSet<_> = chars1.difference(&chars2).collect();
    let unicos2: HashSet<_> = chars2.difference(&chars1).collect();

    println!("\n--- Análisis de caracteres ---");
    println!("Caracteres comunes: {:?}", comunes);
    println!("Caracteres únicos en la primera cadena: {:?}", unicos1);
    println!("Caracteres únicos en la segunda cadena: {:?}", unicos2);
    println!("Total de caracteres comunes: {}", comunes.len());

    // Comparación lexicográfica
    if string1 < string2 {
        println!(
            "Lexicográficamente, '{}' es menor que '{}'.",
            string1, string2
        );
    } else {
        println!(
            "Lexicográficamente, '{}' es mayor que '{}'.",
            string1, string2
        );
    }
}
