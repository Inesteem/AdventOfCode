#ifndef R_BUFF_H
#define R_BUFF_H

#include<algorithm>
#include<unordered_map>
#include<ostream>

using std::ostream;

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
    //      next->prev = this;
    }
    void set_prev(Element *prev){
            this->prev = prev;
    //      prev->next = this;
    }
    void swap(Element *other){
            //std::swap(id, other->id);
            std::swap(value, other->value);
            
    }

    bool operator==(Element &elem){
        return elem.value == value;
    } 
    bool operator!=(Element &elem){
        return elem.value != value;
    } 

    template<class T2> 
    friend ostream & operator <<(ostream &op,Element<T2> &); 
};

template <typename T> ostream & operator <<(ostream &op,Element<T> &elem){
    op << elem.value;
    return op; 
}
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
    //day16
    void spin(size_t num){
        if(!num_elems) return;
        num = num % num_elems;
        while(num){
            last = last->get_prev();
            first = first->get_prev();
            --num;
        }
    }
    //day16
    bool exchange(size_t pos1, size_t pos2){
        if(pos1 > num_elems || pos2 > num_elems) return false;
        if(pos1 == pos2) return true;
        auto min = std::min(pos1, pos2);
        auto max = std::max(pos1, pos2) - min;
        auto elem1 = first;
        while(min--){
           elem1 = elem1->get_next(); 
        }

        auto elem2 = elem1;
        while(max--){
           elem2 = elem2->get_next(); 
        }
        elem1->swap(elem2);
        return true;
    }
    
    //day16 
    bool swap(T val1, T val2){
        if(val1 == val2) return true;
        auto s = num_elems;
        decltype(first) elem1 = nullptr;
        decltype(first) elem2 = nullptr;
        auto p = first;
        while(s-- && (!elem1 || !elem2)){
            if(p->get_value() == val1)
                elem1 = p; 
            else if(p->get_value() == val2)
                elem2 = p; 
            
            p = p->get_next(); 
        }

        if(!elem1 || !elem2) return false;  
        elem1->swap(elem2);
        return true;
    }


        void reverse(size_t num){
                if(num <= 1) return;
                size_t half = num/2;
                Element<T> *start = pos;
                Element<T> *end = pos;
                while(num > 1){
                //      cout << end->get_value() << " -> ";
                        end = end->get_next();
                        --num;
                }               
//              cout << end->get_value() << ";" << endl;
                while(half){
//                      cout << start->get_value()<< " <-> " << end->get_value() << endl;
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
    bool operator ==(Ringbuffer &rb){
        if(num_elems != rb.num_elems) return false;
        auto elem1 = first;
        auto elem2 = rb.first;
        for(int i = 0; i < num_elems; ++i){
            if((*elem1) != (*elem2)) return false;
            elem1 = elem1->get_next();
            elem2 = elem2->get_next();
        }
        return true;
    }

    template<class T2> 
    friend ostream & operator <<(ostream &op,Ringbuffer<T2> &); 
};

template<typename T> ostream & operator <<(ostream &op,Ringbuffer<T> &rb) {
    auto size = rb.size();
    op << "RB[" << size <<"]: { ";
    auto elem = rb.first;
/*    for(int i = 1; i < size; ++i){
        op << *elem << ", ";
        if( i && !(i % 10)) op << "\n\t";
        elem = elem->get_next();
    }
*/ 
    for(int i = 1; i < size; ++i){
        op << *elem;
        elem = elem->get_next();
    }

    op << *elem << " }";
    return op;
}






#endif
