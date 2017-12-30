#include "helper.h"



#define REG_DST  0
#define INSTR    1
#define IM_1     2
#define REG_SRC  4 
#define OP       5 
#define IM_2     6

inline void print_instr(vector<string> &words){
    cout << words[3] << " (" << words[REG_SRC] << " " << 
            words[OP] << " " << words[IM_2] << ") { ";
    cout << words[REG_DST];
    if(words[INSTR] == "inc") 
         cout << " += ";
    else if(words[INSTR] == "dec") 
         cout << " -= ";
    else
         cout << " " << words[INSTR] << " ";

    cout << words[IM_1] << "; }" << endl;

}

typedef pair<string, int> MyPairType;
struct CompareSecond {
    bool operator()(const MyPairType& left, const MyPairType& right) const
    {
        return left.second < right.second;
    }
};



int getMax(std::unordered_map<std::string, int> mymap) {
  std::pair<std::string, int> max 
      = *max_element(mymap.begin(), mymap.end(), CompareSecond());
  return max.second; 
}


unordered_map<string, int> reg_mem;

bool check_condition(vector<string> & words){
    int im = stoi(words[IM_2]);

    if(words[OP] == ">")
        return reg_mem[words[REG_SRC]] > im;

    if(words[OP] == "<")
        return reg_mem[words[REG_SRC]] < im;

    if(words[OP] == "<=")
        return reg_mem[words[REG_SRC]] <= im;

    if(words[OP] == ">=")
        return reg_mem[words[REG_SRC]] >= im;

    if(words[OP] == "==")
        return reg_mem[words[REG_SRC]] == im;

    if(words[OP] == "!=")
        return reg_mem[words[REG_SRC]] != im;

    assert(false);
    return false;
}

void do_op(vector<string> & words){
    int im = stoi(words[IM_1]);

    if(words[INSTR] == "inc")
        reg_mem[words[REG_DST]] += im;
    
    else if(words[INSTR] == "dec")
        reg_mem[words[REG_DST]] -= im;

    else assert(false);
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
    int max = 0;
	while (std::getline(infile, line)) {
    	std::istringstream iss(line);
		string word = "";
		vector<string> words;
		while(iss >> word){
			words.push_back(word);
		}
        if(!words.size()) continue;
        assert(words.size() == 7);
        if(check_condition(words)){
            print_instr(words);
            cout << endl;
            do_op(words);
        } else { 
            cout << "SKIPPING operation: "; 
            print_instr(words);
            cout << endl;
        }
        cout << "-----------" << endl;
        for(auto &it : reg_mem){
            cout << it.first << " : " << it.second << endl;
        }   
        cout << "-----------\n" << endl;
        int tmp_max = getMax(reg_mem);
        if(tmp_max > max) max = tmp_max;
    }
    cout << "The max_element is: " << getMax(reg_mem) << endl;
    cout << "And the total max_element is: " << max << endl;
	return 0;
}
