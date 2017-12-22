#include <iostream>
#include <string>
#include <vector>
#include <cmath>
#include <algorithm>
#include <fstream>
#include <sstream>
#include <unordered_map>

#define STAR9

using std::cout;
using std::endl;
using std::string;
using std::vector;
using std::unordered_map;


unordered_map<int, vector<vector<int> > > database;

void do_stuff(vector<int> &data);

template<class T> void print(vector<T> vec){
    auto size = vec.size();
    cout << "{ ";
    for(int i = 0; i < size-1; ++i)
        cout << vec[i] << " , ";
    cout << vec[size-1] << " }" << endl;

}

template<class T> bool is_same(const vector<T> &vec, const vector<T> &vec2){
    if(vec.size() != vec2.size()) return false;
    for(int i = 0; i < vec.size(); ++i)
        if(vec[i] != vec2[i]) return false;
    return true;
}

bool already_there(vector<int> &data){
    int sum = 0;
    for (auto& n : data)
        sum += n;
    auto it = database.find(sum);
    if(it == database.end()){
        vector<vector<int> > vec = {data};        
        database.insert(std::make_pair(sum,vec)); 
        return false;
    }
    //for(auto &vec_it = (it->second).begin(); vec_it != (it->second).end(); ++vec_it){
     //   if(is_same(*vec_it, data)) return true;
    for(int i = 0; i < (it->second).size(); ++i){
        if(is_same((it->second)[i], data)) return true;
    }
    (it->second).push_back(data);
    return false;
}


int main(int argc, char *argv[]){
    

    {
        vector<int> data = {0,2,7,0};
        do_stuff(data);
        cout << already_there(data) << endl;
    }
    {
        vector<int> data = {2,4,1,2};
        do_stuff(data);
        cout << already_there(data) << endl;
    }
    {
        vector<int> data = {3,1,2,3};
        do_stuff(data);
        cout << already_there(data) << endl;
    }
    {
        vector<int> data = {0,2,3,4};
        do_stuff(data);
        cout << already_there(data) << endl;
    }
    {
        vector<int> data = {1,3,4,1};
        do_stuff(data);
        cout << already_there(data) << endl;
    }
    int cnt = 0;
    vector<int> data = {11, 11, 13, 7, 0, 15, 5, 5, 4, 4, 1, 1, 7, 1, 15, 11};
    while(!already_there(data)){
        do_stuff(data);
        ++cnt;
    }
    cout << "Es brauchte " << cnt << " Durchgaenge" << endl;
   	return 0;
}

void do_stuff(vector<int> &data){
    cout << "---------------------" << endl;
    print(data);
    int start = max_element(data.begin(),data.end()) - data.begin();
    int num = data[start];
    int size = data.size();
    int end = (start + num) % size;

    if(!num) return;
    if(start + num == end){
        data[start] = 0;
        for(int i = start + 1; i <=end; ++i){
            ++data[i];
        } 
        print(data);
        return;
    } 

    int to_add = num / size;
    cout << "start " << start << endl;
    cout << "end " << end << endl;
    cout << "num " << num << endl;
    cout << "to_add " << to_add << endl;

    data[start] = 0;
    if(start == end){
        for(int i = 0; i <= size; ++i){ 
           data[i] += to_add;
        }

    } else if(end <= start){
        for(int i = 0; i <= end; ++i){ 
           data[i] += to_add+1;
        }
        for(int i = end + 1; i <= start; ++i){ 
           data[i] += to_add;
        }
        for(int i = start + 1; i < size; ++i){ 
           data[i] += to_add + 1;
        }
    } else {
        for(int i = 0; i < start + 1; ++i){ 
           data[i] += to_add;
        }
        for(int i = start + 1; i <= end; ++i){ 
           data[i] += to_add + 1;
        }
        for(int i = end + 1; i < size; ++i){ 
           data[i] += to_add;
        }
    }
    print(data);
}
