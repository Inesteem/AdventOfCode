#include <iostream>
#include <fstream>
using namespace std;


int main(int argc, const char * argv[]){
	std::ifstream infile("../data/day1.txt");
	long sum = 0;
	int mass;
	while(infile >> mass){
		int fuel = mass;
		while(true){
			fuel = fuel/3 - 2;
			if (fuel <= 0) break;
			sum += fuel;
		}
	}
	
	cout << sum << endl; 
	return 0;
}
