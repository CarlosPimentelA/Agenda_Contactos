use serde::{Deserialize, Serialize};
use std::{
    fs::File,
    io::{self, Read, Write},
    path::Path,
};
use uuid::Uuid;

pub fn agenda(eleccion: u8) {
    match eleccion {
        1 => añadir_contacto(),

        2 => editar_contacto(),

        3 => borrar_contacto(),

        4 => mostrar_contactos(),

        5 => mostrar_opciones(),

        _ => mostrar_opciones(),
    }
}

fn añadir_contacto() {
    let mut contactos: Vec<Contacto> = vec![];
    print!("\x1B[2J\x1B[1;1H");
    println!("Coloque el nombre del contacto: ");
    let mut nombre = String::new();
    nombre = obtener_input(&mut nombre).trim().to_string();

    if nombre.is_empty() || nombre.len() <= 2 {
        mostrar_opciones();
        println!("Coloca un nombre real!");
        return;
    };
    print!("\x1B[2J\x1B[1;1H");
    println!("Coloque el apellido del contacto: ");

    let mut apellido = String::new();
    apellido = obtener_input(&mut apellido).trim().to_string();

    if apellido.is_empty() || apellido.len() <= 2 {
        mostrar_opciones();
        println!("Coloca un apellido real!");
        return;
    };
    print!("\x1B[2J\x1B[1;1H");
    println!("Coloque el telefono del contacto: ");
    let mut telefono = String::new();
    telefono = obtener_input(&mut telefono).trim().to_string();

    if telefono.is_empty() || telefono.len() < 10 || !telefono.chars().all(|c| c.is_numeric()) {
        mostrar_opciones();
        println!(
            "El telefono debe tener 10 numeros y no debe contener letras ni numeros. Ejemplo: 1234567890"
        );
        return;
    };
    let contacto = Contacto {
        nombre: nombre.trim().to_string(),
        apellido: apellido.trim().to_string(),
        numero_telefono: telefono.trim().to_string(),
        _id: Uuid::new_v4(),
    };

    contactos.push(contacto);
    let contacto_json = serde_json::to_string_pretty(&contactos).unwrap();
    mostrar_opciones();
    guardar_contacto(contacto_json);
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
    let contactos = read_json("contactos.json");
    mostrar_opciones();
    for contacto in contactos {
        println!(
            "Nombre: {}\nApellido: {}\nTelefono: {}\n",
            contacto.nombre, contacto.apellido, contacto.numero_telefono
        );
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Contacto {
    nombre: String,
    apellido: String,
    numero_telefono: String,
    _id: Uuid,
}

pub fn mostrar_opciones() {
    print!("\x1B[2J\x1B[1;1H");
    print!(
        "Que desea hacer (Seleccione el numero de la opcion deseada): \n1. Añadir contacto\n2. Editar contacto\n3. Borrar contacto.\n4. Mostrar contactos\n5. Limpiar pantalla\n6. Salir.\n"
    );
}

fn obtener_input(input: &mut String) -> &String {
    let mut input = input;
    io::stdin()
        .read_line(&mut input)
        .expect("error: unable to read user input");

    return input;
}

fn guardar_contacto(contacto_json: String) {
    let filename = "contactos.json";
    let verify = std::fs::exists(filename).unwrap();
    let contactos: Vec<Contacto> = serde_json::from_str(&contacto_json).unwrap();

    if !verify {
        let mut file = File::create(filename).unwrap();
        let _ = file.write_all(contacto_json.as_bytes());
    } else {
        let mut all_contacts = read_json(filename);
        if all_contacts.iter().any(|contactos_existentes: &Contacto| {
            contactos.iter().any(|contato_nuevo: &Contacto| {
                contactos_existentes.numero_telefono == contato_nuevo.numero_telefono
            })
        }) {
            println!("Ese numero ya existe");
            return;
        }
        all_contacts.extend(contactos);
        let all_contacts_parsed = serde_json::to_string_pretty(&all_contacts).unwrap();
        let mut file = File::create(filename).unwrap();
        let _ = file.write_all(all_contacts_parsed.as_bytes());
    }
}

fn read_json(filename: &str) -> Vec<Contacto> {
    let path = Path::new(&filename);
    let mut fdata = String::new();
    let mut rfile = File::open(path).expect("File error");
    rfile.read_to_string(&mut fdata).expect("File error");
    let contactos: Vec<Contacto> = serde_json::from_str(&fdata).unwrap();

    return contactos;
}
