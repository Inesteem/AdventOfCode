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
#include <bitset>
#include <cstdint>

#define die(e) do { fprintf(stderr, "%s\n", e); exit(EXIT_FAILURE); } while (0);


using std::cout;
using std::cerr;
using std::clog;
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
        cout << "an error occured; file " << filename << " could not be opened!" << endl;
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


string get_output(const char *exe_path, const char *exe, const char *argv){


	int link[2];
	pid_t pid;
	char output[4096];

	if (pipe(link)==-1)
	  die("pipe");

	if ((pid = fork()) == -1)
	  die("fork");

	if(pid == 0) {

	  dup2 (link[1], STDOUT_FILENO);
	  close(link[0]);
	  close(link[1]);
	  execl(exe_path, exe, argv, (char *)0);
	  die("execl");

	} else {

	  close(link[1]);
	  int nbytes = read(link[0], output, sizeof(output));
	  //printf("Output: (%.*s)\n", nbytes, foo);
	  //wait(NULL);
	  return string(output, nbytes);
	}

	return "";
}

void print_binary(uint32_t i){
	std::bitset<32> x(i);
	cout << x << endl;
}

int numberOfSetBits(uint32_t i){
     // Java: use >>> instead of >>
     i = i - ((i >> 1) & 0x55555555);
     i = (i & 0x33333333) + ((i >> 2) & 0x33333333);
     return (((i + (i >> 4)) & 0x0F0F0F0F) * 0x01010101) >> 24;
}

#endif
