import java.util.Scanner;
import java.util.Vector;
import java.io.File;
import java.io.FileNotFoundException;

public class Required {

    static int ALPHA_LENGTH = 26;
    static String[] binStrings = new String[ALPHA_LENGTH];
    public static void main(String[] args) {


        if(args[0] == null){
            System.err.println("USAGE: java Required.java data.dat");
            return;
        }

        Scanner input = null;
        try {
            File file = new File(args[0]);
            input = new Scanner(file);
        } catch(FileNotFoundException e){
            System.err.println("Could not find the file with the given path");
            input.close();
            return;
        } catch (Exception e) {
            System.err.println("Could not open the file at the given path");
            return;
        }

        while(input.hasNextLine()){
                try {
                    parse(input.nextLine());
                    printBinDump();
                } catch (Exception e) {
                    System.err.println("Error in the format of commands");
                    input.close();
                    e.printStackTrace();
                    return;
                }
        }
        input.close();
    }

    public static void parse(String line){
       Scanner input_line = new Scanner(line);
        Vector<String> commands = new Vector<String>(); 
        int cmdNum = 0;
        char result = (char)-1;
        char in1;
        char in2;
        String command;
        int value;
        boolean found = false;
        while(input_line.hasNext()){
            commands.add(input_line.next());
            cmdNum++;
        }
        for(int i = 0; i < commands.size(); i++){
            if(commands.get(i).compareTo("->") == 0){
                result = commands.get(i + 1).charAt(0);
            }
        }
        if(result == (char)-1){
            System.err.println("Bad instruction set");
            throw new java.lang.IllegalArgumentException(); 
        }
        for(int i = 0; i < commands.size(); i++){
            String cmd = commands.get(i);
            if(cmd.compareTo("AND") == 0){
                binStrings[result - 97] = bitwiseAND(commands.get(i - 1).charAt(0), commands.get(i + 1).charAt(0));
            } else if(cmd.compareTo("OR") == 0){
                binStrings[result - 97] = bitwiserOR(commands.get(i - 1).charAt(0), commands.get(i + 1).charAt(0));
            } else if(cmd.compareTo("NOT") == 0){
                bitwiseNOT(commands.get(i + 1).charAt(0));
            } else if(cmd.compareTo("RSHIFT") == 0){
                bitRShift(commands.get(i - 1).charAt(0), Integer.parseInt(commands.get(i + 1)));
            } else if(cmd.compareTo("LSHIFT") == 0){
                bitLSHIFT(commands.get(i - 1).charAt(0), Integer.parseInt(commands.get(i + 1)));
            }
        }
        if(!found){
            setValue(commands.get(2).charAt(0), toBinary(Integer.parseInt(commands.get(0))));
        }
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
        int ALPHA_LENGTH = 26;
        for(int i = 0; i < ALPHA_LENGTH - 1; i++){
            if(binStrings[i] != null){
                System.out.printf("%c : %d", i + 97, fromBinary(binStrings[i]));
            }
        }
        System.out.println();
    }

    public static String bitwiseAND(char c, char d){
        String anded = "";
        for(int i = 0; i < binStrings[c - 97].length(); i++){
            anded.concat(Integer.toString(Integer.parseInt(binStrings[c - 97]) & Integer.parseInt(binStrings[c - 97])));
        }
        return anded;
    }

    public static String bitwiserOR(char c, char d){
        String ored = "";
        for(int i = 0; i < binStrings[c - 97].length(); i++){
            ored.concat(Integer.toString(Integer.parseInt(binStrings[c - 97]) | Integer.parseInt(binStrings[c - 97])));
        }
        return ored;
    }

    public static String bitRShift(char c, int shift){
        StringBuffer shifted = new StringBuffer("");
        for(int i = binStrings[c - 97].length(); i > 0; i--){
            if(i + shift < binStrings[c - 97].length()){
                shifted.insert(i + shift, Integer.parseInt(binStrings[i]));
            }
        }
        return shifted.toString();
    }

    public static String bitLSHIFT(char c, int shift){
        StringBuffer shifted = new StringBuffer("");
       for(int i = 0; i < binStrings[c - 97].length(); i++){
            if(i - shift >= 0){
                shifted.insert(i - shift, Integer.parseInt(binStrings[i]));
            }
       }
       return shifted.toString();
    }

    public static String bitwiseNOT(char c){
        StringBuffer noted = new StringBuffer("");
        for(int i = 0; i < binStrings[c - 97].length(); i++){
            if(binStrings[c - 97].charAt(i) == '0'){
                noted.insert(i, '1');
            } else{
                noted.insert(i, '0');
            }
        }
        return noted.toString();
    }
}
