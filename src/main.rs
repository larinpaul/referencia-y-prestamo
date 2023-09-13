fn main() {

    let s = String::from("¡Hola amigos!");

    let y = toma_propiedad(s);
    let (y,lognitud) = cadena_longitud(s);
    println!("{}", s);

}


fn calcula_longitud(cadena: String) -> (String, usize) {
    let aux = cadena.len();
    (cadena, aux)
}

fn toma_propiedad(cadena: String) -> String {
    println!("{}", cadena);
    cadena
}


// Solución: la Referencia

// La función tiene una referencia a un objeto como parámetro en lugar de tomar
// posesión del valor.

// Ello se consigue mediante el símbolo & (referencia a partir de ahora).
// Desfrenciar se hace con el símbolo * que veremos más adelante




