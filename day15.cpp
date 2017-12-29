#include "helper.h"
//#include <boost/functional/hash.hpp>
const long divider = 2147483647;

class Generator{
    long value;
    long factor;
public:
    Generator(long value, long factor):value(value), factor(factor){}
    
    void calc(){
        value *= factor;
        value %= divider;

    }
    long get_val_4(){
        do {
            calc();
        } while(value & 0x3);  
        return value;
    }
    long get_val_8(){
        do {
            calc();
        } while(value & 0x7);  
        return value;
    }

};



int main(int argc, char *argv[]){
 //   Generator A(65,16807);
    Generator A(116,16807);
 //   Generator B(8921,48271);
    Generator B(299,48271);
    int num = 0;
//    for(int i = 0; i < 40000000; ++i){
    for(int i = 0; i <   5000000; ++i){
        auto aval = A.get_val_4();
        auto bval = B.get_val_8();
        num += ((aval & 0xFFFF) == (bval & 0xFFFF));
  }
    cout << num << endl;
	return 0;
}
