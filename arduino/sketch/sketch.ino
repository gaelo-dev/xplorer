#include <SoftwareSerial.h>
#include "./xplorer.h"

// El TXD del módulo al pin 2 (RX) del Arduino.
// El RXD del módulo al pin 3 (TX) del Arduino.
SoftwareSerial HM10(2, 3);

byte command = 0;
byte action = 0;
int value = 0;

void setup() {
  Serial.begin(9600);
  HM10.begin(9600);
}

void loop() {
  if (HM10.available()) {
    byte msg = HM10.read();
    if (command == 0) {
      command = msg >> 5;
      action = msg & B00011111;
    } else if (value == 0) {
      action = msg;
    }

    if (msg == NULL) {
      execute();
      command = 0;
      action = 0;
      value = 0;
    } 
    
  }
}

void execute() {
  
  if bitRead(command, 0) {
    // command motor
  }

  if bitRead(command, 1) {
    // command servo
  }
  
  // ...

}
