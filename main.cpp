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
    vector<int> maze;
    if(argc != 2){
        cout << "wrong parameter count, submit filename! Using default vec" << endl;
        maze = {0,3,0,1,-3};
    } else  {

        std::ifstream infile(argv[1], std::ifstream::in);
        int instr;
         while(infile >> instr){
             maze.push_back(instr);
         }
    } 

    int cnt = 0;
    
    #ifdef STAR8 
    for(int i = 0; i < maze.size() && i >= 0;--i){
        ++maze[i];
        i += maze[i]; 
        ++cnt;
    }
    #else
    for(int i = 0; i < maze.size() && i >= 0;){
        auto offset = maze[i];
        if(maze[i] < 3){
            i += maze[i]++;
        } else {
            i += maze[i]--;
        } 
        ++cnt;
    }

    #endif
    cout << "Instrucions : "; print(maze); cout << endl;
    cout << "It took " << cnt << " steps to fool the system!" << endl; 
	return 0;
}
