#include <iostream>
#include <string>

int main() {
    std::string s;
    std::cin >> s;
    
    int counter = 0;
    for (int i=0; i<3; i++) {
        if (s[i] == '1') counter++;
    }

    std::cout << counter << std::endl;
}
