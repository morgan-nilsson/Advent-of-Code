import java.util.Scanner;

public class Wrapping {
    public static void main(String[] args) {

        Scanner inpScanner;
        try {
            File file = new File(args[0]);
            inpScanner = new Scanner(file);
        } catch (Exception e) {
            return;
        }

        int sum = 0;
        while (inpScanner.hasNextLine()) {
            sum += calculateWrapping(inpScanner.nextLine());
        }
        System.out.println(sum);

    }

    public static int calculateWrapping(String line) {
        int wrappingPaper = 2 * l * w + 2 * w * h + 2 * h * l;
        int extra = Math.min(Math.min(2, wrappingPaper), wrappingPaper);
        return wrappingPaper + extra;
    }
}
