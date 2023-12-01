import java.util.Scanner;
import java.io.File;
import java.io.FileNotFoundException;

public class day1 {

    public static void main(String[] args) {
        if(args[0] == null){
            System.err.println("USAGE: java day1.java data.dat");
            return;
        }
        Scanner input = null;
        try {
            File file = new File(args[0]); 
            input = new Scanner(file);
        } catch (FileNotFoundException e){
            System.err.println("File at the given path was not found");
            return;
        } catch (Exception e) {
            System.err.println("File at the path was not able to be opened");
            return;
        }

        int sum = 0;
        while (input.hasNext()) {
            String s = "";
            String line = input.nextLine();
            s + findFirst(line);
            s + findLast(line);
            sum += Integer.parseInt(s);
        }
        input.close();
        System.out.println(sum);
    }

    public static int findFirst(String line) {
        for(int i = 0; i < line.length(); i++){
            try {
                int first = Integer.parseInt(line.charAt(i));
            } catch (NumberFormatException e) {
                continue;
            }
            return first;
        }
    }

    public static int findLast(String line) {
        
        for(int i = line.length(); i > 0;i--){
            try {
                int last = Integer.parseInt(line.charAt(i));
            } catch (NumberFormatException e) {
                continue;
            }
            return last;
        }
    }
}
