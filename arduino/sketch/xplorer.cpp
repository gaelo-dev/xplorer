#include "xplorer.h"

Command::Command(int cmd, byte act, int val) : cmd(cmd), action(act), value(val) {}

Command Command::humidity(int val) { return Command(3, 1 << 0, val); }
Command Command::temperature(int val) { return Command(3, 1 << 1, val); }
Command Command::gas(int val) { return Command(3, 1 << 2, val); }
Command Command::distance(int val) { return Command(3, 1 << 3, val); }

Xplorer::Xplorer(int rx, int tx) : ble(rx, tx), command(0, 0, 0) {}

void Xplorer::attachMotors(int enA, int inA1, int inA2, int enB, int inB1, int inB2) {
	mots.attachMotorA(enA, inA1, inA2);
	mots.attachMotorB(enB, inB1, inB2);
}

void Xplorer::attachArm(int pinBase, int pinElbow, int pinRest, int pinShoulder, int pinDoll, int pinGrip) {
	armito.attach(pinBase, pinElbow, pinRest, pinShoulder, pinDoll, pinGrip);
}

void Xplorer::begin() {
	ble.begin(9600);
}

void Xplorer::listen() {
  if (ble.available()) {
    byte msg = ble.read();
    if (command.cmd == 0) {
      command = Command(msg >> 6, msg & B00111111, 0);
    } else if (command.value == 0) {
      command.value = msg;
    }

    if (msg == NULL) {
      execute();
      command = Command(0, 0, 0);
    } 
  }
}

void Xplorer::execute() {
	switch (command.cmd)
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
