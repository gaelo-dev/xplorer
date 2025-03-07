#include <Servo.h>;

class Carrito {

private: 
  // Motor A
  int pinMotorA1;
  int pinMotorA2;
  int pinSpeedA; 
  
  // Motor B
  int pinMotorB1;
  int pinMotorB2;
  int pinSpeedB;

public:
  void attachMotorA(int en, int in1, int in2){
    pinSpeedA = en; pinMotorA1 = in1; pinMotorA2 = in2;
    Serial.println(pinSpeedA); Serial.println(pinMotorA1); Serial.println(pinMotorA2);
    
    pinMode(pinSpeedA, OUTPUT);
    pinMode(pinMotorA1, OUTPUT);
    pinMode(pinMotorA2, OUTPUT);
  }

  void attachMotorB(int en, int in1, int in2){
    pinSpeedB = en; pinMotorB1 = in1; pinMotorB2 = in2;
    Serial.println(pinSpeedB); Serial.println(pinMotorB1); Serial.println(pinMotorB2);
    
    pinMode(pinSpeedB, OUTPUT);
    pinMode(pinMotorB1, OUTPUT);
    pinMode(pinMotorB2, OUTPUT);
  }

  void setSpeed(int speed){
    analogWrite(pinSpeedA, speed);
    analogWrite(pinSpeedB, speed);
  }

  void forward(){
    digitalWrite(pinMotorA1, LOW);
    digitalWrite(pinMotorA2, HIGH);

    digitalWrite (pinMotorB1, LOW);
    digitalWrite (pinMotorB2, HIGH);
  }

  void backward(){
    digitalWrite(pinMotorA1, HIGH);
    digitalWrite(pinMotorA2, LOW);

    digitalWrite (pinMotorB1, HIGH);
    digitalWrite (pinMotorB2, LOW);
  }

  void rightward(){  
    digitalWrite(pinMotorA1, LOW);
    digitalWrite(pinMotorA2, LOW);

    digitalWrite (pinMotorB1, LOW);
    digitalWrite (pinMotorB2, HIGH);
  }

  void leftward(){
    digitalWrite(pinMotorA1, LOW);
    digitalWrite(pinMotorA2, HIGH);

    digitalWrite (pinMotorB1, LOW);
    digitalWrite (pinMotorB2, LOW);
  }

  void stop(){
    digitalWrite(pinMotorA1, LOW);
    digitalWrite(pinMotorA2, LOW);

    digitalWrite (pinMotorB1, LOW);
    digitalWrite (pinMotorB2, LOW);    
  }
};

class Brazo
{
private:
  Servo base;
  Servo elbow;
  Servo rest;
  Servo shoulder;
  Servo doll;
  Servo grip;

public:
  void attach(int pinBase, int pinElbow, int pinRest, int pinShoulder, int pinDoll, int pinGrip) {
    base.attach(pin); elbow.attach(pinElbow); rest.attach(pinRest)
    shoulder.attach(pinShoulder); doll.attach(pinDoll); grip.attach(pinGrip);
  }

  void writeBase(int grades) {
    base.write(grades);
  }

  void writeElbow(int grades) {
    elbow.write(grades);
  }

  void writeRest(int grades) {
    rest.write(grades);
  }

  void writeShoulder(int grades) {
    shoulder.write(grades);
  }

  void writeDoll(int grades) {
    doll.write(grades);
  }

  void writeGrip(int grades) {
    grip.write(grades);
  }
};

Carrito rojito;
Brazo manita;

void setup ()
{
  Serial.begin(9600);
  rojito.attachMotorA(4, 41, 43);
  rojito.attachMotorB(3, 45, 47);

  manita.attach(5, 6, 7, 8, 9, 10);
}

void loop()
{
  // Movimiento del brazo
  manita.writeBase(90); delay(1000); manita.writeElbow(90); delay(1000)
  manita.writeRest(90); delay(1000); manita.writeShoulder(90); delay(1000)
  manita.writeDoll(90); delay(1000); manita.writeGrip(90); delay(1000)

  // Movimiento del carro
  rojito.setSpeed(200);
  
  rojito.forward();
  delay(2000);
  rojito.stop();
  delay(2000);

  rojito.backward();
  delay(2000);
  rojito.stop();
  delay(2000);

  rojito.leftward();
  delay(3000);
  rojito.stop();
  delay(2000);

  rojito.rightward();
  delay(3000);
  rojito.stop();
  delay(2000);

  // Movimiento del brazo
  manita.writeBase(180); delay(1000); manita.writeElbow(180); delay(1000)
  manita.writeRest(180); delay(1000); manita.writeShoulder(180); delay(1000)
  manita.writeDoll(180); delay(1000); manita.writeGrip(180); delay(1000)
}
