import java.util.Scanner;
import java.io.File;
import java.io.FileNotFoundException;

public class Required {
    public static void main(String[] args) {

        int ALPHA_LENGTH = 26;

        String[] binStrings = new String[ALPHA_LENGTH];

        if(args[0] == null){
            System.err.println("USAGE: java Required.java data.dat");
            return;
        }

        Scanner input;
        try {
            File file = new File(args[0]);
            input = new Scanner(file);
        } catch(FileNotFoundException e){
            System.err.println("Could not find the file with the given path");
            return;
        } catch (Exception e) {
            System.err.println("Could not open the file at the given path");
            return;
        }

        while(input.hasNextLine()){
            parse(input.nextLine());
        }
        input.close();
    }

    public static void parse(String line){
        
    }

    public static void setValue(char c, String bin){
        binStrings[c - 97] = bin;
    }

    public static String toBinary(int i){
        return Integer.toBinaryString(i);
    }

    public static int fromBinary(String bin) {
            return Integer.parseInt(bin , 2);    
    }

    public static void printBinDump(){
        for(int i = 0; i < ALPHA_LENGTH - 1; i++){
            if(binStrings[i] != null){
                System.out.printf("%c : %d", i + 97, fromBinary(binStrings[i]));
            }
        }
    }

    public static String bitwiseAND(char c, char d){
        String anded;
        for(int i = 0; i < binStrings[c - 97]; i++){
            anded.concat(binStrings[c - 97] & binStrings[c - 97]);
        }
        return anded;
    }

    public static String bitwiserOR(char c, char d){
        String ored;
        for(int i = 0; i < binStrings[c - 97]; i++){
            ored.concat(binStrings[c - 97] | binStrings[c - 97]);
        }
        return ored;
    }

    public static String bitShift(char c, int shift){
        StringBuffer shifted;
        for(int i = binStrings[c - 97].length; i > 0; i--){
            if(i + shift < binStrings[c - 97].length){
                shifted.insert(i + shift, binStrings[i]);
            }
        }
        return shifted;
    }
}
