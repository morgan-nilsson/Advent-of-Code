#include <ostream>
#include <sstream>
#include <iostream>
#include <string>
#include <fstream>
#include <vector>

int calcWrapping(int l, int w, int h){
	int volume = l * w * h;
	int absMin = std::min(std::min(l, w), h);
	int min_two = std::min(std::max(l,w),h);
	int perim = 2*absMin + 2*min_two;
	std::cout << absMin << " " << min_two;
	std::cout << "volume:" << volume << "\nPerim " << perim << "\n";
	return volume + perim;;
}

int parseAndStuff(std::string line){
	if(line == "") return 0;
	std::cout << line << std::endl;
	std::stringstream stream(line);
	std::string token;
	std::vector<std::string> tokens;
	char delimit = 'x';
	while(std::getline(stream, token, delimit)){
		tokens.push_back(token);
	}
	// c is a real lang
	return calcWrapping(atoi(tokens.at(0).c_str()),atoi(tokens.at(1).c_str()),atoi(tokens.at(2).c_str()));
}

int main(void){
	std::string inputLine;
	std::ifstream dat;
	dat.open("data.dat");
	long long sum = 0;
	if(dat.is_open()){
		while(dat.good()){
			std::getline(dat, inputLine);
			sum += parseAndStuff(inputLine);
		}
	}
	std::cout << sum << std::endl;
}
