#include <iostream>
#include <string>
#include <vector>
#include <cmath>
using std::cout;
using std::endl;
using std::string;
using std::vector;


template<class T> void print(vector<T> vec){
    auto size = vec.size();
    cout << "{ ";
    for(int i = 0; i < size-1; ++i)
        cout << vec[i] << " , ";
    cout << vec[size-1] << " }" << endl;

}


int get_ring(int num){
    if(num <= 1) return 0;

    auto wurzel = sqrt(num);
    int ret = 0;
    if(!((int)wurzel & 0x1)){//ungerade
        ret = 1 + (wurzel - 1)/2;
    }
    else if ( !((float)((int) wurzel) - wurzel)){ ret = (wurzel-1)/2;}
    else ret =  wurzel/2 + 1;
    
    return ret;
}
//1,3,5,7,9
//0,1,2,3,4
vector<int> get_liners(int num) {
    vector<int> ret;
    if(num <= 1) return ret;

    auto ring = get_ring(num);
    int min, max, magic;

    max = min = magic = (2*ring+1);
    min -= 2;
    max *= max;
    min *= min;
    ++min;

//    cout << num << " -> { " << min << " : " << max << " } " << endl;

    for(int i = 0; i < 4; ++i){
        ret.push_back( max - (magic * i -i + ring));
    }
    return ret;
}



int get_ham(int num) {
    if(num <= 1) return 0;

    auto liners = get_liners(num);

    int dist = num;

    for(auto &elem : liners){
        int tmp = abs(elem-num);
        if(tmp < dist) dist = tmp;

    }    

    return dist + get_ring(num);
}

int main(){
    /*
    for(int i = 0; i < 9*9;  i+=4) {
        cout << i << " : " << get_ring(i) << "\t" << i+1 << " : " << get_ring(i+1) << "\t";
        cout << i+2 << " : " << get_ring(i+2) << "\t" << i+3 << " : " << get_ring(i+3) << endl;
    }*/
   /*
   print( get_liners(5) );
   print( get_liners(25));
   print( get_liners(45));
   print( get_liners(50));
   */
    
//    for(int i = 0; i < 49; ++i)
//        if(get_ham(i) == 4) cout << i << " : " << get_ham(i) << endl;
    cout << get_ham(12) << endl;
    cout << get_ham(23) << endl;
    cout << get_ham(1024) << endl;
    cout << get_ham(347991) << endl;

    return 0;
}
