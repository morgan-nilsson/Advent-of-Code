import java.util.Scanner;
import java.util.Vector;
import java.io.File;
import java.io.FileNotFoundException;
/**
 * d3
 */
public class d3 {

    static public String[] schem;

    public static void main(String[] args) {
        if(args[0] == null){
            System.err.println("USAGE java d3.java data.dat");
            return;
        }

        Scanner input;
        try {
            File file = new File(args[0]);
            input = new Scanner(file);
        } catch(FileNotFoundException e){
            System.err.println("File at given path was not found");
            return;
        } catch (Exception e) {
            System.err.println("File could not be opened");
            return;
        }

        Vector<String> schem = new Vector<String>(); 
        while(input.hasNextLine()){
            schem.add(i, input.nextLine());
        }

        int sum = 0;
        for(int i = 0; i < schem.size(); i++){
            sum += lineSum(schem.get(i));
        }
    }

    public static int lineSum(String line){
        for(int i = 0; i < line.length(); i++){
            int index;
            if(Character.isDigit(line.charAt(i))){
                while (Character.isDigit(i + index)){
                    i++;
                }
            }
        }
    }

    

    public static boolean isPartNumber(int startIndex, int endIndex, int rowIndex, Vector<String> schem){
        for(int offset = 0; offset < startIndex - endIndex; offset++){
            if(isPart(startIndex + offset, rowIndex, schem) == false){
                return false;
            }
        }
        return true;
    }

    public static boolean isPart(int index, int rowIndex, Vector<String> schem){

    }
}
