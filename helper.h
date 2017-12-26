#ifndef HELPER_H
#define HELPER_H

#include <iostream>
#include <string>
#include <queue>
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
#include <iomanip> // setfill, setw

using std::cout;
using std::endl;
using std::string;
using std::vector;
using std::array;
using std::queue;
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


vector<string> get_one_line(string delim, string filename, bool skip_spaces){
    std::ifstream infile(filename, std::ifstream::in);
    if(infile.fail()){
        cout << "an error occured" << endl;
        exit(-1);
    }
	vector<string> ret;

    if(skip_spaces) infile >> std::noskipws;
    char ch;
	string str = "";
    while (infile >> ch) {
		if(delim.find(ch) != std::string::npos){
			ret.push_back(str);
			str = "";
			continue;
		}
		str += ch;
	}
	if(str.size()) ret.push_back(str);

	return ret;

}

#endif
