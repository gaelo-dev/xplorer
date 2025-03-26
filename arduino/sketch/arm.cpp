#include "arm.h"

void Arm::attach(int pinBase, int pinElbow, int pinRest, int pinShoulder, int pinDoll, int pinGrip) {
  base.attach(pinBase); elbow.attach(pinElbow); rest.attach(pinRest);
  shoulder.attach(pinShoulder); doll.attach(pinDoll); grip.attach(pinGrip);
}

void Arm::writeBase(int grades) { base.write(grades); }
void Arm::writeElbow(int grades) { elbow.write(grades); }
void Arm::writeRest(int grades) { rest.write(grades); }
void Arm::writeShoulder(int grades) { shoulder.write(grades); }
void Arm::writeDoll(int grades) { doll.write(grades); }
void Arm::writeGrip(int grades) { grip.write(grades); }
