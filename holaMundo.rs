
/*
AI
Primer ejemplo del del curso de rust, por cierto, este es un comentario de varias lineas en lenguaje rust.
*/
fn main()
{
    println!("\n¡Palestina libreee!\n");

    /*
     Impresión de texto en consola 
     '{}' será reemplazado por cualquier argumento 
     */
    println!("Feliz {} aniversario Republica popular de China",75);
    /*
    También podemos especificar que argumento queremos usar en el lugar del {} correspondiente usando un entero, no se pueden usar números negativos.
    */
    println!("{3},{2}, {0}, {1}, Perdon! tardamos mucho en deshacernos de la plaga judia, ya va {4} un año\n.","Gaza","Rafah","Haifa","Jerusalen",1);

    /*
    También podemos formatear la salida de cantidades en otor sistemas de numeración.
    */
    println!("Base 10:                  {}",   69420); // 69420
    println!("Base 2 (binario):         {:b}", 69420); // 10000111100101100
    println!("Base 8 (octal):           {:o}", 69420); // 207454
    println!("Base 16 (hexadecimal):    {:x}", 69420); // 10f2c

    let number: f64 = 1.0;
    let width: usize = 5;
    println!("{number:>width$}");
    /*
    esta version es erronea debido a que no existe un argumento 1
    println!("My name is {0}, {1} {0}", "Bond");
    */
    //Tipos de datos primitos
    /*
    Hay más maneras de hacer formateos en la impresión de texto en la consola de comandos pero no son de gran utilidad por lo pronto.
    */
}


