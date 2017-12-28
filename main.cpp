#include "helper.h"

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

void test(){
	vector<uint32_t> row(4);// = {0xff,0xf,0xf,0xf};
	string hash = "000000ff000000ff000000ff000000ff";
	get_bits(hash, row);
	print(row);
	int squares_in_use = 0; 
	squares_in_use += 
	  std::accumulate(row.begin(),row.end(),0,
		[](uint32_t sum, uint32_t val) { 
			return sum + numberOfSetBits(val); 
		} 
	  );
	cout << "squares in use: " << squares_in_use << endl; 


	exit(0);
}

int main(int argc, char *argv[]){
	//test();
	vector<unsigned int> row(4);
    auto path = get_current_dir_name(); 	
	string p_str = path;
	p_str+= "/knothash";
	string key = "stpzcrnm";

	int squares_in_use = 0;
	for(int i = 0; i < 128; ++i){
		string line_key = key + string("-") + std::to_string(i);
		auto hash = get_output(p_str.c_str(),"knothash", line_key.c_str());
		hash = hash.substr(0, hash.size()-1);
 		get_bits(hash, row);
		squares_in_use += 
		  std::accumulate(row.begin(),row.end(),0,
			[](uint32_t sum, uint32_t val) { 
				return sum + numberOfSetBits(val); 
			} 
		  );

	}
	cout << "squares in use: " << squares_in_use << endl; 
	return 0;
}
