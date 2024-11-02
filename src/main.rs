#[macro_use]
extern crate rocket;

use rocket::{launch};
use rocket::serde::{json::Json, Serialize, Deserialize};
use rocket::{State, get, post, put, delete, routes};
use std::sync::Mutex;

// Definimos la estructura 'Estudiante' que representará a cada estudiante en nuestra "base de datos".
#[derive(Serialize, Deserialize, Clone)]
struct Estudiante {
    nombre: String,
    apellido: String,
    numero_control: String,
    edad: u8,
    semestre: u8,
}

// Definimos una estructura para mantener el estado de nuestra aplicación.
struct StateDB {
    estudiantes: Mutex<Vec<Estudiante>>,
}

// Endpoint para obtener la lista de todos los estudiantes.
#[get("/estudiantes")]
fn obtener_estudiantes(state: &State<StateDB>) -> Json<Vec<Estudiante>> {
    let estudiantes = state.estudiantes.lock().unwrap();
    Json(estudiantes.clone())
}

// Endpoint para agregar un nuevo estudiante.
#[post("/estudiantes", format = "json", data = "<nuevo_estudiante>")]
fn crear_estudiante(nuevo_estudiante: Json<Estudiante>, state: &State<StateDB>) -> Json<String> {
    let mut estudiantes = state.estudiantes.lock().unwrap();
    estudiantes.push(nuevo_estudiante.into_inner());
    Json("Estudiante agregado con éxito".to_string())
}

// Endpoint para actualizar la información de un estudiante existente.
#[put("/estudiantes/<numero_control>", format = "json", data = "<estudiante_actualizado>")]
fn actualizar_estudiante(numero_control: String, estudiante_actualizado: Json<Estudiante>, state: &State<StateDB>) -> Json<String> {
    let mut estudiantes = state.estudiantes.lock().unwrap();
    for estudiante in estudiantes.iter_mut() {
        if estudiante.numero_control == numero_control {
            *estudiante = estudiante_actualizado.into_inner();
            return Json("Estudiante actualizado".to_string());
        }
    }
    Json("Estudiante no encontrado".to_string())
}

// Endpoint para eliminar un estudiante de la lista.
#[delete("/estudiantes/<numero_control>")]
fn eliminar_estudiante(numero_control: String, state: &State<StateDB>) -> Json<String> {
    let mut estudiantes = state.estudiantes.lock().unwrap();
    estudiantes.retain(|estudiante| estudiante.numero_control != numero_control);
    Json("Estudiante eliminado".to_string())
}

// Función principal que inicia el servidor de Rocket.
#[launch]
fn rocket() -> _ {
    rocket::build()
        .manage(StateDB {
            estudiantes: Mutex::new(vec![]),
        })
        .mount("/", routes![obtener_estudiantes, crear_estudiante, actualizar_estudiante, eliminar_estudiante])
        .configure(rocket::Config {
            port: 8081, // Cambia a otro puerto
            ..Default::default()
        })
}
