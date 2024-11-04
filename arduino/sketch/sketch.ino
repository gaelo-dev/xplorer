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

    if (msg == NULL) {
      execute();
      command = 0;
      action = 0;
    } else if (command != 0) {
      command = msg >> 4;
      action = msg & B00001111;
    } else {
      action = msg;
    }
  }
}

void execute() {
  // todo
}
