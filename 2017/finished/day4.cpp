#include <iostream>
#include <string>
#include <vector>
#include <cmath>
#include <algorithm>
#include <fstream>
#include <sstream>

#define STAR8

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

bool is_anagram(std::string s1, std::string s2){
  std::sort(s1.begin(), s1.end());
  std::sort(s2.begin(), s2.end());
  return s1 == s2;
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
		string pw = "";
		vector<string> passwords;
		while(iss >> pw){
			passwords.push_back(pw);
		}
		++valid;	
#ifdef STAR8
		for(auto &elem : passwords){
			 std::sort(elem.begin(), elem.end());
		}
#endif


		for(auto itr = passwords.begin(); itr != passwords.end();++itr){

			if(std::find(itr+1, passwords.end(), *itr) != passwords.end()){
				--valid;
				cout << line << " : not valid :(" << endl;
				break;
			}
		}
		++lines;
	}	
	cout << valid << " lines from " << lines << " valid!" << endl;
	return 0;
}
