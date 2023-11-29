import java.util.Scanner;
import java.io.File;
import java.io.FileNotFoundException;

public class Lights {

    static int lightsSize = 1000;
    static boolean[][] lights = new boolean[lightsSize][lightsSize];

    public static void main(String[] args) {

        if(args[0] == null){
            System.err.println("USAGE: java Lights.java <filepath>");
            return;
        }
        Scanner input = null;
       try {
            File file = new File(args[0]);
            input = new Scanner(file);
       } catch (FileNotFoundException e) {
            System.err.println("Could not find the given file");
       } catch (Exception e){
            System.err.println("Could not open given file");
       }

       String command;
       while(input.hasNextLine()){
            command = input.nextLine();
            parseInput(command);
       }
       input.close();
       printResults();
    }

    public static void parseInput(String command_StringLine) {
        Scanner input_StringLine = new Scanner(command_StringLine);
        String command_String = input_StringLine.next();

        int startX;
        int startY;
        int endX;
        int endY;

        if(command_String.compareTo("toggle") == 0){
            startX = input_StringLine.nextInt();
            startY = input_StringLine.nextInt();
            input_StringLine.next();
            endX = input_StringLine.nextInt();
            endY = input_StringLine.nextInt();

            toggle(startX, startY, endX, endY);
        } else{
            command_String = input_StringLine.next();
            
            startX = input_StringLine.nextInt();
            startY = input_StringLine.nextInt();
            input_StringLine.next();
            endX = input_StringLine.nextInt();
            endY = input_StringLine.nextInt();

            if(command_String.compareTo("on") == 0){
                lightsSet(true, startX, startY, endX, endY);
            } else if(command_String.compareTo("off") == 0){
                lightsSet(false, startX, startY, endX, endY);
            } else{
                System.err.println("Error reading input invalid input");
                System.exit(0);
            }
        }
        input_StringLine.close();
    }

    public static void lightsSet(boolean set, int startX, int startY, int endX, int endY) {
          for(int x = startX; x <= endX; x++){
                for(int y = startY; y <= endY; y++){
                    lights[x][y] = set;
                }
          }
    }


    public static void toggle(int startX, int startY, int endX, int endY){
        for(int x = startX; x <= endX; x++){
            for(int y = startY; y <= endY; y++){
                lights[x][y] = !lights[x][y];
            }
        }
    }

    public static void printResults() {
        int on = 0;
        for(int x = 0; x < lightsSize; x++){
            for(int y = 0; y < lightsSize; y++){
                if(lights[x][y] == true){
                    on++;
                }
            }
        }

        System.out.println(on + " Lights are lit up");
    }
}
