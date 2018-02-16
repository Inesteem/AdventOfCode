#include "include/helper.h"
#include "include/rbuf.h"
//input = 394
int main(int argc, char *argv[]){
    if(argc != 2){
        cout << "wrong parameter count, submit filename!" << endl;
        exit(0);
    }
    auto steps = stoi(argv[1]);
    //int end = 2018;
    int end = 50000000;
    int pos = 0;
    vector<int> positions(end);
    int last_one = 0;
    for(int i = 1; i < end; ++i){
        pos = (pos + steps) % i;
        if(!pos) last_one = i;
        ++pos;
        positions[i] = pos; 
    }
    int search_pos = (positions[end-1] + 1) % end;
    cout << "searching elem for " << search_pos << endl;
    for(int i = end - 1; i >= 0; --i){
        if(positions[i] < search_pos) --search_pos;
        else if(positions[i] == search_pos){
            cout << i << endl;
            break;
        }
        
        
    }
    cout << "last_one : " << last_one << endl;
    return 0;

    Ringbuffer<int> rb {0,1};
    rb.advance(1);
    for(int i = 1; i < 50000000; ++i){
    //for(int i = 1; i < 2017; ++i){
        rb.advance(steps);
        rb.insert(i+1);
        rb.advance(1);
    } 
        //cout << rb << endl;
    auto elem = rb.get_elem(0);
    cout << (elem->get_next())->get_value() << endl;
  	return 0;
}
