mod agenda;
use agenda::agenda;
use std::io;
fn main() {
    print!(
        "Que desea hacer (Seleccione el numero de la opcion deseada): \n1. Añadir contacto\n2. Editar contacto\n3. Borrar contacto.\n4. Mostrar contactos\n5. Salir.\n"
    );
    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("error: unable to read user input");

        let input = match input.trim().parse::<u8>() {
            Ok(input) => input,
            Err(_) => {
                println!("Coloque un valor valido (1 - 4)");
                continue;
            }
        };

        agenda(input);
        if input == 5 {
            break;
        }
    }
}
