#ifndef xplorer_h
#define xplorer_h

#include "Arduino.h"
#include <SoftwareSerial.h>

#include "motors.h"
#include "arm.h"

class Command
{
public:
  int cmd = 0;
  byte action = 0;
  int value = 0;

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
  Command command;

  void execute();

public:
  Xplorer(int rx, int tx);
  // void setup(int enA, int inA1, int inA2, int enB, int inB1, int inB2, int pinBase, int pinElbow, int pinRest, int pinShoulder, int pinDoll, int pinGrip);
  void attachMotors(int enA, int inA1, int inA2, int enB, int inB1, int inB2);
  void attachArm(int pinElbow, int pinRest, int pinShoulder, int pinDoll, int pinGrip)
  void begin();
  void listen();
};

#endif