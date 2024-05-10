#include <ostream>
#include <sstream>
#include <iostream>
#include <string>
#include <fstream>
#include <vector>

int calcWrapping(int l, int w, int h){
	int wrappingPaper = 2 * l * w + 2 * w * h + 2 * h * l;
	int extra = std::min(std::min(l * w, w * h), h * l);
	return wrappingPaper + extra;
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
