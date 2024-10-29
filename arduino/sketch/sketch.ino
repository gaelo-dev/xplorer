#include <SoftwareSerial.h>
// El TXD del módulo al pin 2 (RX) del Arduino.
// El RXD del módulo al pin 3 (TX) del Arduino.
SoftwareSerial HM10(2, 3);

const int LED13 = 13;

char caracter = '\0';
String mensaje = "";

struct Command {
    String name;
    String action;
    int value;
    bool hasValue;

    // Constructor para inicializar la estructura
    Command() : name(""), action(""), value(0), hasValue(false) {}

    // Función para procesar los caracteres y construir el comando
    void process(char character) {
        static String buffer = "";  // Buffer para acumular el comando completo

        if (character != ';') {
            buffer += character;  // Agregar el carácter al buffer
            return;
        }

        // Procesar el comando completo cuando se recibe el carácter ';'
        parse(buffer);
        buffer = "";  // Limpiar el buffer para el siguiente comando
    }

    // Función para analizar el comando y extraer el nombre, acciones y valores
    void parse(String commandStr) {
        int firstPlus = commandStr.indexOf('+');
        if (firstPlus == -1) {
            Serial.println("Formato de comando inválido");
            return;
        }

        // Extraer el nombre del comando
        name = commandStr.substring(0, firstPlus);
        Serial.print("Comando principal: ");
        Serial.println(name);

        // Procesar cada acción en el comando
        int currentPos = firstPlus + 1;
        while (currentPos < commandStr.length()) {
            int nextPlus = commandStr.indexOf('+', currentPos);
            int equals = commandStr.indexOf('=', currentPos);

            if (nextPlus == -1) nextPlus = commandStr.length();  // Última acción

            action = commandStr.substring(currentPos, (equals != -1 && equals < nextPlus) ? equals : nextPlus);
            hasValue = (equals != -1 && equals < nextPlus);

            if (hasValue) {
                value = commandStr.substring(equals + 1, nextPlus).toInt();
            }

            // Mostrar acción y valor
            Serial.print("Acción: ");
            Serial.print(action);
            if (hasValue) {
                Serial.print(" con valor: ");
                Serial.println(value);
            } else {
                Serial.println(" sin valor");
            }

            // Ejecutar la acción según el nombre y valor de la acción
            execute();

            currentPos = nextPlus + 1;
        }
    }

    // Función que ejecuta acciones basadas en los valores obtenidos
    void execute() {
        if (action == "SPEED" && hasValue) {
            Serial.print("Configurando velocidad del motor a: ");
            Serial.println(value);
            // Implementa lógica para ajustar velocidad
        } else if (action == "FORWARD") {
            Serial.println("Moviendo hacia adelante");
            // Implementa lógica para moverse hacia adelante
        } 
    }
};


Command command;

void setup() {
  Serial.begin(9600);
  HM10.begin(9600);
  pinMode(LED13, OUTPUT);
}

void loop() {
  if (HM10.available()) {
    char inChar = (char)HM10.read();
    command.process(inChar);
  }
}
