import java.util.Scanner;
import java.util.Vector;
import java.io.File;
import java.io.FileNotFoundException;

/**
 * cards
 */
public class cards {

    public static void main(String[] args) {
       if(args[0] == null){
            System.err.println("USAGE java cards.java data.dat");
            return;
       }
        
       Scanner input;
       File file;
       try {
            file = new File(args[0]);
            input = new Scanner(file);
       } catch(FileNotFoundException e){
            System.err.println("File at path was not found");
            return;
       } catch (Exception e) {
            System.err.println("File at path was not able to be found");    
            return;
       }

       int cardNumber = 0;
        while(input.hasNextLine()){
            input.nextLine();
            cardNumber++;
        }

        input.close();
        try {
            input = new Scanner(file);
        } catch (Exception e) {
            System.err.println("Error reading from file");
            return;
        }
        int cardSum = 0;
        for(int i = 0; i < cardNumber; i++){
            cardSum += findCardScores(input.nextLine());
        }
        input.close();
        System.out.println(cardSum);
    }

    public static int findCardScores(String line){
        // 0 - 99 two digit numbers
        boolean[] checkMap = new boolean[100];  
        Vector<Integer> winners = parseCardWinners(line);

        Vector<Integer> picks = parseCardPicks(line);

        for(int i = 0; i < winners.size(); i++){
            checkMap[winners.get(i)] = true;
        }

        int wins = 0;
        for(int i = 0; i < picks.size(); i++){
            if(checkMap[picks.get(i)] == true){
                wins++;
            }
        }

        return points(wins);
    }

    public static int points(int wins){
        if(wins == 0){
            return 0;
        } else if(wins == 1){
            return 1;
        } else{
            return (int)Math.pow(2, wins - 1);
        }
    }

    public static Vector<Integer> parseCardWinners(String line){
        Scanner input = new Scanner(line);
        input.next();

        Vector<Integer> winners = new Vector<>();
        String value = "";
        while (value.compareTo("|") != 0) {
            value = input.next();
        }
        int i = 0;
        while(input.hasNext()){
            winners.add(i, Integer.parseInt(input.next()));
            i++;
        }
        input.close();
        return winners;
    }

    public static Vector<Integer> parseCardPicks(String line) { 
        Scanner input = new Scanner(line);
        input.next();
        input.next();
        Vector<Integer> picks = new Vector<>();
        String value = input.next();
        int i = 0;
        while(value.compareTo("|") != 0) {
            picks.add(i, Integer.parseInt(value));
            i++;
            value = input.next();
        }
        input.close();
        return picks;
    }
    public static void printScores(int[] scores) {
       for(int i = 0; i < scores.length; i++){
            System.out.printf("Card %d score: %d\n", i + 1, scores[i]);
       }
    }
}
