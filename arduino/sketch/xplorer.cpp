#include "xplorer.h"

Command::Command(int cmd, byte act, int val): cmd(cmd), action(act), value(val) {}

Command Command::humidity(int val) { return Command(3, 1 << 0, val); }
Command Command::temperature(int val) { return Command(3, 1 << 1, val); }
Command Command::gas(int val) { return Command(3, 1 << 2, val); }
Command Command::distance(int val) { return Command(3, 1 << 3, val); }

Xplorer::Xplorer(int rx, int tx, int pinGas, int pinEcho, int pinTrigger, int pinDHT): ble(rx, tx), command(0, 0, 0), sens(pinGas, pinEcho, pinTrigger, pinDHT), data(0, 0, 0, 0) {}

void Xplorer::attachMotors(int enA, int inA1, int inA2, int enB, int inB1, int inB2) {
	mots.attachMotorA(enA, inA1, inA2);
	mots.attachMotorB(enB, inB1, inB2);
}

void Xplorer::attachArm(int pinBase, int pinElbow, int pinRest, int pinShoulder, int pinDoll, int pinGrip) {
  armito.attach(pinBase, pinElbow, pinRest, pinShoulder, pinDoll, pinGrip);
}

void Xplorer::begin() {
	ble.begin(9600);
  sens.begin();
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
    if (bitRead(command.action, 0)) { mots.forward(); }
    if (bitRead(command.action, 1)) { mots.backward(); }
    if (bitRead(command.action, 2)) { mots.rightward(); }
    if (bitRead(command.action, 3)) { mots.leftward(); }
    if (bitRead(command.action, 4)) { mots.setSpeed(command.value); }

    break;
  
  case 2:
    // command arm
    if (bitRead(command.action, 0)) { armito.writeBase(command.value); }
    if (bitRead(command.action, 1)) { armito.writeElbow(command.value); }
    if (bitRead(command.action, 2)) { armito.writeRest(command.value); }
    if (bitRead(command.action, 3)) { armito.writeShoulder(command.value); }
    if (bitRead(command.action, 4)) { armito.writeDoll(command.value); }
    if (bitRead(command.action, 5)) { armito.writeGrip(command.value); }

    break;

  default:
    break;
  }
}

void Xplorer::sendDataSensors() {
  int gsc = sens.getGasConcentration();
  if (data.gasConcentration != gsc)
  {
    data.gasConcentration = gsc;
    send(command.gas(gsc));
  }
  
  int humi = sens.getHumidity();
  if (data.humidity =! humi)
  {
    data.humidity = humi;
    send(command.humidity(humi));
  }
  
  int temp = sens.getTemperature();
  if (data.temperature =! temp)
  {
    data.temperature = temp;
    send(command.temperature(temp));
  }
  
  int d = sens.getDistance();
  if (data.distance =! d)
  {
    data.distance = d;
    send(command.distance(d));
  }
}

void Xplorer::send(Command cmd) {
  byte buf[3];
  buf[0] = (cmd.cmd << 6) | cmd.action;
  buf[1] = cmd.value;
  buf[2] = '\0';

  ble.write(buf, 3); 
}
