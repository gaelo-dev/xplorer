#ifndef arm_h
#define arm_h

#include "Arduino.h"
#include "Servo.h"

class Arm
{
private:
  Servo base;
  Servo elbow;
  Servo rest;
  Servo shoulder;
  Servo doll;
  Servo grip;

public:
  void attach(int pinBase, int pinElbow, int pinRest, int pinShoulder, int pinDoll, int pinGrip);

  void writeBase(int grades);
  void writeElbow(int grades);
  void writeRest(int grades);
  void writeShoulder(int grades);
  void writeDoll(int grades);
  void writeGrip(int grades);
};

#endif