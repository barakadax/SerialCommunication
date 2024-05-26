#include <iostream>
#include <fcntl.h>
#include <termios.h>
#include <unistd.h>
#include <string>
#include <locale>
#include <codecvt>

using namespace std;

int main() {
    string portName = "/dev/pts/3";
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

    string dataToWrite;
    wstring_convert<codecvt_utf8_utf16<wchar_t>> converter;
    int bytesWritten;

    while (true) {
        cout << "Enter a message to send: ";
        cin >> dataToWrite;

        wstring utf16Data = converter.from_bytes(dataToWrite);

        bytesWritten = write(serialPort, utf16Data.c_str(), utf16Data.size() * sizeof(wchar_t));

        if (bytesWritten == -1) {
            cerr << "Error writing to serial port" << endl;
        }
    }
    
    close(serialPort);
    return 0;
}
