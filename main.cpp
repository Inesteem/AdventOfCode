#include "helper.h"



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
	char ch;
	int group = 0;
	int number = 0;
	int score = 0;
	bool garbage = false;
	bool ignore = false;
	
	while (infile >> ch) {

		if(ignore){
			ignore = false;
		}
		else if(ch == '!'){
			if(garbage) 
				ignore = true;
		}
		else if(ch == '>'){
			garbage = false;
		}
		else if(ch == '<'){
			garbage = true;
		}	
		else if(ch == '{'){
			if(!garbage)
				++group;
			
		}
		else if(ch == '}'){
			if(!garbage){
				assert(group);
				++number;	
				score += group;
				--group;
			}
		}	
	}
	cout << "Groups: " << number << endl;
	cout << "score : " << score << endl;
	cout << endl;	
	return 0;
}
