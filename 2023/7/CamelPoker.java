import java.util.Scanner;
import java.io.File;
import java.io.FileNotFoundException;

public class CamelPoker {

    public static String[] decks;
    public static int[] bids;
    public static void main(String[] args) {
        Scanner input = null;
        try {
            File file = new File(args[0]);
            input = new Scanner(file);
        } catch (Exception e) {
            System.err.println("Could not open file");
        }


        int deckIndex = 0;
        while(input.hasNext()){
            decks[deckIndex] = input.next();
            bods[deckIndex] = input.nextInt();
            deckIndex++;
        }
    }

    public static void swapDeck(int firstIndex, int secondIndex){
        int tmp = bids[secondIndex];
        bids[secondIndex] = bids[firstIndex];
        bids[firstIndex] = tmp;
    }

    public static int winningDeck(int deck1Index, int deck2Index){
        int deck1Class = 0;
        int deck2Class = 0;
        String deck1 = decks[deck1Index];
        String deck2 = decks[deck2Index];

        deck1Class = calcClass(deck1);
        deck2Class = calcClass(deck2);
        if(deck1Class == deck2Class){
            if(calcTieBreaker(deck1.charAt(0)) > calcTieBreaker(deck2.charAt(0))){
                return 1;
            } else{
                return 2;
            }
        }
    }

    public static int calcClass(int deckIndex){
        String deck = decks[deckIndex]
       if(deck.charAt(0) == deck.charAt(1) && deck.charAt(2) == deck.charAt(3) && deck.charAt(5) == deck.charAt(0) && deck.charAt(0) == deck.charAt(2)){
            return 7;
       }
    }

    public static int calcTieBreaker(char c){
        switch (c) {
            case j:
                return 11;
            case q:
                return 12;
            case k:
                return 13;
            case a:
                return 14;
            default:
                return Integer.parseInt(c);
        }
    }
}
