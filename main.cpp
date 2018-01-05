#include "include/helper.h"

enum Dir {FIN = 0, DOWN = 1, UP = 2, LEFT = 4, RIGHT = 8};
const char *DIRS[] =  {"FIN ", "DOWN", "UP  ", "LEFT", "RIGHT"};
enum Way {BLANC = 0, VERTIC = 1, HORIZ = 2, CROSS = 4};

struct Field{
    int x,y;
    int width, height;
    vector<vector<int> > field;
    string path;
    Dir dir;
    Field(vector<string> &lines):x(0),y(0),width(lines[0].size()),height(lines.size()), 
            field(vector<vector<int> >(height)), path(""), dir(DOWN){
        x = lines[0].find("|");
        assert(x != std::string::npos);
        int width = lines[0].size();

        for(int i = 0; i < height; ++i){
            assert(width == lines[i].size());
            field[i] = vector<int>(width);

            for(int j = 0; j < width; ++j){
                if(     lines[i][j] == '|') field[i][j] = VERTIC; 
                else if(lines[i][j] == '-') field[i][j] = HORIZ; 
                else if(lines[i][j] == '+') field[i][j] = CROSS; 
                else if(lines[i][j] == ' ') field[i][j] = BLANC; 
                else field[i][j] = lines[i][j]; 
            }
        }
    }
    inline int get_elem(){
        return field[y][x];
    }
    inline bool out_of_bounce(){

        if(y < 0 || x < 0 || x >= width || y >= height || get_elem() == BLANC){
            return true;
        }
        return false;
    }

    inline void set_new_dir(){
        if(dir != UP && dir != DOWN){
            if(y && field[y-1][x] != BLANC)
                dir = UP; 
            else if(y < height - 1 && field[y+1][x] != BLANC)
                dir = DOWN;
            return ;
        }
        
        if(dir != LEFT && dir != RIGHT){
            if(x && field[y][x-1] != BLANC)
                dir = LEFT; 
            
            if(x < width - 1 && field[y][x+1] != BLANC)
                dir = RIGHT; 
            return;
        }
        
    }

    string go(){
        
        int elem;
        while(true){
            print();
            elem = get_elem();
            if(elem > CROSS){
                path += (unsigned char) elem;
            } 
            else if(elem == CROSS){
                set_new_dir();
            }
            
            if(dir == UP)           --y;
            else if(dir == DOWN)    ++y;
            else if(dir == RIGHT)   ++x;
            else if(dir == LEFT)    --x;
            if(out_of_bounce()) break;
        }

        return path;
    }

    void print(){
        cout << x << " , " << y << ": " << path << endl;
        return;
        for(int h = 0; h < height; ++h){
            for(int w = 0; w < width; ++w){
                auto elem = field[h][w];    
                if(h == y && w == x) cout << " #";
                else if(!elem) cout << "  ";
                else if(elem <= 4) cout << " " << elem;
                else  cout << " " << (unsigned char) elem;
            }
            cout << endl;
        }   
        cout << endl;
    
    }
};



int main(int argc, char *argv[]){


	if(argc != 2){
        cout << "wrong parameter count, submit filename!" << endl;
        exit(0);
    }
    string output;
	auto lines = get_one_line("\n", argv[1], true);
    Field field(lines);
    string path = field.go(); 
    cout << "path " << path << endl;
  
   	return 0;
}
