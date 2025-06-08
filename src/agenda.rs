use std::io;

use uuid::Uuid;

pub fn agenda(eleccion: u8) {
    match eleccion {
        1 => añadir_contacto(),

        2 => editar_contacto(),

        3 => borrar_contacto(),

        4 => mostrar_contactos(),

        5 => println!("Saliste!"),

        _ => mostrar_opciones(),
    }
}

fn añadir_contacto() {
    print!("\x1B[2J\x1B[1;1H");
    println!("Coloque el nombre del contacto: ");
    let mut nombre = String::new();
    io::stdin()
        .read_line(&mut nombre)
        .expect("error: unable to read user input");
    print!("\x1B[2J\x1B[1;1H");

    print!("\x1B[2J\x1B[1;1H");
    println!("Coloque el apellido del contacto: ");

    let mut apellido = String::new();
    io::stdin()
        .read_line(&mut apellido)
        .expect("error: unable to read user input");

    print!("\x1B[2J\x1B[1;1H");
    println!("Coloque el telefono del contacto: ");
    let mut telefono = String::new();
    io::stdin()
        .read_line(&mut telefono)
        .expect("error: unable to read user input");

    let contacto = Contacto {
        nombre: nombre.trim().to_string(),
        apellido: apellido.trim().to_string(),
        numero_telefono: telefono.trim().to_string(),
        _id: Uuid::new_v4(),
    };

    println!(
        "Nombre: {}, Apellido: {}, Telefono: {}, ID: {}",
        contacto.nombre, contacto.apellido, contacto.numero_telefono, contacto._id
    );
    mostrar_opciones();
}

fn editar_contacto() {
    mostrar_opciones();
    println!("Editar");
}

fn borrar_contacto() {
    mostrar_opciones();
    println!("Borrar");
}

fn mostrar_contactos() {
    mostrar_opciones();
    println!("Contactos")
}

#[derive(Debug)]
pub struct Contacto {
    nombre: String,
    apellido: String,
    numero_telefono: String,
    _id: Uuid,
}

pub fn mostrar_opciones() {
    print!("\x1B[2J\x1B[1;1H");
    print!(
        "Que desea hacer (Seleccione el numero de la opcion deseada): \n1. Añadir contacto\n2. Editar contacto\n3. Borrar contacto.\n4. Mostrar contactos\n5. Salir.\n"
    );
}
