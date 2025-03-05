#include "motors.h"

void Motors::attachMotorA(int en, int in1, int in2) {
    pinSpeedA = en; pinMotorA1 = in1; pinMotorA2 = in2;
    Serial.println(pinSpeedA); Serial.println(pinMotorA1); Serial.println(pinMotorA2);
    
    pinMode(pinSpeedA, OUTPUT);
    pinMode(pinMotorA1, OUTPUT);
    pinMode(pinMotorA2, OUTPUT);
}

void Motors::attachMotorB(int en, int in1, int in2) {
    pinSpeedB = en; pinMotorB1 = in1; pinMotorB2 = in2;
    Serial.println(pinSpeedB); Serial.println(pinMotorB1); Serial.println(pinMotorB2);
    
    pinMode(pinSpeedB, OUTPUT);
    pinMode(pinMotorB1, OUTPUT);
    pinMode(pinMotorB2, OUTPUT);
}

void Motors::setSpeed(int speed){
    analogWrite(pinSpeedA, speed);
    analogWrite(pinSpeedB, speed);
}

void Motors::forward(){
    digitalWrite(pinMotorA1, LOW);
    digitalWrite(pinMotorA2, HIGH);

    digitalWrite (pinMotorB1, LOW);
    digitalWrite (pinMotorB2, HIGH);
}

void Motors::backward(){
    digitalWrite(pinMotorA1, HIGH);
    digitalWrite(pinMotorA2, LOW);

    digitalWrite (pinMotorB1, HIGH);
    digitalWrite (pinMotorB2, LOW);
}

void Motors::rightward(){  
    digitalWrite(pinMotorA1, HIGH);
    digitalWrite(pinMotorA2, LOW);

    digitalWrite (pinMotorB1, LOW);
    digitalWrite (pinMotorB2, HIGH);
}

void Motors::leftward(){
    digitalWrite(pinMotorA1, LOW);
    digitalWrite(pinMotorA2, HIGH);

    digitalWrite (pinMotorB1, HIGH);
    digitalWrite (pinMotorB2, LOW);
}

void Motors::stop(){
    digitalWrite(pinMotorA1, LOW);
    digitalWrite(pinMotorA2, LOW);

    digitalWrite (pinMotorB1, LOW);
    digitalWrite (pinMotorB2, LOW);
}
