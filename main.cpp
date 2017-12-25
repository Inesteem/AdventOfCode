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
			cout << end->get_value() << " -> ";
			end = end->get_next();
			--num;
		}		
		cout << end->get_value() << ";" << endl;
		while(half){
			cout << start->get_value()<< " <-> " << end->get_value() << endl;
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

void test(){
    Ringbuffer<int> rb;
#ifdef INPUT
	for(int i = 0; i < 256; ++i){rb.append(i);}
	vector<int> coms  = {14,58,0,116,179,16,1,104,2,254,167,86,255,55,122,244};
#else
	for(int i = 0; i < 5; ++i){rb.append(i);}
	vector<int> coms = {3,4,1,5};
#endif
	auto size = rb.size();
	int skip = 0;
	
	for( auto & com : coms ){
		print(rb.get_vec());
		assert(com <= size);
			cout << "reverse " << com << " elems!" << endl;
		rb.reverse(com);
		rb.advance(com + skip); 
		++skip;	
	}
	print(rb.get_vec());
	cout << "mul of first elements : " << rb.get_mul() << endl;
	exit(0);

}

int main(int argc, char *argv[]){
//	test();

#ifdef INPUT
	vector<int> elems(256);
	for(int i = 0; i < 256; ++i){elems[i] = i;}
	vector<int> coms  = {14,58,0,116,179,16,1,104,2,254,167,86,255,55,122,244};
#else
	vector<int> elems = {0,1,2,3,4};
	vector<int> coms = {3,4,1,5};
#endif
	auto size = elems.size();

	int s_pos = 0;
	int pos = 0;
	int e_pos = 0;
	int skip = 0;

	for(auto &com : coms){
		assert(com <= size);
		s_pos = pos;	
		e_pos = (pos + com - 1 ) % size;
		print_s(elems, s_pos, e_pos);
		cout << "s_pos : " << s_pos <<  " : " << elems[s_pos] << endl; 
		cout << "e_pos : " << e_pos <<  " : " << elems[e_pos] << endl; 
		if(e_pos < s_pos){
			while(e_pos >= 0 && s_pos < size){
				std::swap(elems[e_pos], elems[s_pos]);
				--e_pos;
				++s_pos;
			}	
			if(e_pos < 0) e_pos = size-1;
			else if(s_pos == size) s_pos = 0;
			cout << "\t"; print(elems);
			cout << "\t"; 
		cout << "s_pos : " << s_pos <<  " : " << elems[s_pos] << endl; 
			cout << "\t"; 
		cout << "e_pos : " << e_pos <<  " : " << elems[e_pos] << endl; 

		}
		while(e_pos > s_pos){
			std::swap(elems[e_pos], elems[s_pos]);
			--e_pos;
			++s_pos;
		}
		pos += (com + skip);
		pos %= size;
		++skip;
	}	

		print(elems);
		cout << "mul of first elements : " << elems[0] * elems[1] << endl;

	return 0;
}
