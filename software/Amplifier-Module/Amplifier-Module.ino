/*
 *
 */

 #include <Wire.h>

// Pins from the rotary encoder subsystem
const int volClockwisePin = 4;
const int volCounterClockwisePin = 5;

// I2C Pins
const int I2C_SDA = 21;
const int I2C_SCL = 22;

// I2C control of the amp
const int AMP_CONTROL_W_ADDR= 0x98;  // Write address
const int AMP_CONTROL_R_ADDR= 0x99;  // Read address
const byte AMP_R0_ADDR = 0x00;  // Register 0 sets the page for register addressses
const byte AMP_R60_ADDR = 0x3C; // P0-R60 control the tracking of the volume control
const byte AMP_R62_ADDR = 0x3E; // P0-R60 control the tracking of the volume control for SPK_OUTA

volatile int8_t direction = 0; // Volume up of down
const int8_t VOL_UP = 1;
const int8_t VOL_DOWN = -1;
const int8_t VOL_INC = 1; // How much to change the volume in each step

void setup() {

  attachInterrupt(digitalPinToInterrupt(volClockwisePin), volIncreasingISR, RISING);
  attachInterrupt(digitalPinToInterrupt(volCounterClockwisePin), volDecreasingISR, RISING);

  Wire.begin(I2C_SDA, I2C_SCL); // Set up I2C and pins
  setVolumeTracking(); // SPK_OUTA volume controls the volume for both channels.
  Serial.begin(115200);
}

void loop() {

  // Process Volume
  if (direction == VOL_UP) {
    changeVolume(VOL_INC);
    direction = 0;
    Serial.println("VOL++");
  } else if (direction == VOL_DOWN ){
    changeVolume(-VOL_INC);
    direction = 0;
    Serial.println("VOL--");
  }

}

void volIncreasingISR () {
  direction = VOL_UP;
}

void volDecreasingISR () {
  direction = VOL_DOWN;
}

void setVolumeTracking() {
  // Set the page to 0
  setPage(0x00);

  // Now write to P0-R60 to set tracking of the volume
  Wire.beginTransmission(AMP_CONTROL_W_ADDR);
  Wire.write(AMP_R60_ADDR); //Write to register 60 to control tracking
  Wire.write(0x02);  // Select channel SPK_OUTA to control volume
  Wire.endTransmission();
}


/* Change the volume by the specifed increment
*/
void changeVolume(int8_t increment) {

  Serial.print("changeVolume("); Serial.print(increment); Serial.println(")");
  // Set the page to 0
  setPage(0x00);

  //Specify the volume register
  Wire.beginTransmission(AMP_CONTROL_W_ADDR);
  Wire.write(AMP_R62_ADDR);
  Wire.endTransmission(false); // Keep the  connection open (repeat)

  // Now read the current volume
  Wire.requestFrom(AMP_CONTROL_R_ADDR, 1);
  byte currentVolume = 0;
  currentVolume = Wire.read();

  //DEBUG
  Serial.print("Current Volume:");
  Serial.println(currentVolume);

  int newVolume = currentVolume - increment;
  if (newVolume >= 0 && newVolume <= 254) {
    uint8_t digitalVolume = (uint8_t)newVolume;
    Wire.beginTransmission(AMP_CONTROL_W_ADDR);
    Wire.write(digitalVolume);
    Wire.endTransmission(true); // Stop the connection
  } else if (newVolume == 0) {
    // Send the max volume again to bring everything to
    // an orderly end
    Wire.beginTransmission(AMP_CONTROL_W_ADDR);
    Wire.write(0);
    Wire.endTransmission(true);
  } else if (newVolume < 254){
    // Send the min volume again to bring everything to
    // an orderly end
    Wire.beginTransmission(AMP_CONTROL_W_ADDR);
    Wire.write(254);
    Wire.endTransmission(true);
  }

}

void setPage(byte page) {
Serial.print("setPage(");Serial.print(page); Serial.println(")");
  
  Wire.beginTransmission(AMP_CONTROL_W_ADDR);
  Wire.write(AMP_R0_ADDR); //Write to register 0 to select the register page
  Wire.write(page);  // Select page
  Wire.endTransmission();
}
