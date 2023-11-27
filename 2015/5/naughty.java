import java.util.Scanner;
import java.io.FileNotFoundException;
import java.io.File; 

public class naughty {

    public static void main(String[] args) {

        if(args[0] == null){
            System.err.println("USAGE java naughty.java <FILEPATH>");
            return;
        }
        File file = null;
        Scanner input = null;
        try {
            file = new File(args[0]);     
            input = new Scanner(file);
        } catch (FileNotFoundException e){
            System.err.println("Unable to find the given file");
        } catch (Exception e) {
            System.err.println("Unable to read from given file");
        }
    
       String input_String;
       while(input.hasNext()){
            input_String = input.next();
            if(!isNice(input_String)){
                printExitNaughty(input_String);
            }
       }
       input.close();
    }

    public static boolean isNice(String input_String) {
        short vowel = 0;
        boolean repeatLetter = false;

        for(int i = 0;i < input_String.length(); i++){
            if(isVowel(input_String.charAt(i))){
                vowel++;
            }
            if(isNaughtySubStrings(input_String, i)){
                return false;
            }
            if(isRepeat(input_String)){
                repeatLetter = true;
            }
        }

        if(vowel < 3){
            return false;
        }
        if(!repeatLetter){
            return false;
        } else{
            return true;
        }
    }

    public static boolean isVowel(char c){
         char[] vowels = {'a', 'i', 'e', 'o', 'u'};

        for(int i = 0; i < vowels.length; i++){
            if(c == vowels[i]){
                return true;
            }
        }
        return false;
    }

    public static boolean isNaughtySubStrings(String input_String, int index) {
       String substring;

        try {
            substring = input_String.substring(index, index + 2);
       } catch (Exception e) {
            return false;
       }
        return isNaughtySubStringsComparison(substring);
    }

    public static boolean isNaughtySubStringsComparison(String substring) {
        String[] naughty = {"ab", "cd", "pq", "xy"};
        for(int i = 0; i < naughty.length; i++){
            if(substring.compareTo(naughty[i]) == 0){
                return true;
            }
        }
        return false;
    }

    public static boolean isRepeat(String input_String) {
        boolean hasRepeat = false;

       for(int i = 0; i < input_String.length() - 1; i++){
            if(input_String.charAt(i) == input_String.charAt(i + 1)){
                hasRepeat = true;
            }
       }
       return hasRepeat;
    }
    
    public static void printExitNaughty(String input_String) {
        System.out.println("The String " + input_String + " is naughty");
    }
}
