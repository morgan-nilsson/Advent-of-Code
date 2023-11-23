import java.util.Scanner;
import java.math.BigInteger;
import java.security.MessageDigest;
import java.security.NoSuchAlgorithmException;


public class SantaHash {
   

    public static void main() {

    MessageDigest md;

        try {
            md = MessageDigest.getInstance("MD5");        
        } catch (NoSuchAlgorithmException exception) {
            System.out.println("Error getting MD5 hash");
            return;
        }
    
        Scanner input = new Scanner(System.in);
        System.out.printf("Please enter the Secret Key");
        String key = input.next();
        input.close();

        byte[] digest = md.digest(key.getBytes());
        BigInteger digested = new BigInteger(1, digest);

        System.out.println(digested);
    }
}
