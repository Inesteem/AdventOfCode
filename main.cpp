#include "include/helper.h"
#include <stack>
using std::stack;


#define INSTR    0
#define REG_DST  1
#define REG_SRC  2 
#define OPERAND  2 
//stack<int> last_played;
long last_played = 0;

unordered_map<string, long> reg_mem;

void print_map(){
    for(auto &reg : reg_mem){
        cout << reg.first << ": " << reg.second << endl;
    }
}

int do_op(vector<string> & words){
    bool is_digit = false;
    if(words.size() > OPERAND){ 
        is_digit = (words[OPERAND].find_first_not_of( "0123456789-" ) == string::npos);
        if(is_digit) cout << "digit!" << endl;
    }
    if(words[INSTR] == "snd")
        //last_played.push(reg_mem[words[REG_DST]]);
        last_played = reg_mem[words[REG_DST]];
    
    else if(words[INSTR] == "set"){
        if(is_digit)
            reg_mem[words[REG_DST]] = stoi(words[OPERAND]);
        else 
            reg_mem[words[REG_DST]] = reg_mem[words[REG_SRC]];

    } else if(words[INSTR] == "add"){
        if(is_digit)
            reg_mem[words[REG_DST]] += stoi(words[OPERAND]);
        else 
            reg_mem[words[REG_DST]] += reg_mem[words[REG_SRC]];
    } else if(words[INSTR] == "mul"){
        if(is_digit)
            reg_mem[words[REG_DST]] *= stoi(words[OPERAND]);
        else 
            reg_mem[words[REG_DST]] *= reg_mem[words[REG_SRC]];

    } else if(words[INSTR] == "mod"){
        if(is_digit)
            reg_mem[words[REG_DST]] %= stoi(words[OPERAND]);
        else 
            reg_mem[words[REG_DST]] %= reg_mem[words[REG_SRC]];
 
    } else if(words[INSTR] == "rcv"){
        if(reg_mem[words[REG_DST]]){
            //assert(!last_played.empty());
             //last_played.top();
            //last_played.pop();
            cout << "last sound played: " << last_played << endl;
            exit(0);
        }
    }
    else if(words[INSTR] == "jgz"){
        if(reg_mem[words[REG_DST]] > 0){
            return stoi(words[OPERAND]);
        }
        
    }
 
    else assert(false);

    return 1;
}
        


int main(int argc, char *argv[]){
	if(argc != 2){
		cout << "wrong parameter count, submit filename!" << endl;
		exit(0);
	}	
    auto lines = get_lines(" ", argv[1], true);

    for(int i = 0; i < lines.size();){
        cout << endl;
        print(lines[i]);
        i += do_op(lines[i]);
        print_map();
    }
	return 0;
}
