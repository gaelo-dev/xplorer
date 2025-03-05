#ifndef xplorer_h
#define xplorer_h

#include "Arduino.h"
#include <SoftwareSerial.h>

#include "motors.h"

class Xplorer
{
private:
  SoftwareSerial ble;
  Motors mots;

  int command = 0;
  byte action = 0;
  int value = 0;

  void execute();

public:
  Xplorer(int rx, int tx);
  void setup(int enA, int inA1, int inA2, int enB, int inB1, int inB2);
  void listen();
};

#endif