project_name := ArduinoBlank
board_type := arduino:avr:uno
libraries := $(patsubst %,--library %, $(shell ls -d lib/*/))
port := $(shell ls /dev/ttyACM* | awk {'print $1'})


.PHONY: run
run: compile
	arduino-cli upload -b $(board_type) main

.PHONY: upload
upload:
	arduino-cli upload -b $(board_type) --port $(port) main

.PHONY: compile
compile:
	arduino-cli compile -b $(board_type) $(libraries) main


.DEFAULT_GOAL: run

