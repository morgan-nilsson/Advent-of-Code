import java.util.Scanner;

public class Lisp{
    public static void main(String[] args) {
        Scanner input = new Scanner(System.in);
        System.out.printf("\nPlease enter your pattern: ");
        String inputString = input.next();
        input.close();

        int floor = 0;
        for(int i = 0; i < inputString.length(); i++){
            if(inputString.charAt(i) == '('){
                floor++;
            } else if(inputString.charAt(i) == ')'){
                floor--;
            } else{
                System.err.println("There where invalid characters in the pattern");
                return;
            }
        }

        System.out.printf("Floor %d\n", floor);
    }
}