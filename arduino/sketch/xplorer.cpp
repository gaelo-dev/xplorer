#include "xplorer.h"

Xplorer::Xplorer(int rx, int tx) : ble(rx, tx) {
    Motors mots;
}

void Xplorer::setup(int enA, int inA1, int inA2, int enB, int inB1, int inB2) {
    ble.begin(9600);

    mots.attachMotorA(enA, inA1, inA2);
    mots.attachMotorB(enB, inB1, inB2);
}

void Xplorer::listen() {
  if (ble.available()) {
    byte msg = ble.read();
    if (command == 0) {
      command = msg >> 6;
      action = msg & B00111111;
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

void Xplorer::execute() {
  switch (command)
  {
  case 1:
    // command motor
    break;
  
  case 2:
    // command arm
    break;

  default:
    break;
  }
  
  // ...
}
