#include <iostream>
#include <string>

using std::cout;
using std::endl;
using std::string;


int star1(string &str){
    auto size = str.size();
    if(!size || size == 1) return 0;
    str += str[0];
    int s = 0;
    
    for(size_t i = 0; i < size; ++i){
        if(str[i] == str[i+1]){
            s += str[i] - '0';
        }
    }
    return s;
}

int main(){

    string num1 = "1212";
    string num2 = "1221";
    string num3 = "123425";
    string num4 = "123123";
    string num5 = "12131415";
    cout << num1 << " : " << star2(num1) << endl;
    cout << num2 << " : " << star2(num2) << endl;
    cout << num3 << " : " << star2(num3) << endl;
    cout << num4 << " : " << star2(num4) << endl;
    cout << num5 << " : " << star2(num5) << endl;
    cout << star2(input) << endl;

    return 0;
}
