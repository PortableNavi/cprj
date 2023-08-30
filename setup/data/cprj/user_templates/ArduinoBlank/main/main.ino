int LED_STATUS;


void setup()
{
  pinMode(LED_BUILTIN, OUTPUT);
  LED_STATUS = HIGH;

}


void loop()
{
  digitalWrite(LED_BUILTIN, LED_STATUS);
  LED_STATUS = !LED_STATUS;
  delay(500);
}
