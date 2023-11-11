import java.util.Scanner;

public class Wrapping {
    public static void main(String[] args) {
        Scanner inpScanner = new Scanner(System.in);
        System.out.printf("Please enter the width");
        int w = inpScanner.nextInt();
        System.out.printf("Please enter the height");
        int h = inpScanner.nextInt();
        System.out.printf("Please enter the length");
        int l = inpScanner.nextInt();

        int wrappingPaper = 2 * l * w + 2 * w * h + 2 * h * l;
        int extra = Math.min(Math.min(2, wrappingPaper), wrappingPaper);
    }    
}
