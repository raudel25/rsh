# rsh

Implementación de un **shell** en **Rust**.

## Ejecutando el Proyecto

Para ejecutar el proyecto debe tener instalado en su sistema operativo el ambiente de desarrollo de **Rust**.
El proyecto cuenta con varias dependencias, las cuales se especifican en el archivo `Cargo.toml`. Una vez cumplidas
estas especificaciones debe dirigirse a la raíz del proyecto y ejecutar:

```
make
```

si estas en **Linux** o 

```
cargo run
```

### Funcionalidades:

- basic: funcionalidades básicas
- pipes: implementación de múltiples tuberías
- background: permite correr procesos en el background
- spaces: los comandos pueden estar separados por cualquier cantidad de espacios
- history: se almacena un historial de comandos
- ctrl+c: finaliza el proceso actual
- chain: permite ejecutar múltiples comandos en una sola línea y comandos de forma condicional
- conditional: permite ejecutar comandos de forma condicional
- variables: permite almacenar variables
- format: permite dar un formato específico al comando introducido por el usuario (comillas y paréntesis)

### Comandos:

- cd: cambia de directorio
- exit: finaliza la ejecución del shell
- fg: trae hacia el foreground el último proceso enviado al background
- jobs: lista todos los procesos en el background
- history: muestra el historial de comandos
- again: ejecuta un comando almacenado en el historial
- true: representa un comando que siempre se ejecuta con éxito
- false: representa un comando que nunca se ejecuta con éxito
- get: muestra el valor de las variables
- set: modifica el valor de una variable
- unset: elimina una variable


Los detalles de las funcionalidades y la ejecución se detallan dentro del propio
programa mediante el comando `help`.



