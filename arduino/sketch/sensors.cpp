#include "sensors.h"

Data::Data(int gas, int humidity, int temperature, int distance): gasConcentration(gas), humidity(humidity), temperature(temperature), distance(distance) {}

Sensors::Sensors(int pinGas, int pinEcho, int pinTrigger, int pinDHT): pinGas(pinGas), pinEcho(pinEcho), pinTrigger(pinTrigger), dht(pinDHT, DHT11) {}

void Sensors::begin() {
  dht.begin();
  pinMode(pinTrigger, OUTPUT); 
  pinMode(pinEcho, INPUT); 
  digitalWrite(pinTrigger, LOW);
}

int Sensors::getGasConcentration() {
  return analogRead(pinGas);
}

int Sensors::getHumidity() {
  return dht.readHumidity();
}

int Sensors::getTemperature() {
  return dht.readTemperature();
}

int Sensors::getDistance() {
  long t; // tiempo que demora en llegar el eco

  digitalWrite(pinTrigger, HIGH);
  delayMicroseconds(10); // Enviamos un pulso de 10us
  digitalWrite(pinTrigger, LOW);

  t = pulseIn(pinEcho, HIGH); // Obtenemos el ancho del pulso
  return t/59; // cm
}
