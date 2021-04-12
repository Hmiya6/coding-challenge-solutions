#include <iostream>
#include <vector>
#include <string>
using namespace std;



int main() {
    int A, B, C, X;
    cin >> A >> B >> C >> X;
    int res = 0;
    for (int a = 0; a<=A; a++) {
        for (int b = 0; b<=B; b++) {
            for (int c=0; c<=C; c++) {
                int total = 500*a + 100*b + 50*c;
                if (total == X) {
                    res++;
                }
            }
        }
    }

    cout << res << endl;
}


/* my answer

int main() {
    int num500, num100, num50, target;
    cin >> num500 >> num100 >> num50 >> target;

    int counter = 0;
    for (int i=0; i<=num500; i++) {
        int sum = 0;
        // cout << sum << endl;
        sum += 500*i;
        if (sum == target) {
            // cout << sum << endl;
            ++counter;
        } 
        if (sum >= target) {
            break;
        }
        int sum500 = sum;
        for (int j=0; j<=num100; j++) {
            // cout << i << j << endl;
            sum = sum500;
            sum += 100*j;
            if (sum == target) {
                // cout << sum << endl;
                ++counter;
            }
            if (sum >= target) {
                break;
            }
            int sum500100 = sum;
            for (int k=0; k<=num50; k++) {
                // cout << i << j << k << endl;
                sum = sum500100;
                sum += 50*k;
                if (sum == target) {
                    // cout << sum << endl;
                    ++counter;
                }
                if (sum >= target+1) {
                    break;
                }
            }
        }
    }
    cout << counter << endl;
}
*/
