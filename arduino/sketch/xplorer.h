#ifndef xplorer_h
#define xplorer_h

#include "Arduino.h"
#include <SoftwareSerial.h>

#include "arm.h"
#include "motors.h"
#include "sensors.h"

class Command
{
public:
  int cmd;
  byte action;
  int value;

  Command(int cmd, byte act, int val);
  
  Command humidity(int val);
  Command temperature(int value);
  Command gas(int value);
  Command distance(int value);
};

class Xplorer
{
private:
  SoftwareSerial ble;
  Motors mots;
  Arm armito;
  Sensors sens;

  Data data;
  Command command;

  void execute();
  void send(Command cmd);

public:
  Xplorer(int rx, int tx, int pinGas, int pinEcho, int pinTrigger, int pinDHT);
  void attachMotors(int enA, int inA1, int inA2, int enB, int inB1, int inB2);
  void attachArm(int pinBase, int pinElbow, int pinRest, int pinShoulder, int pinDoll, int pinGrip);
  void begin();

  void listen();
  void sendDataSensors();
};

#endif