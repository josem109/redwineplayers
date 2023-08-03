fn main() {
    let edad = 300;
    let mensaje_edad = match edad {
        18 => "Tienes 18, apenas puedes entrar",
        17 => "Todavia no puedes entrar, falta poco",
        0..=16 => "Aun eres menor de edad",
        22..=100 => "Disfruta con responsabilidad",
        19 | 20 | 21 => "Consume alcohol con moderacion",
        _ => "Tu edad aun no esta programada",
    };
    println!("El mensaje es: {}", mensaje_edad);
}
