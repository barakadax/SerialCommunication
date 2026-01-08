#include <iostream>
#include <fcntl.h>
#include <unistd.h>
#include <thread>
#include <chrono>
#include <vector>
#include <codecvt>
#include <locale>
#include <cstdint>

int main(int argc, char* argv[]) {
    if (argc < 2) {
        std::cerr << "Usage: " << argv[0] << " <port>" << std::endl;
        return 1;
    }
    const char* port = argv[1];
    int fd = open(port, O_RDONLY | O_NONBLOCK);
    
    if (fd < 0) {
        std::cerr << "Error: Could not open " << port << std::endl;
        return 1;
    }

    char buffer[4096];
    ssize_t bytesRead = 0;

    while (true) {
        std::this_thread::sleep_for(std::chrono::milliseconds(300));
        
        bytesRead = read(fd, buffer, sizeof(buffer) - 1);
        
        if (bytesRead > 0) {
            buffer[bytesRead] = '\0';
            std::cout << "data received: " << buffer << std::endl;
        } else if (bytesRead < 0 && errno != EAGAIN && errno != EWOULDBLOCK) {
            std::cerr << "Read error" << std::endl;
        }
    }

    close(fd);
    return 0;
}
