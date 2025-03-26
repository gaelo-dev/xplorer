#include "./xplorer.h"

// El TXD del módulo al pin 5 (RX) del Arduino.
// El RXD del módulo al pin 6 (TX) del Arduino.
Xplorer xplorer(5, 6);

void setup() {
  Serial.begin(9600);

  xplorer.attachMotors(4, 41, 43, 3, 45, 47);
  xplorer.attachArm(5, 6, 7, 8, 9, 10);
  xplorer.begin()
}

void loop() {
  xplorer.listen();
}
