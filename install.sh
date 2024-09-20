#!/bin/bash

TARGET_FILE="$(ls *.py)"
TARGET_NAME="${TARGET_FILE%.*}"
DIST_DIR="$(pwd)/dist"
BUILD_DIR="$(pwd)/build"

if [ "$1" == "-i" ]; then
	if [ ! -d "$DIST_DIR" ] || [ ! -d "$BUILD_DIR" ]; then
		pyinstaller --onefile --name $TARGET_NAME $TARGET_FILE
		while [ ! -f $DIST_DIR/$TARGET_NAME ]; do
			sleep 1
		done
	fi

	if [ ! -d "$DIST_DIR" ]; then
		echo "Error: The directory $DIST_DIR does not exist."
		exit 1
	fi

	if [ ! -d "$BUILD_DIR" ]; then
		echo "Error: The directory $BUILD_DIR does not exist."
		exit 1
	fi

	if ! grep -q "$DIST_DIR" ~/.bashrc; then
  		echo "export PATH=\$PATH:$DIST_DIR" >> ~/.bashrc
		echo ""
		echo "The directory $DIST_DIR has been added to the PATH."
	else
		echo ""
		echo "The directory $DIST_DIR is already in the PATH."
		exit 0
	fi

elif [ "$1" == "-r" ]; then

	if [ ! -d "$DIST_DIR" ]; then
		echo "Error: The directory $DIST_DIR does not exist."
		exit 1
	fi

	if [ ! -d "$BUILD_DIR" ]; then
		echo "Error: The directory $BUILD_DIR does not exist."
		exit 1
	fi

	SPEC_FILE=$(ls *.spec)
	if [ -z "$SPEC_FILE" ]; then
		echo "Error: The spec file does not exist."
		exit 1
	fi

	rm -r $DIST_DIR $BUILD_DIR $SPEC_FILE

	if grep -q "$DIST_DIR" ~/.bashrc; then
		sed -i "\|$DIST_DIR|d" ~/.bashrc
		echo "The directory has been removed from the PATH."
	else
		echo "The directory $DIST_DIR is not in the PATH."
		exit 0
	fi

else
	echo "Usage: ./install.sh [ -i | -r ]"
	echo "Options:"
	echo "  -i  Install the program."
	echo "  -r  Remove the program."
	exit 1
fi

