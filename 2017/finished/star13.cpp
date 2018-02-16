#include <iostream>
#include <string>
#include <vector>
#include <cmath>
#include <algorithm>
#include <fstream>
#include <sstream>
#include <unordered_map>
#include <cassert>


#define STAR8

using std::cout;
using std::endl;
using std::string;
using std::vector;
using std::pair;
using std::make_pair; 
using std::unordered_map;
//prog name, (weight, num of references)

class Node {
public:
    Node *parent;
    string name;
    int weight;
    vector<Node *> children;
    
    Node(string name, int weight): parent(nullptr),name(name),weight(weight){ }

    inline bool is_leaf(){
        return !children.size();
    }
    inline bool same_weight(Node *n){
        return n->weight == weight;
    }
    int get_weight(){
        int w = weight;
        for(auto &c : children)
            w -= c->weight;
        return w;
    }

    void print(){
        cout << "name          : " << name << endl;
        cout << "weight        : " << get_weight() << endl;
        cout << "tower_weight  : " << weight << endl;
        cout << "num of childs : " << children.size() << endl;
    }
    
    Node *get_unbalanced_one(int weight_expected){
        cout << "-----(" << weight_expected << ")-----" << endl;
        print();
        int cmp = 0;
        int size = children.size();

        if(!size){
             weight = weight_expected;
             return this;
        }
        
        assert(size > 1);
        cmp = children[0]->weight; 
       
        if(!children[0]->same_weight(children[1])){
            if(size > 2) {
                cmp = children[2]->weight;
            } else {
                assert(false);

            }
        }
      
        cout << "cmp " << cmp << endl;

       for(int i = 0; i < children.size(); ++i){
           if(children[i]->weight != cmp) return children[i]->get_unbalanced_one(cmp);
       }

       cout << "-3-" << endl;
       //own weight is wrong
       
       weight = weight_expected;
       return this;
    }
   
    int calc_tower_weight(){
        for(auto &child : children){
            weight += child->calc_tower_weight();
        }
        return weight;
    }
};

unordered_map<string, Node*> programs;

inline bool make_a_family(string &child, string &parent){

    auto ch_it = programs.find(child);
    auto pa_it = programs.find(parent);
    if(ch_it == programs.end() || pa_it == programs.end()) return false;
    
    ch_it->second->parent = pa_it->second;
    pa_it->second->children.push_back(ch_it->second);

    return true;
}

inline void insert(string &name, int weight){
    auto it = programs.find(name);
    if(it == programs.end()){//no prog inside
        Node *n = new Node(name,weight);
        programs.insert(make_pair(name,n));
        return;
    }
    if(weight != -1){
        it->second->weight = weight;
    }
}

inline void insert(string &name){
    insert(name,-1);
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
		string word = "";
		vector<string> words;
		while(iss >> word){
			words.push_back(word);
		}
        if(!words.size()) continue;
        assert(words.size() >= 2);
        words[1].erase(0,1);
       // words[1].erase(str.size()-1,1);
        insert(words[0],std::stoi(words[1]));
        for(int i = 3; i < words.size(); ++i){
                //words[i].erase(std::remove(words[i].begin(), words[i].end(), ','), words[i].end());
            auto pos = words[i].size() - 1;
            if(words[i][pos] == ',') words[i].erase(pos,1);
            insert(words[i]);
            make_a_family(words[i],words[0]);
        }	
    }
    Node *root = nullptr;
    for(auto it : programs){
//        cout << it.first << " : " << it.second.first << " : " << it.second.second << endl;
        if(!it.second->parent){
            root = it.second;
            
        }
    }
    assert(root);
    cout << "root is : " << root->name << endl;
    root->calc_tower_weight();
   // for(auto it : programs){
   //     it.second->print();
   //     cout << endl;
   // }
    cout << "searching for the spast" << endl;
    auto spast = root->get_unbalanced_one(-1);
    //assert(spast);
    cout << endl << "And the spast is : " << endl;
    spast->print(); 
    
	return 0;
}
