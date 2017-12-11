#include <iostream>
#include <string>
#include <vector>
#include <cmath>
#include <algorithm>
#include <fstream>
#include <sstream>

#define STAR9

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
int main(int argc, char *argv[]){
    vector<int> data = {0,2,7,0};
    int cnt = 0;
  
    int start = 2;

   // int end = (start + data[start]) % num;
    int to_add = 7 / data.size();
    cout << "to_add " << to_add << endl;
    int end = (start + 7) % data.size();
    data[start] = 0;
    for(int i = 0; i < data.size(); ++i){
        
        if(end > start){
            if(i <= start){
                data[i] += to_add - 1;
            } else { 
                data[i] += to_add;
            }
        } else { 
            if(i > end && i <= start){
                data[i] += to_add - 1;
            } else {
                data[i] += to_add;

            }

        }
   }

    print(data);
   	return 0;
}
