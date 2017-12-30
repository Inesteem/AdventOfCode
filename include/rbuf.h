#ifndef R_BUFF_H
#define R_BUFF_H

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

};








#endif
