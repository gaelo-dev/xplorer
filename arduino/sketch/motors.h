#ifndef motors_h
#define motors_h

#include "Arduino.h"

class Motors 
{
private: 
// Pin motor A
int pinMotorA1;
int pinMotorA2;
int pinSpeedA; 

// Motor B
int pinMotorB1;
int pinMotorB2;
int pinSpeedB;

public:
void attachMotorA(int en, int in1, int in2);
void attachMotorB(int en, int in1, int in2);

void setSpeed(int speed);

void forward();
void backward();
void rightward();
void leftward();
void stop();
};

#endif