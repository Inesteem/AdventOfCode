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
	int g_chars = 0;
	int number = 0;
	int score = 0;
	bool garbage = false;
	bool ignore = false;
	infile >> std::noskipws;
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
		else if(!garbage && ch == '<'){
			garbage = true;
		}	
		else if(!garbage && ch == '{'){
			++group;
			
		}
		else if(!garbage && ch == '}'){
			assert(group);
			++number;	
			score += group;
			--group;
		} 
		else if(garbage){
			++g_chars;
		}	
	}
	cout << "Groups :  " << number << endl;
	cout << "Score :   " << score << endl;
	cout << "Garbage : " << g_chars << endl;
	cout << endl;	
	return 0;
}
