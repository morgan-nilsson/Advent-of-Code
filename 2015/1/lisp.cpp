#include <iostream>
#include <vector>
#include <string>

int main(void){
    std::string inputString;

    std::cout << "Please enter your Pattern";
    std::cin >> inputString;

    int floor;
    for(int i = 0; i < inputString.length(); i++){
        if(inputString.at(i) == '('){
            floor++;
        } else if(inputString.at(i) == ')'){
            floor--;
        } else{
            std::cout << "Error in the Pattern" << std::endl;
            return -1;
        }
        std::cout << inputString;
    }
    std::cout << "floor " << floor << std::endl;
}