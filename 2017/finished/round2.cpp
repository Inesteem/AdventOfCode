#include <iostream>
#include <string>
#include <vector>

using std::cout;
using std::endl;
using std::string;
using std::vector;

vector<vector<int> > input = {
{790,99,345,1080,32,143,1085,984,553,98,123,97,197,886,125,947},
{302,463,59,58,55,87,508,54,472,63,469,419,424,331,337,72},
{899,962,77,1127,62,530,78,880,129,1014,93,148,239,288,357,424},
{2417,2755,254,3886,5336,3655,5798,3273,5016,178,270,6511,223,5391,1342,2377},
{68,3002,3307,166,275,1989,1611,364,157,144,3771,1267,3188,3149,156,3454},
{1088,1261,21,1063,1173,278,1164,207,237,1230,1185,431,232,660,195,1246},
{49,1100,136,1491,647,1486,112,1278,53,1564,1147,1068,809,1638,138,117},
{158,3216,1972,2646,3181,785,2937,365,611,1977,1199,2972,201,2432,186,160},
{244,86,61,38,58,71,243,52,245,264,209,265,308,80,126,129},
{1317,792,74,111,1721,252,1082,1881,1349,94,891,1458,331,1691,89,1724},
{3798,202,3140,3468,1486,2073,3872,3190,3481,3760,2876,182,2772,226,3753,188},
{2272,6876,6759,218,272,4095,4712,6244,4889,2037,234,223,6858,3499,2358,439},
{792,230,886,824,762,895,99,799,94,110,747,635,91,406,89,157},
{2074,237,1668,1961,170,2292,2079,1371,1909,221,2039,1022,193,2195,1395,2123},
{8447,203,1806,6777,278,2850,1232,6369,398,235,212,992,7520,7304,7852,520},
{3928,107,3406,123,2111,2749,223,125,134,146,3875,1357,508,1534,4002,4417}
};
int star3_helper(vector<int> & line){
    auto size = line.size();
    if(!size || size == 1) return 0;
    int min =  line [0];
    int max =  line [0];
    for(size_t i = 0; i < size; ++i){
        if(line[i] < min ) min = line [i];
        if(line[i] > max ) max = line [i];
    }
    return max - min;
}
int star3(vector<vector<int> > &vec){
    int sum = 0;

    for(auto &i : vec){
        sum += star3_helper(i);
     }

    return sum;
}

int star4_helper(vector<int> &line){
    if(line.size() < 2) return 0;
    int sum = 0;

   float max, min, div;
    
    for(size_t i = 0; i < line.size() - 1; ++i){
        for(size_t j = i+1; j < line.size(); ++j){
           if(line[i] > line[j]){
                max = line[i];
                min = line[j];
           } else {
                min = line[i];
                max = line[j];

           }

           div = max/min;
           if(div == (int) div){
               sum += div;
           }
        
        }
    }
    return sum;

}
int star4(vector<vector<int> > &vec){
    int sum = 0;

    for(auto &i : vec){
        sum += star4_helper(i);
     }

    return sum;
}



int main(){
    vector<vector<int> > one = { 
    {5, 9, 2, 8},
    {9, 4, 7, 3},
    {3, 8, 6, 5}};
    cout << star4(one) << endl;
    cout << star4(input) << endl;

    return 0;
}
