import java.util.Scanner;

public class SantaMap {
    public static void main(String[] args) {
        System.out.println("Please enter Santa's path");
        Scanner input = new Scanner(System.in);
        String santaPath = input.next();
        input.close();

        int x = 0;
        int y = 0;

        PointMap pointMap = new PointMap();
        pointMap.setPoint(x, y);
        for(int i = 0; i < santaPath.length(); i++){
            if(pointMap.getPoint(x, y) == false){
                pointMap.setPoint(x, y);
            }

            switch (santaPath.charAt(i)) {
                case '^':
                    y++; 
                    break;
                case '>':
                    x++;
                    break;

                case 'v':
                    y--;
                    break;

                case 'V':
                    y--;
                    break;

                case '<':
                    x--;
                    break;

                default:
                    System.out.println("Error reading path");
                    return;
            }
        }
        pointMap.setPoint(x, y);
        System.out.println("Santa visited " + pointMap.getSize() + " Houses");
    }
}
