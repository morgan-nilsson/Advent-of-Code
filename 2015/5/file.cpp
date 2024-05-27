#include <iostream>
#include <fstream>
#include <string>

using namespace std;

bool isNice(string str){
	cout << str;
	if(str == ""){
		return false;
	}
	char vowels [] = {'a','e','i','o','u'};
	bool nice = true;

	int vowelCount = 0;
	for(int i = 0; i < str.length(); i++){
		for(int j = 0; j < 5; j++){
			if(str.at(i) == vowels[j]){
				j = 5;
				vowelCount++;
			}
		}
	}
	bool repeats = 0;
	bool badRepeat = 0;
	string seq [] = {"ab","cd","pq","xy"};
	for(int i = 0; i < str.length()-1; i++){
		if(str.at(i)==str.at(i+1)) repeats = 1;
		for(int j = 0; j < 4; j++){
			if(str.at(i) == seq[j].at(0) && str.at(i+1) == seq[j].at(1)) badRepeat = 1;
		}
	}

	// better way to do but im too lazy
	if(vowelCount < 3) nice = 0;
	if(!repeats) nice = 0;
	if(badRepeat) nice = 0;
	return nice;
}

int main(void){
	fstream file;
	file.open("test.dat", ios::out);
	int totalNice = 0;
	if(file.is_open()){
		while(file){
			string s;
			getline(file, s);
			if(isNice(s)) totalNice++;
		}
	}
	cout << totalNice;
}


