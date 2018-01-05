#include "include/helper.h"

enum Dir {DOWN = 1, UP = 2, LEFT = 4, RIGHT = 8};
const char *DIRS[] =  {"DOWN", "UP  ", "LEFT", "RIGHT"};
enum Way {BLANC = 0, VERTIC = 1, HORIZ = 2, CROSS = 4};

struct Field{
    int x,y;
    int width, height;
    vector<vector<int> > field;
    int last_dir;
    Field(vector<string> &lines):x(0),y(0),width(lines[0].size()),height(lines.size()), 
            field(vector<vector<int> >(height)), last_dir(0xf){
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

    inline bool check_dir(Dir dir){
        return last_dir & dir;
    }

    inline void set_last_dir(Dir dir){
        if(get_elem() == CROSS) last_dir = 0xf ^ dir;
        else last_dir = dir; 
    }
    inline bool valid(){
        return x > 0 && y > 0 && x < width && y < height;
    }
 
    Way skip_way(Way to_skip, int xoff, int yoff){

        if(!valid()) return BLANC;
        while(get_elem() == to_skip){
            x += xoff;
            y += yoff;
        } 
        return (Way) get_elem();
    }

    bool new_dir(Dir ndir, Way fway, int xoff, int yoff){
        if(!check_dir(ndir) ) return false;

        int xold = x, yold = y;
        x += xoff;
        y += yoff;
        auto nway = skip_way(fway, xoff, yoff);
        if(nway == BLANC) {
            x = xold;
            y = yold;
            return false;
        }
        if(field[yold][xold] != fway) field[yold][xold] = BLANC;
        set_last_dir(ndir);
        return true;
    }


    bool go_up(){

        return new_dir(UP, HORIZ, 0, -1);
    }
    bool go_down(){

        return new_dir(DOWN, HORIZ, 0, 1);
    }

    bool go_right(){

        return new_dir(RIGHT, VERTIC, 1,0);
    }

    bool go_left(){

        return new_dir(LEFT, VERTIC, -1, 0);
    }

    int get_elem(){
        return field[y][x];
    }
    void print(){
        cout << x << " , " << y << endl;
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
    string path = "";
    unsigned char elem;
    while(true){
        field.print();
        if((elem = (unsigned char) field.get_elem()) > 4) path += elem;
        if(field.go_down()) continue;
        if(field.go_left()) continue;
        if(field.go_right()) continue;
        if(field.go_up()) continue;
        break;
    }   
    cout << "path " << path << endl;
  
   	return 0;
}
