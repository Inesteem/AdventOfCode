#ifndef HELPER_H
#define HELPER_H

#include <iostream>
#include <string>
#include <list>
#include <vector>
#include <array>
#include <cmath>
#include <algorithm>
#include <fstream>
#include <sstream>
#include <unordered_map>
#include <cassert>
#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>

using std::cout;
using std::endl;
using std::string;
using std::vector;
using std::array;
using std::list;
using std::pair;
using std::make_pair;
using std::unordered_map;
using std::stoi;

template <template <typename, typename> class Container, 
          typename Value,
          typename Allocator=std::allocator<Value> >
void print(const Container<Value, Allocator> & vec){
    auto size = vec.size();
    cout << "{ ";
    for(int i = 0; i < size-1; ++i)
        cout << vec[i] << " , ";
    cout << vec[size-1] << " }" << endl;
}




#endif
