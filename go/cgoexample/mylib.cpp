#include <string>
#include <iostream>

extern "C" {
    const char* SayHello(const char* name) {
        static std::string result;
        result = "Hello, " + std::string(name) + "!";
        return result.c_str();
    }
}
