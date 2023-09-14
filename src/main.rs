fn main() {

    let s = String::from("¡Hola amigos!");

    let y = toma_propiedad(s);
    let (y,longitud) = calcula_longitud(s);
    println!("{}", s);

    // Solución: la Referencia

    // La función tiene una referencia a un objeto como parámetro en lugar de tomar
    // posesión del valor.

    // Ello se consigue mediante el símbolo & (referencia a partir de ahora).
    // Desfrenciar se hace con el símbolo * que veremos más adelante


    // Préstamo
    // Llamamos préstamo a tener referencias como parámetrso de función. Como en
    // la vida real, si una persona posee algo, se lo puede pedir prestado. Cuando
    // termines de usarlo, tienes que devolverlo.

    // ¿Qué sucede si queremos modificar algo que tenemos prestado?

    // Como la vida misma
    // No se puede modifical lo que te han prestado

    modificar(&s);

    println!("{}", s, longitud);
    
    // Y ahora vamos a implementar la referencia mutable

    let mut ss = String::from("¡Hola ");

    modificar(&mut ss);

    println!("{}",ss);


    // Referencias mutables

    // Pero las referencias mutables tienen una gran restricción:
    // solo puede haber una referencia mutable a un dato en un ámbito determinado.

    let mut sss = String::from("¡Hola amigos!");

    let cad1 = &mut sss;
    let cad2 = &mut sss;

    println!("{} {}",cad1, cad2);

    let mut ssss = String::from("¡Hola amigos!");

    let cad1 = &sss;
    let cad2 = &sss;
    let cad3 = &mut sss;

    println!("{} {} {}", cad1, cad2, cad3);
    // No pueden coexistir esas dos referencias...

    let mut ssss = String::from("¡Hola amigos!");

    let cad11 - &ssss;
    let cad22 = &ssss;
    println!("{}, cad11, cad22");

    let cad33 = &mut ssss;
    println!("{}", cad33);


    // Restricción
    
    // Esta restricción permite la mutación pero de una manera mut controlada. Es
    // algo comn lo que los nuevos rustáceis tienen que enfrentarse, porque la mayoría
    // de los lenguajes de programación te permiten mutar cuando quieras.

    // El beneficion de esta restricción es que Rust puede evitar las carreras de datos en
    // tiempo de compilación. Una carrera de datos es similar a una condición de carrera
    // y ocurre cuando ocurren estos tres comportamientos:
    // * Dos o más punteros acceden a los mismos datos al mismo tiempo.
    // * Se está utilizando al menos uno de los punteros para escribir en los datos.
    // * No se utiliza ningún mecanismo para sincronizar el acceso a los datos.


    // Referencias colgantes

    // En los lenguajes con punteros, es fácil crear erróneamente un puntero colgante,
    // un puntero que hace referencia a una ubicación en la memoria que se le puede
    // haber dado a otra persona, liberando algo de memoria mientras se conserva un
    // puntero a esa memoria.

    // En Rust el compilador garantiza que las referencias nunca serán referencias
    // colgantes: si tiene una referencia a algunos datos, el compilador se asegurará de
    // que los datos no salgan del ámbito antes de que lo haga la referencia a los datos.

    let referencia_a_nada = colgante();


}

fn colgante() -> String {
    let sssss = String::from("Referencia colgante");
    sssss
}

fn modificar(cadena: &String) {
    cadena.push_str("amigos !"); // Error, porqqe no se puede modifical un objeto prestado
}


fn calcula_longitud(cadena: String) -> (String, usize) {
    let aux = cadena.len();
    (cadena, aux)
}

fn toma_propiedad(cadena: String) -> String {
    println!("{}", cadena);
    cadena
}








