#include <iostream>
#include <fstream> 
#include <string>
#include <vector>
#include <sstream>
#include <cassert>
#include <cmath>
#include <climits>
using namespace std;


struct Coord {
	int x;
	int y;
	int s_num;

//	public Coord(int x = 0, int y = 0) : x(x), y(y) {}

	bool move(char dir, int steps){
		s_num += steps;
		switch (dir) {
			case 'D': y -= steps; break;
			case 'U': y += steps; break;
			case 'R': x += steps; break;
			case 'L': x -= steps; break;
			default: assert(false);	
		}
		return x >= 0 && y >= 0;
	}

	int distance(Coord &c){
		return abs(c.x - x) + abs(c.y - y);
	
	}
};


vector<string> make_path(string str){
	string token; 
	istringstream ss(str);
	vector<string> str_vec;

	while(getline(ss, token, ',')) {
		str_vec.push_back(token);
	}

	return str_vec;
}


vector<int> do_stuff( vector<string> &v, vector<string> &v2, vector<vector<bool> > &path){
	{
	Coord c = {7000,5500,0};
		for(auto e : v){
			char dir = e[0];
			int steps = stoi(e.substr(1,e.size()-1));
			for (int i = 0; i < steps; ++i){
				assert(c.move(dir,1) >= 0);
				cout << c.x << " " << c.y << endl;
				assert(c.x < path.size() && c.y < path[0].size()); 
				path[c.x][c.y] = true;
			}
		}
	}
	vector<int> vec;
	{
	Coord start = {7000,5500,0};
	Coord c = {7000,5500,0};

	int min_dist = INT_MAX;
	for(auto e : v2){
		char dir = e[0];
		int steps = stoi(e.substr(1,e.size()-1));
		for (int i = 0; i < steps; ++i){
			assert(c.move(dir,1) >= 0);
			if(path[c.x][c.y]){
				vec.push_back(c.x);
				vec.push_back(c.y);
				vec.push_back(c.s_num);
				cout << c.x << " " << c.y << endl;
				min_dist = min(min_dist, c.distance(start));
			}
		}
	}
	cout << min_dist << endl;
	}
	int min_steps = INT_MAX;
	{
		Coord c = {7000,5500,0};
		for(auto e : v){
			char dir = e[0];
			int steps = stoi(e.substr(1,e.size()-1));
			for (int i = 0; i < steps; ++i){
				assert(c.move(dir,1) >= 0);
				for(int idx = 0; idx < vec.size(); idx+=3){
					if (c.x == vec[idx] && c.y == vec[idx+1]){
						min_steps = min(min_steps, c.s_num + vec[idx+2]);
					}
				} 
			}
		}
	}
	cout << min_steps <<endl;
	return vec;	

//	cout << i << endl;
//	cout << x_min_val << " " << x_max_val << endl;
//	cout << y_min_val << " " << y_max_val << endl;

}


int main(){



	string l1,l2;
	cin >> l1;
	cin >> l2;
	auto str_vec1 = make_path(l1);
	auto str_vec2 = make_path(l2);
	vector<vector<bool> > path(17000, vector<bool>(11100, false));
	vector<int> vec1 = do_stuff(str_vec1, str_vec2, path);
	cout << vec1.size() << endl;
//	assert(vec1.size() == vec2.size());
//	return 0;
//	int min_steps = INT_MAX;
//	for (int i = 0; i < vec1.size(); i+=3) {
//		for (int j = 0; j < vec2.size(); j+=3) {
//			if (vec1[i] == vec2[j] && vec1[i+1] == vec2[j+1])
//				min_steps = min(vec1[i+2] + vec2[j+2], min_steps);
//		}	
//	}
//	cout << min_steps << endl;
	return 0;
}



