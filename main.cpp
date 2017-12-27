#include "helper.h"

inline int calc_pos(int depth, int picco){
	if(!picco || depth == 1) return 0;

	int mod = 2*(depth-1);
	int num = picco%mod;
	if(num <= mod/2) return num;

	if(depth & 0x10)//even
		return mod-num-1;

	return mod-num;
}


void print_levels(vector<int> &levels, int picco){
 	cout << picco << ": " << endl;	
	int max = *std::max_element(levels.begin(), levels.end());
	for(int i = 0; i < levels.size(); ++i){
		cout << " " << i << "  ";
	}
	cout << endl;

	for(int i = 0; i < max; ++i){
		for(int level = 0; level < levels.size(); ++level){
			auto depth = levels[level];
			bool scanner = calc_pos(depth, picco)  == i; 
			if(depth > i){
				if(scanner){ 
					if(level == picco) cout << "(S) ";
					else cout << "[S] ";
				} else{
					if(!i && level == picco) cout << "( ) ";
					else cout << "[ ] ";
				} 
			}	
			else if(!i){
				 if(level == picco) cout << "(.) ";
				 else cout << "... ";
			}
			else cout << "    ";
		}
		cout << endl;
	}
}

int main(int argc, char *argv[]){


	if(argc != 2){
        cout << "wrong parameter count, submit filename!" << endl;
        exit(0);
    }
	auto lines = get_one_line("\n", argv[1], true);
	vector<int> levels;	
	for(auto &str : lines){
		auto pos = str.find(":");
		assert(pos != std::string::npos); 
		int level = stoi(str.substr(0, pos));
		int depth = stoi(str.substr(pos + 1, str.size()-1 ));

		int fill = level - levels.size();
		assert(fill >= 0);
		if(fill){
			levels.insert(levels.end(), fill+1, 0);
			levels[levels.size()-1] = depth;
		} else {
			levels.push_back(depth);
		}
	}
	int piccos = 0;
	int score_of_doom = 0;
	for(int i = 0; i < levels.size(); ++i){
		//print_levels(levels, i);
		auto depth = levels[i];
		bool scanner = !calc_pos(depth, i);
		if(depth && scanner){
			score_of_doom += depth * i;
		}
	}
	cout << score_of_doom << endl;
	return 0;
}
