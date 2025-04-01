#include "./xplorer.h"

// El TXD del módulo al pin 5 (RX) del Arduino.
// El RXD del módulo al pin 6 (TX) del Arduino.
Xplorer xplorer(5, 6, A0, 20, 21, 32);

void setup() {
  Serial.begin(9600);

  xplorer.attachMotors(4, 41, 43, 3, 45, 47);
  xplorer.attachArm(7, 8, 9, 10, 11, 12);
  xplorer.begin();
}

void loop() {
  xplorer.listen();
  xplorer.sendDataSensors();
  delay(100);
}
