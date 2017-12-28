#include "helper.h"
#define INPUT

template <class T>
class Element{
	Element *next;
	Element *prev;
	T value;
public:

	Element(T value, Element *next = nullptr, Element *prev = nullptr): next(next), prev(prev), value(value){}

	Element *get_next(){return next;}
	Element *get_prev(){return prev;}
	T get_value(){return value;}

	void set_next(Element *next){
		this->next = next;
	//	next->prev = this;
	}
	void set_prev(Element *prev){
		this->prev = prev;
	//	prev->next = this;
	}
	void swap(Element *other){
		//std::swap(id, other->id);
		std::swap(value, other->value);
		
	}
};


template <class T>
class Ringbuffer{
	size_t num_elems;
	Element<T> *last;
	Element<T> *first;
	Element<T> *pos;
	
public:

	Ringbuffer(): num_elems(0), last(nullptr), first(nullptr), pos(nullptr){
		
	}
	~Ringbuffer(){

		if(!num_elems){ return; }

		if(num_elems == 1){
			delete first;
			return;		
		}

		Element<T> *tmp = first;	
		do {
			tmp = first->get_next();
			delete first;
			first = tmp;
		} while(first != last);
	}
	void reset_pos(){
		pos = first;
	}

	void append(T value){
		if(!num_elems){
			first = new Element<T>(value);
			first->set_next(first);
			last = first;
			pos = first;
		} else {
			Element<T> *new_elem = new Element<T>(value, first, last);
			first->set_prev(new_elem);
			last->set_next(new_elem);
			last = new_elem;
		}
		++num_elems;
	}

	void advance(size_t num){
		while(num){
			pos = pos->get_next();
			--num;
		}
	}  

	void reverse(size_t num){
		if(num <= 1) return;
		size_t half = num/2;
		Element<T> *start = pos;
		Element<T> *end = pos;
		while(num > 1){
		//	cout << end->get_value() << " -> ";
			end = end->get_next();
			--num;
		}		
//		cout << end->get_value() << ";" << endl;
		while(half){
//			cout << start->get_value()<< " <-> " << end->get_value() << endl;
			start->swap(end);
			start = start->get_next();
			end   = end->get_prev();
			--half;
		}
	} 
	
	size_t size(){
		return num_elems;
	}

	vector<T> get_vec(){
		vector<T> values(num_elems);
		Element<T> *tmp = first;
		for(int i = 0; i < num_elems; ++i){
			values[i] = tmp->get_value();
			tmp = tmp->get_next();
		}
		return values;
	}
	T get_mul(){
		return first->get_value() * first->get_next()->get_value();
	}

};




void print_s(vector<int> &vec, int s_pos, int e_pos){
 	auto size = vec.size();
    cout << "{ ";
    for(int i = 0; i < size-1; ++i){
		if(i == s_pos) cout << "(";
        cout << vec[i];
		if(i == e_pos) cout << ")";
		cout << " , ";
	}
	
	if(size-1 == s_pos) cout << "(";
    cout << vec[size-1];
	if(size-1 == e_pos) cout << ")";
	cout << " }" << endl;
}

vector<int> get_sparse_hash(string str){
    Ringbuffer<int> rb;
	for(int i = 0; i < 256; ++i){rb.append(i);}

	vector<int> coms;
	for(int i = 0; i < str.size(); ++i){
		coms.push_back((unsigned char)str[i]);
	}
	
	coms.push_back(17);
	coms.push_back(31);
	coms.push_back(73);
	coms.push_back(47);
	coms.push_back(23);
	auto size = rb.size();
	int skip = 0;
	for(int i = 0; i < 64; ++i){	
		for( auto & com : coms ){
			assert(com <= size);
		//		cout << "reverse " << com << " elems!" << endl;
			rb.reverse(com);
			rb.advance(com + skip); 
			++skip;	
		}
	}
//	cout << "mul of first elements : " << rb.get_mul() << endl;
	return rb.get_vec();
}
vector<int> get_dense_hash(vector<int> &vec){
	assert(vec.size() == 256);
	vector<int> ret(16);
	for(int i = 0; i < 16; ++i){
			ret[i] = 0;
		for(int j = 0; j < 16; ++j){
			ret[i] ^= vec[i * 16 + j];
		}
	}
	return ret;
}

void print_hash(vector<int> &hash){
	for(auto & ch : hash){
		cout << std::hex << std::setfill('0') << std::setw(2) << ch;
	}
	cout << endl;
}

void test(){

	auto sparse = get_sparse_hash("");
	auto dense = get_dense_hash(sparse);
	print_hash(dense);
	cout << "a2582a3a0e66e6e86e3812dcb672a272" << endl;
	cout << "----------------------------------------" << endl;

	sparse = get_sparse_hash("AoC 2017");
	dense = get_dense_hash(sparse);
	print_hash(dense);
	cout << "33efeb34ea91902bb2f59c9920caa6cd" << endl;
	cout << "----------------------------------------" << endl;


	sparse = get_sparse_hash("1,2,3");
	dense = get_dense_hash(sparse);
	print_hash(dense);
	cout << "3efbe78a8d82f29979031a4aa0b16a9d" << endl;
	cout << "----------------------------------------" << endl;

	sparse = get_sparse_hash("1,2,4");
	dense = get_dense_hash(sparse);
	print_hash(dense);
	cout << "63960835bcdc130f0b66d7ff4f6a5a8e" << endl;
	cout << "----------------------------------------" << endl;
}

int main(int argc, char *argv[]){
    if(argc != 2){
        cout << "wrong parameter count, submit string to hash!" << endl;
        exit(0);
    }

	//sparse = get_sparse_hash("14,58,0,116,179,16,1,104,2,254,167,86,255,55,122,244");
	auto sparse = get_sparse_hash(string(argv[1]));
	auto dense = get_dense_hash(sparse);

	print_hash(dense);





	return 0;
}
