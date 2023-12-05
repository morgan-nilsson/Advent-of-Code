import java.util.Scanner;
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
            System.err.println("Error reading frim file");
            return;
        }
        int[] cardScores = new int[cardNumber];
        for(int i = 0; i < cardNumber; i++){
            cardScores[i] = findCardScores(input.nextLine());
        }
        input.close();
        printScores(cardScores);
    }

    public static int findCardScores(String line){
        // 0 - 99 two digit numbers
        Boolean[] checkMap = new Boolean[100];  
        int[] winners = parseCardWinners(line);
        for(int i = 0; i < winners.length; i++){
            System.out.printf("%d ", winners[i]);
        }
        System.out.println();
        int[] picks = parseCardPicks(line);
        for(int i = 0; i < picks.length; i++){
            System.out.printf("%d ", picks[i]);
        }

        for(int i = 0; i < winners.length; i++){
            checkMap[winners[i]] = true;
        }

        int wins = 0;
        for(int i = 0; i < picks.length; i++){
            if(checkMap[picks[i]] == true){
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
            return (int)Math.pow(2, wins);
        }
    }

    public static int[] parseCardWinners(String line){
        Scanner input = new Scanner(line);
        input.next();

        int[] winners = new int[8];
        String value = "";
        while (value.compareTo("|") != 0) {
            value = input.next();
        }
        input.next();
        int i = 0;
        while(input.hasNext()){
            winners[i] = Integer.parseInt(input.next());
            i++;
        }
        input.close();
        return winners;
    }

    public static int[] parseCardPicks(String line) { 
        Scanner input = new Scanner(line);
        input.next();

        int[] picks = new int[8];
        String value = input.next();
        int i = 0;
        while(value.compareTo("|") != 0) {
            value = input.next();
            picks[i] = Integer.parseInt(value);
            i++;
        }
        input.close();
        return picks;
    }
    public static void printScores(int[] scores) {
       for(int i = 0; i < scores.length; i++){
            System.out.printf("Card %d score: %d\n", i, scores[i]);
       }
    }
}
