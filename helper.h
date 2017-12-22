#ifndef HELPER_H
#define HELPER_H

#include <iostream>
#include <string>
#include <vector>
#include <cmath>
#include <algorithm>
#include <fstream>
#include <sstream>
#include <unordered_map>
#include <cassert>

using std::cout;
using std::endl;
using std::string;
using std::vector;
using std::pair;
using std::make_pair;
using std::unordered_map;
using std::stoi;


template<class T> void print(vector<T> vec){
    auto size = vec.size();
    cout << "{ ";
    for(int i = 0; i < size-1; ++i)
        cout << vec[i] << " , ";
    cout << vec[size-1] << " }" << endl;

}


#endif
