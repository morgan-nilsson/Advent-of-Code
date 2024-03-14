import java.io.File;
import java.util.Scanner;
import java.util.Vector;

class Yap{

    final static String fileName = "data.dat";
    public static void main(String[] args) {
        Scanner input;
        try {
            input = new Scanner(new File(fileName));
        } catch (Exception e) {
            e.printStackTrace();
            return;
        }
        Vector<String> strings = new Vector<>();
        while(input.hasNextLine()){
            strings.add(input.nextLine());
        }
    }
}