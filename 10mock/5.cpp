#include <iostream>
#include <string>
using namespace std;

/* sum of digits
N = 834

sum = 0

834 % 10 = 4 -> add 4
834 / 10 = 83

83 % 10 = 3 -> add 3
83 / 10 = 8

8 % 10 = 8 -> add 8
8 / 10 = 0 -> break
*/

int DigitsSum(int n) {
    int sum = 0;
    while (n > 0) {
        sum += n % 10;
        n /= 10;
    }
    return sum;
}

int main() {
    int N, A, B;
    cin >> N >> A >> B;
    int total = 0;
    for (int n=1; n<=N; n++) {
        int sum = DigitsSum(n);
        if (A <= sum && sum <= B) {
            total += n;
        }
    }
    cout << total << endl;
}



/*
int main() {
    int N;
    int A, B;
    std::cin >> N >> A >> B;
    
    int res = 0;
    for (int n=1; n<=N; n++) {
        string n_str = to_string(n);
        
        int sum = 0;
        for (int i=0; i<n_str.size(); i++) {
            string tmp = "";
            tmp += n_str[i];
            int num = std::stoi(tmp);
            sum += num;
        }
        if (A <= sum && sum <= B) {
            res += n;
        } 
    }
    cout << res << endl;
}
*/
