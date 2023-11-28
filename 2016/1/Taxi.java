import java.util.Scanner;
import java.io.FileNotFoundException;
import java.io.File;

public class Taxi {
    public static void main(String[] args) {
        int x = 0;
        int y = 0;
        
        if(args[0] == null){
            System.err.println("USAGE java Taxi.java <filepath>");
            return;
        }

        Scanner input;
        try{
            File file = new File(args[0]);
            input = new Scanner(file);
        } catch(FileNotFoundException e){
            System.err.println("File give nwas not found");
            return;
        } catch (Exception e){
            System.err.println("File was unable to be opened");
            return;
        }

        String input_String;
        while (input.hasNext()) {
            input_String = input.next();
            char direction = input_String.charAt(0);
            int distance = Integer.valueOf(input_String.charAt(1));
            move(direction, distance);
        }

        input.close();

        System.out.printf("The Easter Bunny Headquarters is %d away",  distance(x, y));
    }

    public static void move(char direction, int distance){
        int distance_vector = distance;
        int facing = 0;
        switch (direction) {
            case 'R':
                
                break;
            case 'L':
                distance_vector = distance_vector * -1;
                break;
            default:
                System.err.println("Error in the given directions");
                break;
        }
    }

    public static int distance(int x, int y){
        return x + y;
    }
}