#include "./xplorer.h"

// El TXD del módulo al pin 2 (RX) del Arduino.
// El RXD del módulo al pin 3 (TX) del Arduino.
Xplorer xplorer(2, 3);

void setup() {
  Serial.begin(9600);
  xplorer.setup(4, 41, 43, 3, 45, 47);
}

void loop() {
  xplorer.listen();
}
