mod agenda;
use agenda::agenda::agenda;
use std::io;
fn main() {
    print!(
        "Que desea hacer (Seleccione el numero de la opcion deseada): \n1. Añadir contacto\n2. Editar contacto\n3. Borrar contacto.\n"
    );

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("error: unable to read user input");
    let eleccion = input
        .replace("\r", "")
        .replace("\n", "")
        .parse::<u8>()
        .unwrap();

    agenda(eleccion);
}
