#include "helper.h"
//#include <boost/functional/hash.hpp>

unsigned char memory[128][128];

class Memory{
public:
	int id;
	vector<Memory *> connections;
	Memory(): id(0){}
	void connect(Memory *mem){
		mem->connections.push_back(this);
		connections.push_back(mem);
	}

	void set_group(int gid){
		assert(gid);
		if(id) return;
		id = gid;
		for(auto mem : connections)
			mem->set_group(gid);
	}
};

inline int32_t get_skey(int32_t row, int32_t col){
	int32_t ret = 0xffff & row;
	ret = ret << 16;
	ret |= (col & 0xffff);
	return ret;
}

unordered_map<int32_t,Memory> memory_map;


void get_bits(string &hash, vector<unsigned int> &row){
	assert(row.size() == 4 && hash.size() == 32);
	for(int i = 0; i < 4; ++i){
		std::stringstream ss;
		ss << std::hex << hash.substr(i*8, 8);
		//cout << hash.substr(i*8, 8) << endl;
		ss >> row[i];
	}
    // output it as a signed type
    //std::cout << static_cast<int>(x) << std::endl;

}
void fill_bits(string &hash, int row){
	assert(hash.size() == 32);
	uint32_t x;
	for(int i = 0; i < 4; ++i){
		std::stringstream ss;
		ss << std::hex << hash.substr(i*8, 8);
		ss >> x;
		for(int j = 0; j < 32; ++j){
			//memory[row][31-j+32*i] = (x & 0x1) ? '#' : '.';
			memory[row][31-j+32*i] = (x & 0x1);
			if(x & 0x1){
				memory_map[get_skey(row,31-j+32*i)];
			}
			x = x>>1;
		}
	}
    // output it as a signed type
    //std::cout << static_cast<int>(x) << std::endl;

}




int main(int argc, char *argv[]){
	vector<unsigned int> row(4);
    auto path = get_current_dir_name(); 	
	string p_str = path;
	p_str+= "/knothash";
	//string key = "flqrgnkx";
	string key = "stpzcrnm";
	//int squares_in_use = 0;
	for(int i = 0; i < 128; ++i){
		string line_key = key + string("-") + std::to_string(i);
		auto hash = get_output(p_str.c_str(),"knothash", line_key.c_str());
		hash = hash.substr(0, hash.size()-1);
 		fill_bits(hash, i);
	/*
 		get_bits(hash, row);
		squares_in_use += 
		  std::accumulate(row.begin(),row.end(),0,
			[](uint32_t sum, uint32_t val) { 
				return sum + numberOfSetBits(val); 
			} 
		  );
	*/
	}
	//print map
    for(int row = 0; row < 8; ++row){
        for(int col = 0; col < 8; ++col){
            if(!memory[row][col]) cout << ".";
            else cout << "#";

        }
        cout << endl;
    }

	//connect rows
	for(int row = 0; row < 128; ++row){
		for(int col = 1; col < 128; ++col){
			if(!memory[row][col]) continue;
			if(memory[row][col-1]){
				memory_map[get_skey(row,col)].connect(&memory_map[get_skey(row,col-1)]);
			} 
		}
	}

	//connect columns 
	for(int row = 1; row < 128; ++row){
		for(int col = 0; col < 128; ++col){
			if(!memory[row][col]) continue;
			if(memory[row-1][col]){
				memory_map[get_skey(row,col)].connect(&memory_map[get_skey(row-1,col)]);
			} 
		}
	}
	
	int num = 0;
	for(auto &elem : memory_map){
		if(!elem.second.id){elem.second.set_group(++num);}
	}
	cout << num << endl;

	for(int row = 0; row < 8; ++row){
		for(int col = 0; col < 8; ++col){
			if(!memory[row][col]) cout << ".";
			else cout << "["<<memory_map[get_skey(row,col)].id<<"]";
			
		}
		cout << endl;
	}

	return 0;
}
