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

    if telefono.is_empty() || telefono.len() <= 9 || !telefono.chars().all(|c| c.is_numeric()) {
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
        _id: Some(Uuid::new_v4()),
    };

    contactos.push(contacto);
    let contacto_json = serde_json::to_string_pretty(&contactos).unwrap();
    mostrar_opciones();
    guardar_contacto(contacto_json);
}

fn editar_contacto() {
    let filename = String::from("contactos.json");
    let mut contactos = read_json(&filename);
    let mut input = String::new();
    println!("Coloque el numero del contacto a editar: ");
    let telefono = obtener_input(&mut input).trim().to_string();
    let contacto_a_editar = match contactos
        .iter_mut()
        .find(|contacto| contacto.numero_telefono == *telefono)
    {
        Some(contacto) => contacto,
        None => {
            println!("Numero inexistente!");
            return;
        }
    };

    let contactos_sin_edicion = read_json(&filename);
    let mut contactos_sin_editar = contactos_sin_edicion
        .iter()
        .filter(|contacto| contacto.numero_telefono != telefono)
        .collect::<Vec<_>>();
    input = String::from("");
    println!("\x1B[2J\x1B[1;1H");
    println!(
        "Seleccione la opcion que desea editar del contacto:\n1. Todo el contacto\n 2. El nombre\n 3. El apellido\n 4. El numero\n "
    );
    let opciones = obtener_input(&mut input).trim().parse::<u8>();
    if opciones == Ok(1) {
        let mut nombre = String::new();
        println!("Coloque el nombre: ");
        nombre = obtener_input(&mut nombre).trim().to_string();
        let mut apellido = String::new();
        println!("Coloque el apellido: ");
        apellido = obtener_input(&mut apellido).trim().to_string();
        let mut telefono = String::new();
        println!("Coloque el telefono: ");
        telefono = obtener_input(&mut telefono).trim().to_string();

        contacto_a_editar.set_all_contact(nombre, apellido, telefono);
    }
    if opciones == Ok(2) {
        let mut nombre = String::new();
        println!("Coloque el nombre: ");
        nombre = obtener_input(&mut nombre).trim().to_string();
        contacto_a_editar.set_nombre(nombre);
    }
    if opciones == Ok(3) {
        let mut apellido = String::new();
        println!("Coloque el apellido: ");
        apellido = obtener_input(&mut apellido).trim().to_string();
        contacto_a_editar.set_apellido(apellido);
    }
    if opciones == Ok(4) {
        let mut telefono = String::new();
        println!("Coloque el telefono: ");
        telefono = obtener_input(&mut telefono).trim().to_string();
        contacto_a_editar.set_telefono(telefono);
    }
    contactos_sin_editar.push(contacto_a_editar);

    let contacto_parsed = serde_json::to_string_pretty(&contactos_sin_editar).unwrap();
    let mut file = File::create(filename).unwrap();
    let _ = file.write_all(contacto_parsed.as_bytes());

    mostrar_opciones();
}

fn borrar_contacto() {
    let filename = String::from("contactos.json");
    println!("Coloque el numero del contacto a eliminar: ");
    let mut telefono = String::new();
    let all_contacts = read_json(&filename);
    telefono = obtener_input(&mut telefono).trim().to_string();
    let contacto_filtrado = all_contacts
        .iter()
        .filter(|contacto| contacto.numero_telefono != telefono)
        .collect::<Vec<_>>();
    let contacto_parsed = serde_json::to_string_pretty(&contacto_filtrado).unwrap();
    let mut file = File::create(filename).unwrap();
    let _ = file.write_all(contacto_parsed.as_bytes());

    mostrar_opciones();
}

fn mostrar_contactos() {
    let contactos = read_json("contactos.json");
    mostrar_opciones();

    if contactos.is_empty() {
        println!("No hay contactos!")
    }
    for contacto in contactos {
        println!(
            "Nombre: {}\nApellido: {}\nTelefono: {}\n",
            contacto.nombre, contacto.apellido, contacto.numero_telefono
        );
    }
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

#[derive(Serialize, Deserialize, Debug, Clone)]

pub struct Contacto {
    nombre: String,
    apellido: String,
    numero_telefono: String,
    _id: Option<Uuid>,
}

impl Contacto {
    fn set_nombre(&mut self, nombre: String) {
        self.nombre = nombre;
    }
    fn set_apellido(&mut self, apellido: String) {
        self.apellido = apellido;
    }
    fn set_telefono(&mut self, telefono: String) {
        self.numero_telefono = telefono;
    }

    fn set_all_contact(&mut self, nombre: String, apellido: String, telefono: String) {
        self.nombre = nombre;
        self.apellido = apellido;
        self.numero_telefono = telefono;
    }
}
