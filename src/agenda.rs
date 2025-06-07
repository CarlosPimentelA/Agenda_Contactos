pub mod agenda {
    pub fn agenda(eleccion: u8) {
        match eleccion {
            1 => añadir_contacto(),

            2 => editar_contacto(),

            3 => borrar_contacto(),

            _ => print!("Porfavor seleccione el numero de la opcion indicada"),
        }
    }

    fn añadir_contacto() {
        print!("Añadir");
    }

    fn editar_contacto() {
        print!("Añadir")
    }

    fn borrar_contacto() {
        print!("Añadir")
    }
}
