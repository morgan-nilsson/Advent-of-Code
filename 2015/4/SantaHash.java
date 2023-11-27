import java.util.Scanner;
import java.math.BigInteger;
import java.security.MessageDigest;
import java.security.NoSuchAlgorithmException;


public class SantaHash {
   

    public static void main(String[] args) {

    MessageDigest md;

        try {
            md = MessageDigest.getInstance("MD5");        
        } catch (NoSuchAlgorithmException exception) {
            System.out.println("Error getting MD5 hash");
            return;
        }
    
        Scanner input = new Scanner(System.in);
        System.out.printf("Please enter the Secret Key: ");
        String key_String = input.next();
        input.close();

        int i = 0;
        while(true){
            String key = key_String + i;
            byte[] digest = md.digest(key.getBytes());
            BigInteger digested = new BigInteger(1, digest);
            if(isTrue(digest)){
                break;
            }
        }
        PrintCorrect(i);
    }
        
    public static Boolean isTrue(Byte[] digest) {
        int i = 0;
       if(digest[i++] || digest[i++] || digest[i++] || digest[i++] || digest[i++]){
            return false;
       } else {
            return true;
       }
    }
    public static void PrintCorrect(int correct) {
       System.out.println(correct); 
    }
}
