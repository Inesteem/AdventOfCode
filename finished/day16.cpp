#include "include/helper.h"
#include "include/rbuf.h"

#define NUM 1000000000
class Command {
public:
    unsigned char type;
    int op1,op2;
    Command(unsigned char type, int op1, int op2 = 0):type(type), op1(op1), op2(op2){}

    bool modify(Ringbuffer<unsigned char> &rb){
        if(type == 'p')
            return rb.swap(op1,op2);
        if(type == 'x')
            return rb.exchange(op1,op2);
        rb.spin(op1);
        return true;
    }
   bool operator==(Command &com){
        if(com.type != type) return false;
        if(type == 's') return true;
        if(op1 == com.op1 && op2 == com.op2) return true;
        if(op2 == com.op1 && op1 == com.op2) return true;
        return false;
   } 
};


int main(int argc, char *argv[]){
    if(argc != 2){
        cout << "wrong parameter count, submit filename!" << endl;
        exit(0);
    }

    Ringbuffer<unsigned char> rb;
    for(int i = 0; i < 16; ++i)
        rb.append('a'+ i);
    cout << endl;
    auto lines = get_one_line(",\n", argv[1], true);

    list<Command> coms;

    for(auto &move : lines){
        assert(move.size() > 1);
        if(move[0] == 's'){

          //  rb.spin(stoi(move.substr(1)));
          coms.push_back(Command('s',stoi(move.substr(1))));
            continue;
        }
        auto pos = move.find("/");
        assert(pos != std::string::npos && pos < move.size()-1);
        if(move[0] == 'x'){
            auto pos1 = stoi(move.substr(1, pos-1));
            auto pos2 = stoi(move.substr(pos+1));

            //assert(rb.exchange(pos1, pos2));
            coms.push_back(Command('x',pos1, pos2));
        } else if(move[0] == 'p'){
            assert(move.size() == 4);
            //assert(rb.swap(move[1], move[3]));
            coms.push_back(Command('p',move[1], move[3]));
        } else {
            cerr << "parsing error: " << move << endl; 
            exit(-1);
        } 

    }
    /*
    cout << coms.size() << endl;
    Command *last = nullptr;
    auto last_it = coms.begin();
    auto it = coms.begin(); ++it;
    for(; it != coms.end(); ++it){
        if((*last_it) == (*it)){
            if(it->type == 's'){
                it->op1 += last_it->op1;
                it = coms.erase(last_it);
            } else {
                it = coms.erase(last_it, it);
            }
            cout<<"equal" << endl;
        } 
        last_it = it;
    }
    cout << coms.size() << endl;*/

    Ringbuffer<unsigned char> rb2;
    for(int i = 0; i < 16; ++i)
        rb2.append('a'+ i);
    
     cout << endl;
    int end = NUM+1;
    bool checked = false;
    for(int i = 1; i < end; ++i){
        cout << "\r" << i << "/" << NUM;
        for(auto &com : coms)
            com.modify(rb);
        if(i == 1) cout << endl << "solution 1 : " << rb << endl;
        if(!checked && (rb == rb2)){ 
            cout << endl << "found pattern: " << i << endl; 
            end = NUM % i + 1;
            i = 0;
            checked = true;
        }
    }
    cout << endl << "end solution: " << rb << endl;
	return 0;
}
