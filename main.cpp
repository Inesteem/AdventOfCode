#include "helper.h"
#define INPUT

#define N  0
#define S  1
#define NE 2
#define NW 3
#define SE 4
#define SW 5

const char *DIR[] = {" N", " S", "NE", "NW", "SE", "SW"};

/*
inline size_t oppos_idx(string &dir){
	if(!dir.compare("ne")) return SW;
	if(!dir.compare("sw")) return NE;

	if(!dir.compare("se")) return NW;
	if(!dir.compare("nw")) return SE;
	
	if(!dir.compare("n")) return S;
	if(!dir.compare("s")) return N;

	cout << "unknown direction!" << endl;
	exit(-1);	
}
*/
void remove_opposed_dirs(vector<int> &num_dir){
	int diff = std::abs(num_dir[N] - num_dir[S]);
		cout << "N-S: " << num_dir[N] << " - " << num_dir[S] << endl;
	if(num_dir[N] > num_dir[S]){
		num_dir[N] = diff;
		num_dir[S] = 0;
		
	} else {
		num_dir[S] = diff;
		num_dir[N] = 0;
	}
		cout << "NW-SE: " << num_dir[NW] << " - " << num_dir[SE] << endl;

	diff = std::abs(num_dir[NW] - num_dir[SE]);
	if(num_dir[NW] > num_dir[SE]){
		num_dir[NW] = diff;
		num_dir[SE] = 0;
	} else {
		num_dir[SE] = diff;
		num_dir[NW] = 0;
	}
		cout << "SW-NE: " << num_dir[SW] << " - " << num_dir[NE] << endl;
	diff = std::abs(num_dir[SW] - num_dir[NE]);
	if(num_dir[SW] > num_dir[NE]){
		num_dir[SW] = diff;
		num_dir[NE] = 0;
	} else {
		num_dir[NE] = diff;
		num_dir[SW] = 0;
		}

}


inline void rb_helper(vector<int> &num_dir, int dir1, int dir2, int new_dir){
	auto min = std::min(num_dir[dir1],num_dir[dir2]);
	if(min) cout << DIR[dir1] << ": " << num_dir[dir1] << " | " <<
					DIR[dir2] << ": " << num_dir[dir2] << " | " <<
					DIR[new_dir] << ": " << num_dir[new_dir] << " => "<< endl;
	num_dir[new_dir] += min;
	num_dir[dir1] -= min;
	num_dir[dir2] -= min;
	if(min) cout << DIR[dir1] << ": " << num_dir[dir1] << " | " <<
					DIR[dir2] << ": " << num_dir[dir2] << " | " <<
					DIR[new_dir] << ": " << num_dir[new_dir] << endl;

}
void replace_bends(vector<int> &num_dir){
	rb_helper(num_dir, N, SW, NW);
	rb_helper(num_dir, N, SE, NE);
	rb_helper(num_dir, S, NW, SW);
	rb_helper(num_dir, S, NE, SE);
	rb_helper(num_dir, NW, NE, N);
	rb_helper(num_dir, SW, SE, S);
}


int main(int argc, char *argv[]){


	if(argc != 2){
        cout << "wrong parameter count, submit filename!" << endl;
        exit(0);
    }

	auto line = get_one_line(",\n", argv[1], true);
	
	int max = 0;
	
	vector <int> num_dir = {0,0,0,0,0,0};
	for(auto &str : line){

		if(!str.compare("n")) ++num_dir[N];
		else if(!str.compare("s")) ++num_dir[S];
		else if(!str.compare("nw")) ++num_dir[NW];
		else if(!str.compare("sw")) ++num_dir[SW];
		else if(!str.compare("ne")) ++num_dir[NE];
		else if(!str.compare("se")) ++num_dir[SE];
		else { cout << "ERROR while parsing" << endl; exit(-1); }

		remove_opposed_dirs(num_dir);
		replace_bends(num_dir);
		int steps = std::accumulate(num_dir.begin(), num_dir.end(),0);
		cout << "steps needed : " << steps << endl;
		max = std::max(max, steps);
	}
		cout << "max steps needed : " << max << endl;
	

	return 0;
}
