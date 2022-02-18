#include <Arduino.h>

const int ADDR_O = 19;
const int ADDR_1 = 18;
const int ADDR_2 = 5;

const int SRC_EN = 4;

void set_source_channel(byte);

void setup()
{
  Serial.begin(115200);

  pinMode(ADDR_O, OUTPUT);
  pinMode(ADDR_1, OUTPUT);
  pinMode(ADDR_2, OUTPUT);
  pinMode(SRC_EN, OUTPUT);

  Serial.println("Enter source [0..7]");
}

void loop()
{
  byte source_channel = 0;


  if (Serial.available())
  {
    char ch = Serial.read();

    Serial.print(ch);

    if (isDigit(ch))
    {
      source_channel = (ch - '0');
      if (source_channel < 8 && source_channel >= 0)
      {
        set_source_channel(source_channel);
        Serial.println();
        Serial.println("Enter source ['0..7']");
      }
      else
      {
        Serial.println();
        Serial.println("Incorrect source. Please reenter.");
        Serial.println("Enter source ['0..7']");
      }
    }
  }
}

void set_source_channel(byte channel)
{
  digitalWrite(SRC_EN, LOW);
  delay(10);

  digitalWrite(ADDR_O, bitRead(channel, 0));
  digitalWrite(ADDR_1, bitRead(channel, 1));
  digitalWrite(ADDR_2, bitRead(channel, 2));

  delay(10);
  digitalWrite(SRC_EN, HIGH);
}
