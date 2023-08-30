CC 						:= gcc
CFLAGS 				:= -Wall
INCLUDE_DIRS 	:= include


SRC_FILES 		:= main.c


OBJ_FILES := $(patsubst %.c,build/obj/%.o,$(SRC_FILES))
INCLUDES 	:= $(patsubst %,-I%,$(INCLUDE_DIRS))


.PHONY: default
.DEFAULT_GOAL: default
default: build


.PHONY: makedirs
makedirs:
	mkdir -p build/obj


build/obj/%.o: src/%.c
	$(CC) -c $< -o $@ $(CFLAGS) $(INCLUDES)


@[name]: $(OBJ_FILES)
	$(CC) $^ -o build/$@ $(CFLAGS)


.PHONY: clean
clean:
	rm -r build ||:

.PHONY: build
build: makedirs @[name]


.PHONY: run
run:
	build/@[name]

