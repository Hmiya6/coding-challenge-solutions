#include <iostream>
#include <string>
#include <vector>

int main() {
    std::vector<int> input;
    int num;
    std::cin >> num;
    for (int i=0; i<num; i++) {
        int tmp;
        std::cin >> tmp;
        input.push_back(tmp);
    }
    
    int counter = 0;
    while (true) {
        bool flag = true;
        for (int i=0; i<input.size(); i++) {
            if ( input[i] % 2 == 0) {
                input[i] = input[i]/2;
                continue;
            } else {
                flag = false;
                break;
            }
        }
        if (flag) {
            counter++;
        } else {
            break;
        }
    }

    std::cout << counter << std::endl;
}
