#include "helper.h"


class Programm {
public:
	int id;
	int groupid;
	vector<Programm*> connections;
	bool visited;

	Programm():id(0),groupid(-1), visited(false){}

	void add_con(Programm *p){
		connections.push_back(p);	
	}

};

unordered_map<int,Programm> progs;

bool wide_first_search(Programm *first_elem, int id){
	queue<Programm*> to_visit;
	to_visit.push(first_elem);
	Programm *now;
	while(to_visit.size()){
	 	now = to_visit.front();
	 	to_visit.pop();
		if(now->visited) continue;
		if(now->id == id) return true;
		now->visited = true;
		for(auto &p : now->connections){
			to_visit.push(p);
		}
	}
	return false;
}

int main(int argc, char *argv[]){


	if(argc != 2){
        cout << "wrong parameter count, submit filename!" << endl;
        exit(0);
    }
	string p_delim = "<->";
	auto lines = get_one_line("\n", argv[1], true);
	
	for(auto &str : lines){
		vector<int> prog_con;
		auto pos = str.find(p_delim);
		assert(pos != std::string::npos); 
		prog_con.push_back(stoi(str.substr(0, pos)));
		str.erase(0, pos + p_delim.length());

		while ((pos = str.find(",")) != std::string::npos) {
			prog_con.push_back(stoi(str.substr(0, pos)));
			str.erase(0, pos + 1);
		}	
		prog_con.push_back(stoi(str));
		assert(prog_con.size() > 1);
		for(int i = 1; i < prog_con.size(); ++i){
			progs[prog_con[0]].add_con(&progs[prog_con[i]]);
			progs[prog_con[i]].add_con(&progs[prog_con[0]]);
		}
		print(prog_con);
	}
 	for(auto &p : progs){
		p.second.id = p.first;
	} 	

	//search for progID 0
	int group = 0;
	for(auto &p : progs){
		group += wide_first_search(&p.second, 0);
//		if(!wide_first_search(&p.second, 0)){
//			cout << "prog " << p.first << " has no connection to 0!" << endl;
//		}
		for(auto &p2 : progs){
			p2.second.visited = false;
		}
	}
	cout << "group size : " << group << endl;

	return 0;
}
