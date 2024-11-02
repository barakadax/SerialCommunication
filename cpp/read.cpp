#include <iostream>
#include <fcntl.h>
#include <termios.h>
#include <unistd.h>
#include <string>
#include <locale>
#include <codecvt>

using namespace std;

int main() {
    string portName = "/dev/pts/6";
    int serialPort = open(portName.c_str(), O_WRONLY | O_NOCTTY);

    if (serialPort == -1) {
        cerr << "Error opening serial port: " << portName << endl;
        return 1;
    }

    termios tty;
    tcgetattr(serialPort, &tty);

    cfsetispeed(&tty, B115200);
    cfsetospeed(&tty, B115200);

    tty.c_cflag &= ~PARENB;
    tty.c_cflag &= ~CSTOPB;
    tty.c_cflag &= ~CSIZE;
    tty.c_cflag |= CS8;
    tty.c_cflag &= ~CRTSCTS;

    tty.c_cc[VMIN] = 1;
    tty.c_cc[VTIME] = 0;

    tcsetattr(serialPort, TCSANOW, &tty);

    char buffer[1024];

    while (1) {
		ssize_t length = read(serialPort, &buffer, sizeof(buffer));

		if (length > 1) {
			buffer[length] = '\0';
			cout << buffer;
		}
	}
    
    close(serialPort);
    return 0;
}
