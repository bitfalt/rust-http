# rust-http

# Instituto Tecnológico de Costa Rica
## Escuela de Ingeniería en Computación
### Implementación de Servidor HTTP

- Greivin Mauricio Fernández Brizuela c.2022437510
- Daniel Alonso Garbanzo Carvajal c.2022117129
- Ericka Michelle Cerdas Mejias c.2022138199

** IC-6600 -- Principios de Sistemas Operativos
* Fecha de entrega: 4 de octubre

## 1. Introducción

Este proyecto implementa un servidor HTTP simple desde cero utilizando Rust. El servidor soporta las principales operaciones HTTP (GET, POST, PUT, DELETE, PATCH) y gestiona sesiones de usuario con cookies. Está diseñado para manejar múltiples solicitudes concurrentes utilizando hilos y asegura el acceso seguro a los datos de sesión mediante `Arc` y `Mutex`.

## 2. Descripción General de la Arquitectura

El servidor está estructurado en tres componentes principales:

1. **Estructura `Server`**: Gestiona los datos de sesión y maneja las conexiones entrantes.
2. **Estructura `Client`**: Representa una conexión de cliente individual, manejando el análisis de solicitudes y el envío de respuestas.
3. **Estructura `HttpRequest`**: Representa una solicitud HTTP con su método, ruta, encabezados, cuerpo y cookie.

### Módulos Clave:

- **Concurrencia**: El servidor utiliza el modelo de hilos de Rust (`thread::spawn`) junto con `Arc` (Conteo de Referencias Atómicas) y `Mutex` para gestionar datos compartidos entre hilos. Esto permite al servidor manejar múltiples clientes simultáneamente sin colisiones de datos o condiciones de carrera.
- **Gestión de Sesiones**: Las sesiones se gestionan mediante cookies. Cada sesión se identifica por un ID de sesión único generado usando `uuid`, que se almacena en un `HashMap` dentro de la estructura `Server`. Esto permite al servidor rastrear las sesiones de usuario a través de múltiples solicitudes.

## 3. Cómo se Maneja la Concurrencia

La concurrencia se logra utilizando las características de la biblioteca estándar de Rust:

- **Hilos**: Cada conexión entrante genera un nuevo hilo utilizando `thread::spawn`, permitiendo al servidor manejar múltiples solicitudes simultáneamente.
- **Gestión de Datos Compartidos**: Se utiliza el patrón `Arc<Mutex<Server>>` para compartir de forma segura el acceso a los datos de sesión del servidor entre hilos. `Arc` permite múltiples propietarios, y `Mutex` asegura que solo un hilo pueda acceder o modificar los datos a la vez.

## 4. Operaciones HTTP

El servidor soporta las siguientes operaciones HTTP:

- **GET**: Recupera recursos basados en la ruta solicitada.
- **POST**: Procesa datos enviados en el cuerpo de la solicitud.
- **PUT**: Actualiza recursos con los datos proporcionados.
- **DELETE**: Elimina recursos especificados por la ruta.
- **PATCH**: Actualiza parcialmente recursos con los datos proporcionados.

## 5. Gestión de Cookies

El servidor maneja la gestión de sesiones utilizando cookies. Cuando un nuevo cliente se conecta, se genera un ID de sesión único utilizando la crate `uuid`, y se almacena en el `HashMap` de sesiones del servidor. Si una solicitud contiene una cookie de sesión, el servidor verifica las sesiones existentes y reutiliza la sesión si es válida.

## 6. Manejo de Errores y Robustez

El servidor incorpora un manejo básico de errores para gestionar posibles problemas como solicitudes mal formadas, desconexiones de clientes y fallos en el flujo de datos.

- **Solicitudes Mal Formadas**: El servidor verifica si hay solicitudes incompletas o mal formadas y devuelve mensajes de error apropiados.
- **Registro**: Utiliza las funciones de la crate `log` (`info`, `error`) para registrar las operaciones del servidor y los errores, lo que ayuda en la depuración y el monitoreo.

## 8. Tipo de Thread Pool Utilizado
Para mejorar la gestión de la concurrencia y controlar mejor el uso de los recursos del sistema, implementamos un Fixed Thread Pool utilizando la crate threadpool. Esto permite que el servidor maneje múltiples conexiones simultáneamente, limitando el número de hilos activos en un momento dado. Esto evita la sobrecarga del sistema que podría ocurrir si se crearan demasiados hilos dinámicamente.

## 9. Pruebas del Fixed Thread Pool
- **Prueba de Creación de Hilos**:
   Objetivo:
   Verificar que el Fixed Thread Pool crea hilos solo hasta el límite establecido y reutiliza los hilos existentes para nuevas solicitudes.

   Procedimiento:
      1. Configura el pool con un tamaño fijo, por ejemplo, 4 hilos: let pool = ThreadPool::new(4);.
      2. Inicia el servidor.
      3. Envía múltiples solicitudes (más de 4) concurrentemente usando curl o una herramienta similar:
   
   Resultado Esperado:
   Solo se crean 4 hilos, y estos se reutilizan para manejar todas las solicitudes, sin crear hilos adicionales.
![Figura 1: Comando prueba]()
- **Prueba de Saturación del Pool**:
   Objetivo:
   Evaluar cómo responde el pool cuando todas las threads están ocupadas y llegan más solicitudes.

   Procedimiento:
      1. Envía 10 solicitudes concurrentes rápidas
   Resultado Esperado:
   Las primeras 4 solicitudes se procesan inmediatamente; las demás se encolan y se procesan a medida que los hilos se desocupan.

- **Prueba de Reutilización de Hilos**:
   Objetivo:
   Asegurarse de que los hilos se reutilizan para múltiples tareas sin ser destruidos y recreados.

   Procedimiento:
      1. Envía una serie de solicitudes espaciadas en el tiempo (cada 1 segundo).
   Resultado Esperado:
   Los hilos activos son reutilizados para cada nueva solicitud, sin necesidad de crear nuevos hilos.

## 7. Instrucciones para Ejecutar el Servidor

1. **Configuración**:
   - Asegúrate de que Rust esté instalado en tu sistema. Si no lo está, puedes instalarlo desde [el sitio web oficial de Rust](https://www.rust-lang.org/).

2. **Dependencias**:
   - El proyecto utiliza las crates `uuid` y `log`. Asegúrate de que estas dependencias estén incluidas en tu archivo `Cargo.toml`:
   ```toml
   [dependencies]
   log = "0.4"
   env_logger = "0.10"
   uuid = { version = "1.2", features = ["v4"] }

3. **Ejecutar el servidor**:
    - Ejecutar un bash y dirigirse a la carpeta http-rust
    - Ejecutar el comando cargo run
    - Enviar request por medio de postman o curl