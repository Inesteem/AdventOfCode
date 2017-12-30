#include "include/helper.h"
#include "include/rbuf.h"

#define NUM 1000000000


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
    for(auto &move : lines){
        assert(move.size() > 1);
        if(move[0] == 's'){
            rb.spin(stoi(move.substr(1)));
            continue;
        }
        auto pos = move.find("/");
        assert(pos != std::string::npos && pos < move.size()-1);
        if(move[0] == 'x'){
            auto pos1 = stoi(move.substr(1, pos-1));
            auto pos2 = stoi(move.substr(pos+1));

            assert(rb.exchange(pos1, pos2));
        } else if(move[0] == 'p'){
            assert(move.size() == 4);
            assert(rb.swap(move[1], move[3]));
        } else {
            cerr << "parsing error: " << move << endl; 
            exit(-1);
        } 

    }

    cout << endl << rb << endl;
	return 0;
}
