build:
	g++ src/main.cpp src/glad.c -o application -I./include -L./lib -lglfw3 -lGL -lX11 -lpthread -lXrandr -lXi -ldl

run: build
	./application

build-ex1:
	g++ src/ex1.cpp src/glad.c -o application -I./include -L./lib -lglfw3 -lGL -lX11 -lpthread -lXrandr -lXi -ldl

run-ex1: build-ex1
	./application
