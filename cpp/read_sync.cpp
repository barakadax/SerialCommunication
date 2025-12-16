#include <iostream>
#include <fcntl.h>
#include <unistd.h>
#include <vector>
#include <codecvt>
#include <locale>
#include <cstdint>

int main() {
    const char* port = "/dev/pts/2";
    int fd = open(port, O_RDONLY);
    
    if (fd < 0) {
        std::cerr << "Error: Could not open " << port << std::endl;
        return 1;
    }

    std::wstring_convert<std::codecvt_utf8_utf16<char16_t>, char16_t> converter;

    while (true) {
        uint8_t buffer[4096];
        ssize_t bytesRead = read(fd, buffer, sizeof(buffer));
        
        if (bytesRead > 0) {
            std::vector<char16_t> u16data;
            u16data.reserve(bytesRead / 2);

            for (ssize_t i = 0; i < (bytesRead & ~1); i += 2) {
                char16_t codeUnit = buffer[i] | (buffer[i+1] << 8);
                u16data.push_back(codeUnit);
            }

            try {
                if (!u16data.empty()) {
                    std::string utf8Str = converter.to_bytes(u16data.data(), u16data.data() + u16data.size());
                    if (utf8Str.size() >= 3 && 
                        (unsigned char)utf8Str[0] == 0xEF && 
                        (unsigned char)utf8Str[1] == 0xBB && 
                        (unsigned char)utf8Str[2] == 0xBF) {
                        utf8Str.erase(0, 3);
                    }
                    
                    std::cout << "data received: " << utf8Str << std::endl;
                }
            } catch (const std::exception& e) {
                std::cerr << "UTF conversion error" << std::endl;
            }
        } else if (bytesRead < 0) {
            std::cerr << "Read error" << std::endl;
        }
    }

    close(fd);
    return 0;
}
