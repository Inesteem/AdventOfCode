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
struct stats{
 int min, max, magic, ring;   
 int side;   
};


stats get_stats(int num){
    stats s;
    if(num <= 1) return s;
    s.ring = get_ring(num);

    s.max = s.min = s.magic = (2*s.ring+1);
    s.min -= 2;
    s.max *= s.max;
    s.min *= s.min;
    ++(s.min);



    return s;
}


bool is_edge(int idx) {
    if(!idx) return true;
    int num = idx + 1;

    auto ring = get_ring(num);
    int max, magic;
    max =  magic = (2*ring+1);
    max *= max;

    for(int i = 0; i < 4; ++i ){
        if(num == (i + max - i * magic)) return true;

    }
    return false; 
}

int main(){
    vector<int> init = {1,1,2,4,5,10,11,23,25};

    vector<int> res(10000);
    for(int i = 0; i < init.size(); ++i){
        res[i] = init[i];
    }

   for(int i = init.size(); i < res.size(); ++i){
        auto s = get_stats(i + 1);
        if(is_edge(i)){
            int diag = s.ring * 8;
            res[i] = res[i-1];
            if(i+1 == s.max)                        res[i] += res[i - diag - 8]; 
            else if(i+1 == s.max - s.magic + 1)     res[i] += res[i - diag - 6]; 
            else if(i+1 == s.max - 2 * s.magic + 2) res[i] += res[i - diag - 4]; 
            else                                    res[i] += res[i - diag - 2]; 
        }
         else if(is_edge(i-1)){
            res[i] = 2 * res[i-1] - res[i-2];
            if(i+1 == s.max)                        res[i] += res[i - diag - 8 + 1]; 
            else if(i+1 == s.max - s.magic + 1)     res[i] += res[i - diag - 6 + 1]; 
            else if(i+1 == s.max - 2 * s.magic + 2) res[i] += res[i - diag - 4 + 1]; 
            else                                    res[i] += res[i - diag - 2 + 1]; 

        }

    } 
/
    return 0;
}
