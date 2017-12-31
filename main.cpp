#include "include/helper.h"


#define INSTR    0
#define REG_DST  1
#define REG_SRC  2 
#define OPERAND  2 
queue<long> tasks_p1;
queue<long> tasks_p2;

long snt_count = 0;

unordered_map<string, long> reg_mem_p1;
unordered_map<string, long> reg_mem_p2;

void print_map(int id){
    assert(id);
    decltype(reg_mem_p1) &reg_mem =  (id == 1) ? reg_mem_p1 : reg_mem_p2 ;
    //cout << "Program " << id << ":" << endl;
    for(auto &reg : reg_mem){
        cout << reg.first << ": " << reg.second << endl;
    }
}

int do_op(vector<string> & words, int id){
    assert(id);
    decltype(reg_mem_p1) &reg_mem =     (id == 1) ? reg_mem_p1 : reg_mem_p2 ;
    decltype(tasks_p1)   &tasks =       (id == 1) ? tasks_p1   : tasks_p2 ;
    decltype(tasks_p1)   &other_tasks = (id == 1) ? tasks_p2   : tasks_p1 ;
    
    bool is_digit = (words[words.size() - 1].find_first_not_of( "0123456789-" ) == string::npos);

    if(words[INSTR] == "snd"){
        if(is_digit)
            other_tasks.push(stoi(words[1]));
        else 
            other_tasks.push(reg_mem[words[REG_DST]]);
        if(id == 2) ++snt_count; 
    } else if(words[INSTR] == "set"){
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
        if(tasks.empty()) return 0;
        reg_mem[words[REG_DST]] = tasks.front();
        tasks.pop();
    }
    else if(words[INSTR] == "jgz"){
        if(reg_mem[words[REG_DST]] > 0){
            int ret;
            if(is_digit)
                ret = stoi(words[OPERAND]);
            else 
                ret = reg_mem[words[REG_SRC]];
            if(ret) return ret;
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
    int pos1 = 0; 
    int pos2 = 0;
    reg_mem_p1["p"] = 0;
    reg_mem_p2["p"] = 1;
    int p1_blocked = 0;
    int p2_blocked = 0;
    while(p1_blocked < 2 || p2_blocked < 2){
        for(; pos1 < lines.size();){
            //cout << endl << "[1] ";
            //print(lines[pos1]);
            auto ret = do_op(lines[pos1], 1);
            if(!ret){
                ++p1_blocked; 
                cout << "P1 waiting at instruction["<<pos1<<"]: "; print(lines[pos1]); cout << endl;
                cout << tasks_p2.size() << endl;
                print_map(1);
                break;
            }
            p1_blocked = 0;
            pos1 += ret;
        //    print_map(1);
        }
        for(; pos2 < lines.size();){
  //          cout << endl << "[2] ";
//            print(lines[pos2]);
            auto ret = do_op(lines[pos2], 2);
            if(!ret){
                ++p2_blocked; 
                cout << "P2 waiting at instruction["<<pos2<<"]: "; print(lines[pos2]); cout << endl;
                cout << tasks_p1.size() << endl;
                print_map(2);
                break;
            }
            p2_blocked = 0;
            pos2 += ret;
//            print_map(2);
        }

        if(pos1 == lines.size() && pos2 == lines.size()) break;
    }
    cout << "prog 1 send " << snt_count << " times a value!" << endl;
	return 0;
}
