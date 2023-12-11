import java.util.Scanner;
import java.util.Vector;
import java.io.File;
import java.io.FileNotFoundException;
/**
 * d3
 */
public class d3 {


	public static Vector<String> schem = new Vector<String>(); 

    public static void main(String[] args) {
        if(args[0] == null){
            System.err.println("USAGE java d3.java data.dat");
            return;
        }

        Scanner input;
        try {
            File file = new File(args[0]);
            input = new Scanner(file);
        } catch(FileNotFoundException e){
            System.err.println("File at given path was not found");
            return;
        } catch (Exception e) {
            System.err.println("File could not be opened");
            return;
        }

        while(input.hasNextLine()){
            schem.add(input.nextLine());
        }
	for(int i = 0; i < schem.size(); i++){
		System.out.println(schem.get(i));
	}

        int sum = 0;
        for(int i = 0; i < schem.size(); i++){
            sum += lineSum(schem.get(i), i);
        }
	System.out.println(sum);
    }

    public static int lineSum(String line, int rowIndex){
	int i = 0;
	int lineSum = 0;
	String str = "";
	while (i < line.length()) {
		if(Character.isDigit(line.charAt(i))){
			int offset = 0;
				while (Character.isDigit(line.charAt(i + offset))) {

					if(i + offset < line.length()){
						System.out.println("Offset: " + offset + " I value: " + i + " Line length: " + line.length() + " Char at that is " + line.charAt(i + offset));
						str += line.charAt(i + offset);
						offset++;

					}
					if(i + offset >= line.length()){
						break;
					}
				}
				if(isPartNumber(i, i + offset, rowIndex)){
					System.out.printf("%s\n\n", str);
					lineSum += Integer.parseInt(str);
				}
				str = "";
				i = i + offset;
		}
		i++;
	}
	return lineSum;
    }

    public static boolean isPartNumber(int startIndex, int endIndex, int rowIndex){
	    boolean isPartNumber = false;
        for(int offset = 0; offset < endIndex - startIndex; offset++){
            if(isPart(startIndex + offset, rowIndex) == true){
                isPartNumber = true;
            }
        }
        return isPartNumber;
    }

    public static boolean isPart(int index, int rowIndex){
		int mapMaxWidth = schem.get(0).length();
		int mapMaxHeight = schem.size();
		boolean isPart = false;

		System.out.println("Checked Index: " + index);

		if(index - 1 >= 0) {
                    if(isPartChar(schem.get(rowIndex).charAt(index - 1))) isPart = true;
                }
                if(index + 1 < mapMaxWidth) {
                    if(isPartChar(schem.get(rowIndex).charAt(index + 1))) isPart = true;
                }
                if(rowIndex - 1 >= 0) {
                    if(isPartChar(schem.get(rowIndex - 1).charAt(index))) isPart = true; 
                }
                if(rowIndex + 1 < mapMaxHeight) {
                    if(isPartChar(schem.get(rowIndex + 1).charAt(index))) isPart = true;
                }

                // corner cases
                if(index - 1 >= 0 && rowIndex + 1 < mapMaxHeight){
                    if(isPartChar(schem.get(rowIndex + 1).charAt(index - 1))) isPart = true;
                }
                if(index + 1 < mapMaxWidth && rowIndex + 1 < mapMaxHeight){
                    if(isPartChar(schem.get(rowIndex + 1).charAt(index + 1))) isPart = true; 
                }
                if(index - 1 >= 0 && rowIndex - 1 >= 0){
                    if(isPartChar(schem.get(rowIndex - 1).charAt(index - 1))) isPart = true; 
                }
                if(index + 1 < mapMaxWidth && rowIndex - 1 >= 0){
                    if(isPartChar(schem.get(rowIndex - 1).charAt(index + 1))) isPart = true; 
                }
		return isPart;
    }

    public static boolean isPartChar(char c){
	boolean isPartChar = true;
	if(c == '.') isPartChar = false;
	else if(Character.isDigit(c)) isPartChar = false;
	return isPartChar;
    }
}
