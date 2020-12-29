#include <cassert>
#include <iostream>
#include <vector>
#include <unordered_map>

using namespace std;

struct Node {
    long value;
    Node *prev;
    Node *next;
};

class RingBuffer{
    Node *head;
    Node *tail;
    size_t size;
    unordered_map<long, Node> nodes;
public:
    RingBuffer() {
        head = nullptr;
        tail = nullptr;
        size = 0; 
    }
     
    Node *get(long elem) {
        assert(nodes.find(elem) != nodes.end());     
        return &nodes[elem];
    }
    
    Node* start() { return head; }
    size_t num_nodes() { return size; }
    void remove (Node *node) {
        assert(node->next != nullptr && node->prev != nullptr);
        --size;
        node->prev->next = node->next;
        node->next->prev = node->prev;
        if (node == tail) tail = node->prev;
        else if (node == head) head = node->next;
        node->prev = node->next = nullptr;
    }

    void insert_after(Node *dst, Node *node) {
        assert(!node->next && !node->prev);
        ++size;
        if (dst == tail) tail = node;
        node->prev = dst;
        node->next = dst->next;
        dst->next->prev = node;
        dst->next = node; 
        

    }


    void push_back(long elem) {
        assert(nodes.find(elem) == nodes.end());     
        ++size;
        nodes[elem].value = elem;

        if (tail == nullptr) {
            tail = &nodes[elem];
            head = &nodes[elem];      
            nodes[elem].next =nodes[elem].prev = &nodes[elem];
            return;
        }             
        tail->next = &nodes[elem];
        nodes[elem].prev = tail;
        nodes[elem].next = head;
        tail = &nodes[elem];
    }
    



    void print() {
        Node *mom = head;
        do {
            cout << mom->value << " ";
            mom = mom->next;
        }while (head != mom);     
        cout << endl;
    }


    void star1() {
        Node *mom = get(1)->next;
        cout << "star1: ";
        while (mom->value != 1){    
            cout << mom->value;
            mom = mom->next;
        }
        cout << endl;
    } 

    void star2() {
        cout << "star2: " << get(1)->next->value * get(1)->next->next->value << endl;
    } 

};


int main() {

    bool star2 = true;

    RingBuffer cups;

    vector<long> vals({4,6,9,2,1,7,5,3,8});
    //vector<long> vals({3,8,9,1,2,5,4,6,7});
    for (auto v : vals)  {
        cups.push_back(v);
    }
    long lowest = 1;
    long highest = 9;
    auto iterations = 100;
    if (star2) {
        for(int i = 10; i < 1000001; ++i) {
            cups.push_back(i);
        }
        highest = 1000000;
        iterations = 10000000;
    }


    Node *removed[3] = {nullptr, nullptr,nullptr};
    auto cur = cups.start();


    for(int i = 0; i <iterations; ++i) {
        //cout << "\n" << "-- move " << i + 1<< " --\n" << "cups: ";
        //cups.print();
        //cout << "pickup: ";
        for (int j = 0; j < 3; ++j){
            Node *rem =cur->next;
            removed[j]= rem;
        //    cout << rem->value << " " <<cur->next->value << " " <<rem->prev->value << " ";
          //  cout.flush();
            cups.remove(rem);
        }
//        cout << endl;

        Node * dst;
        auto s= cups.num_nodes();
        long val = cur->value-1;
        do {
            if (val <= 0) {
                val = highest;
            }
            dst=cups.get( val );
            --val; 
        } while (!dst->prev);

//         cout << "dest: " << dst->value << endl;

        for (int j = 0; j < 3; ++j){
            cups.insert_after(dst,removed[2-j]);
        }

        cur = cur->next;
    }

    if (star2) {
        cups.star2();
    } else {
        cups.star1();
    }


    return 0;
}
