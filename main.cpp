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
    long get_val(){
        return value;
    }
};



int main(int argc, char *argv[]){
    //Generator A(65,16807);
    Generator A(116,16807);
    //Generator B(8921,48271);
    Generator B(299,48271);
    int num = 0;
    for(int i = 0; i < 40000000; ++i){
        A.calc(); B.calc();
        auto aval = A.get_val();
        auto bval = B.get_val();
//        cout << A.get_val() << "   " << B.get_val() << endl; 
        num += ((aval & 0xFFFF) == (bval & 0xFFFF));
   }
    cout << num << endl;
	return 0;
}
