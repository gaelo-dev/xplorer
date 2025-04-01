#ifndef sensors_h
#define sensors_h

#include "Arduino.h"
#include "DHT.h"

class Data
{
public:
  int gasConcentration;
  int humidity;
  int temperature;
  int distance;

  Data(int gas, int humidity, int temperature, int distance);
};

class Sensors
{
private:
  int pinGas;
  int pinEcho;
  int pinTrigger;
  DHT dht;

public:
  Sensors(int pinGas, int pinEcho, int pinTrigger, int pinDHT);
  void begin();

  int getGasConcentration();
  int getHumidity();
  int getTemperature();
  int getDistance();
};

#endif