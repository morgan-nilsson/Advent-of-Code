import java.util.Scanner;
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

        int lineCount = 0;
        while(input.hasNextLine()){
            input.nextLine();
            lineCount++;
        }

        input.reset();
        schem = new String[lineCount];
        for(int i = 0; i < lineCount; i++){
            schem[i] = input.nextLine();
        }


    }
}
