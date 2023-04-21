pub const COMMANDS:[&str;17] = [ "rsh",
"cd",
"exit",
"pipes",
"background",
"fg",
"jobs",
"history",
"again",
"ctrl+c",
"chain",
"conditional",
"true",
"false",
"get",
"set",
"unset",
];
pub const COMMANDS_HELP:[&str;17] = [ "Raudel Alejandro Gómez Molina\n\nFuncionalidades:\n\nbasic: funcionalidades básicas\npipes: implementación de múltiples tuberías\nbackground: permite correr procesos en el background\nspaces: los comandos pueden estar separados por cualquier cantidad de espacios\nhistory: se almacena un historial de comandos\nctrl+c: finaliza el proceso actual\nchain: permite ejecutar múltiples comandos en una sola línea y comandos de forma condicional\nconditional: permite ejecutar comandos de forma condicional\nvariables: permite almacenar variables\n\nComandos:\n\ncd: cambia de directorio\nexit: finaliza la ejecución del shell\nfg: trae hacia el foreground el último proceso enviado al background\njobs: lista todos los procesos en el background\nhistory: muestra el historial de comandos\nagain: ejecuta un comando almacenado en el historial\ntrue: representa un comando que siempre se ejecuta con éxito\nfalse: representa un comando que nunca se ejecuta con éxito\nget: muestra el valor de las variables\nset: modifica el valor de una variable\nunset: elimina una variable\n",
"El comando cd <dir> cambia el directorio actual del shell al directorio especificado por el usuario, si <dir> no se  especifica se toma por default el home como directorio.\n\nEjemplo:\n\nrsh $ cd new_dir # se mueve hacia la carpeta new_dir\nrsh $ cd         # se mueve hacia home\n",
"El comando exit finaliza la ejecución del shell.\n",
"En este apartado encontramos 4 comandos ( | , < , > , >> ) que nos permitirán redirigir la entrada y la salida de los comandos que ejecutemos.\n\nEl comando command1 | command2 ejecuta <command1>, redirecciona la salida de <command1> a la entrada de <command2> y luego ejecuta <command2>.\n\nEl comando command < file redirecciona el contenido del archivo <file> a la entrada de <command> y ejecuta <command>.\n\nEl comando command > file ejecuta <command> redirecciona la salida de <command> hacia el archivo <file>, sobrescribiendo\nel contenido de <file>.\n\nEl comando command >> file hace lo mismo que el comando anterior pero escribe al final de <file> sin sobrescribir el\ncontenido de dicho archivo.\n\nAdicionalmente se pueden combinar todas estos comandos en una sola línea.\n\nEjemplo:\n\nrsh $ command1 | command2\nrsh $ command < file\nrsh $ command > file\nrsh $ command >> file\nrsh $ command1 < file1 | command2 | command3 > file2 # el contenido de file1 se redirecciona a la entrada de command1, la salida de command1 se redirecciona a la entrada de command2, la salida de command2 se redirecciona a la entrada de command3 y la salida de command3 se redirecciona a file2.\n",
"El operador & al final de un comando ejecuta dicho comando el background esto significa que dicho comando se ejecuta  sin afectar el ciclo del shell.\n\nEjemplo:\n\nrsh $ cp movie.mkv .. & # el comando se envía al background\nrsh $ ls                # el ciclo del shell no espera a que el comando anterior termine su ejecución.\n",
"El comando fg <number> trae el proceso enumerado con <number> desde el background a el foreground. Si <number> no se especifica se toma el último proceso enviado hacia el background.\n\nEjemplo:\n\nrsh $ cp movie.mkv .. &\nrsh $ rm file.zip &\nrsh $ jobs\n[1]     cp movie.mkv ..     1925\n[2]     rm file.zip     1927\nrsh $ fg 1                      # el comando <cp movie.mkv ..> se trae de vuelta al foreground\n                                  # esperamos hasta que <cp movie.mkv ..> se ejecute\nrsh $ fg                        # el comando <rm file.zip> se trae de vuelta al foreground\n",
"El comando jobs lista todos los procesos que están ocurriendo en el background.\n\nEjemplo:\n\nrsh $ cp movie.mkv .. &\nrsh $ rm file.zip &\nrsh $ jobs             # lista los 2 comandos anteriores ocurriendo en el background\n[1]     cp movie.mkv ..     1925\n[2]     rm file.zip     1927\n",
"El comando history muestra los últimos 100 comandos ejecutados en el shell, enumerados desde 1 para el más antiguo hasta 100 para el último comando.\n\nEjemplo:\n\nrsh $ history\n1: ls\n2: cd\n3: history\nrsh $ cd      # se guarda en el historial\nrsh $ history\n1: ls\n2: cd\n3: history\n4: cd\n5: history\nrsh $  hola   # no se guarda en el historial\n",
"El comando again <number> ejecuta un comando almacenado en el historial <number> viene dado por el orden en que se ejecutaron los comandos desde 1 para el más viejo hasta 100 para el último, si <number> no se especifica se ejecuta  el ultimo comando. Una vez que se ejecuta el comando especificado anteriormente este pasa al historial.\n\nEjemplo:\n\nrsh $ history\n1: ls\n2: cd\n3: pwd\n4: history\nrsh $ again 1 # se ejecuta ls\nrsh $ again   # se ejecuta ls dado que fue el último comando\n",
"ctrl+c envía una señal SIGINT al comando que se está ejecutando, si se ejecuta nuevamente ctrl+c se envía una señal SIGKILL al comando que se está ejecutando.\n",
"En este apartado tenemos los comandos ( ; , || , && ).\n\nEl operador ; permite ejecutar varios comandos en la misma línea.\n\nEl comando command1 && command2 ejecuta command1 y si este tiene éxito ejecuta command2\n\nEl comando command1 || command2 ejecuta command1 y si este no tiene éxito ejecuta command2 en caso contrario no se ejecuta más nada.\n\nAdicionalmente se pueden combinar todos estos comandos en una sola línea. Contamos con otros 2 comandos especiales (true, false) que simulan una condición que siempre se cumple y otra que nunca se cumple respectivamente.\n\nEjemplo:\n\nrsh $ command1; command2;\nrsh $ command1 && command2\nrsh $ command1 || command2\nrsh $ command1 && command2; || command3 # se ejecutará command1, si este tiene éxito se ejecutará command2, si ambos commandos command1 y command2 tienen éxito no se ejecutará más nada en caso contrario se ejecutará command3\n",
"En este apartado encontramos 4 comandos (if, then, else y end) que nos permiten realizar una operación condicional en una sola línea.\n\nEl comando if <condition> then <execute1> else <execute2> end, primero ejecuta el comando <condition> y si este tiene éxito entonces se ejecuta <execute1> en caso contrario se ejecuta <execute2>, end se usa para indicar en fin de la operación condicional. El comando else puede no especificarse: if <condition> then <execute> end, si <condition> tiene exito <execute> se ejecutará en caso contrario no pasa nada.\n\nAdicionalmente contamos con otros 2 comandos especiales (true, false) que simulan una condición que siempre se cumple y otra que nunca se cumple respectivamente. Adicionalmente se puede anidar una operación condicional dentro de otra.\n\nEjemplo:\n\nrsh $ if cond then execute end\nrsh $ if cond then execute1 else execute2 end\nrsh $ if cond1 then if cond2 then execute1 end else execute2 end\nrsh $ if true then execute end                                   # siempre se ejecutará execute\nrsh $ if false then execute end                                  # nunca se ejecutará execute\n",
"El comando true siempre se ejecuta con éxito, puede ver detalles de su uso en chain y conditional.\n",
"El comando false siempre falla al ejecutarse, puede ver detalles de su uso en chain y conditional.\n",
"El comando get <c> muestra el valor de la variable <c> en el sistema. Si la variable no existe el comando no tendrá éxito. Si <c> no se especifica el comando lista todas las variables.\n\nEjemplo:\n\nrsh $ set a hola\nrsh $ get a      # se muestra el valor de a\nhola\nrsh $ get b      # b no se encuentra el sistema\nrsh $ set c 25\nrsh $ get        # se muestran todas las variables del sistema\na = hola\nc = 25\n",
"El comando set <c> <value> permite introducir una nueva variable al sistema o modificar el valor de una ya existente. Las variables introducidas deben ser letras minúsculas del alfabeto inglés. Si <value> es está entre comillas `command` se ejecuta command y la salida de de ese comando se almacena como valor de <c>, si la salida del comando es vacía la ejecución de set no tiene éxito y por tanto no se guarda la variable.\n\nEjemplo:\n\nrsh $ set a hola # como a no existe se crea en el sistema con valor hola\nrsh $ set a `ls` # el valor de a se modifica con la salida de ls\narchivo.txt rsh\nrsh $ get a\narchivo.txt rsh\nrsh $ set a `cd` # la salida de cd es vacía por tanto el set falla y el valor de la variable no cambia\n",
"El comando unset <c> elimina la variable <c> del sistema con su respectivo valor. Si <c> no existe el comando no tiene éxito.\n\nEjemplo:\n\nrsh $ set c hola\nrsh $ get c\nhola\nrsh $ unset c # c se elimina del sistema\n",
];
