#include <iostream>
#include <string>
#include <vector>
#include <cmath>
#include <algorithm>

using std::cout;
using std::endl;
using std::string;
using std::vector;


template<class T> void print(vector<T> vec){
    auto size = vec.size();
    cout << "{ ";
    for(int i = 0; i < size-1; ++i)
        cout << vec[i] << " , ";
    cout << vec[size-1] << " }" << endl;

}


int get_ring(int num){
    if(num <= 1) return 0;

    auto wurzel = sqrt(num);
    int ret = 0;
    if(!((int)wurzel & 0x1)){//ungerade
        ret = 1 + (wurzel - 1)/2;
    }
    else if ( !((float)((int) wurzel) - wurzel)){ ret = (wurzel-1)/2;}
    else ret =  wurzel/2 + 1;
    
    return ret;
}
//1,3,5,7,9
//0,1,2,3,4
bool is_edge(int idx) {
    if(!idx) return true;
    int num = idx + 1;

    auto ring = get_ring(num);
    int max, magic;
    max =  magic = (2*ring+1);
    max *= max;

    for(int i = 0; i < 4; ++i ){
        if(num == (i + max - i * magic)) return true;

    }
    return false; 
}

int main(){

	vector<int> vec = {1,1,2,4,5,10,11,23,25};

	int ring = 2;
	int width = 5;
	cout << "starting with vec.size() == " << vec.size() << endl;
	while(ring < 5){
		int min = width-2; min *= min; ++min;
	    int dist = (ring-1) * 8 ;	

		//vec.size() == min
		int num = vec[vec.size()-1];
		num += vec[vec.size()-dist];
		vec.push_back(num);
		 
		++dist;
		
		for(int i = 0; i < 4; ++i){
			//number after edge
			num = vec[vec.size()-1];
			num += vec[vec.size()-2];
			num += vec[vec.size() - dist];
			num += vec[vec.size() - dist + 1];
//			cout << num << " = " << vec[vec.size()-1] << " + " << vec[vec.size()-2] << " + " << vec[vec.size()-dist] << " + " <<vec[vec.size()-dist+1] << endl;
			vec.push_back(num);
			int j = 0; if(!i) j = 1; // entry point
			for(; j < width-4; ++j){//2 edges, 2 near edges
				num = vec[vec.size()-1];
				num += vec[vec.size()-dist+1];
				num += vec[vec.size()-dist];
				num += vec[vec.size()-dist-1];
//			cout << num << " = " << vec[vec.size()-1] << " + " << vec[vec.size()-dist-1] << " + " << vec[vec.size()-dist] << " + " <<vec[vec.size()-dist+1] << endl;
				vec.push_back(num);
				 
			}
			//number befor edge
			num = vec[vec.size()-1];
			num += vec[vec.size()-dist];
			num += vec[vec.size()-dist-1];
			if(i == 3)
				num += vec[vec.size()-dist+1];
			
//			cout << num << " = " << vec[vec.size()-1] << " + " << vec[vec.size()-dist] << " + " <<vec[vec.size()-dist-1] << endl;
			vec.push_back(num);
			++dist;
			//edge
			num = vec[vec.size()-dist];
			num += vec[vec.size()-1];
			if(i == 3)
				num += vec[vec.size()-4*width+5];
//			cout << num << " = " << vec[vec.size()-dist] << " + " << vec[vec.size()-1]<< endl;
			vec.push_back(num);
			 
			++dist;
		}
		width+=2;		
		++ring;		
	}	
	if(std::find(vec.begin(), vec.end(), 347991) != vec.end()) cout << "found!" << endl;	
	print(vec);	



    return 0;
}
