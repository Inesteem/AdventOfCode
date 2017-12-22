#include <iostream>
#include <string>
#include <vector>
#include <cmath>
#include <algorithm>
#include <fstream>
#include <sstream>
#include <unordered_map>
#include <cassert>


#define STAR8

using std::cout;
using std::endl;
using std::string;
using std::vector;
using std::pair;
using std::make_pair; 
using std::unordered_map;
//prog name, (weight, num of references)
unordered_map<string, pair<int,int> > programs;

template<class T> void print(vector<T> vec){
    auto size = vec.size();
    cout << "{ ";
    for(int i = 0; i < size-1; ++i)
        cout << vec[i] << " , ";
    cout << vec[size-1] << " }" << endl;

}

inline void insert(string &name, weight){
    auto it = programs.find(name);
    if(it == programs.end()){//no prog inside
        programs.insert(make_pair(name,make_pair(weight,0)));
        return;
    }
    if(weight != -1){
        it->second.first = weigth;
    }
    ++(it->second.second);
}
inline void insert(string &name){
    insert(name,-1);
}

int main(int argc, char *argv[]){
	if(argc != 2){
		cout << "wrong parameter count, submit filename!" << endl;
		exit(0);
	}	
	std::ifstream infile(argv[1], std::ifstream::in);
	if(infile.fail()){
		cout << "an error occured" << endl;
		exit(-1);
	}
	std::string line;
	int valid = 0;	
	int lines = 0;	
	while (std::getline(infile, line)) {
    	std::istringstream iss(line);
		string word = "";
		vector<string> words;
		while(iss >> word){
			words.push_back(word);
		}
        if(!words.size()) continue;
        assert(words.size() >= 2);
       // words[1].erase(0,1);
       // words[1].erase(str.size()-1,1);
        insert(words[0],std::stoi(words[1],1));
        cout << words[1] << " : " << std::stoi(words[1],1) << endl;
        for(int i = 3; i < words.size(); ++i){
                //words[i].erase(std::remove(words[i].begin(), words[i].end(), ','), words[i].end());
            auto pos = words[i].size() - 1;
            if(words[i][pos] == ',') words[i].erase(pos,1);
            insert(words[i]);
        }	
    }
    for(auto it : programs){
        cout << it->first << " : " << it->second.first << " : " << it->second.second << endl;
    }
	return 0;
}
