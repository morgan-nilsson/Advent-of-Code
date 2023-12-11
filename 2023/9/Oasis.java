import java.util.Scanner;
import java.util.Vector;
import java.io.File;
import java.io.FileNotFoundException;

public class Oasis {
    public static void main(String[] args) {
        if(args[0] == null){
            System.err.println("USAGE java Oasis.java data.dat");
            return;
        }
        Scanner input;
        try {
            File file = new File(args[0]);
            input = new Scanner(file);
        } catch (Exception e) {
            System.err.println("Error in reading From file");
            return;
        }

        String line;
        while(input.hasNextLine()){
            line = input.nextLine();
            predict(line);
        }


    }

    public static int predict(String line){
        int[] arr = parseToArr(line);
        int[] upperArray = arr;
        Vector<int[]> layers = new Vector<int[]>(); 
        while(!layerEqual(upperArray)){
            layers.add(upperArray);
            upperArray = nextLayer(upperArray);
        }
        return predictLayers(layers);

    }

    public static int predictLayers(Vector<int[]> layers){
        int change = layers.get(layers.size() - 1)[0];
        for(int i = 0; i < layers.size(); i++){
           change = predictLayer(layers.get(i), change); 
        }
        return change;
    }

    public static int predictLayer(int[] layer, int change){
        return layer[layer.length - 1] + change;
    }

    public static int[] parseToArr(String line){
        Vector<Integer> vec = new Vector<Integer>();
        Scanner input = new Scanner(line);
        while(input.hasNextInt()){
            vec.add(input.nextInt());
        }

        int[] ret = new int[vec.size()];
        for(int i = 0; i < vec.size(); i++){
            ret[i] = vec.get(i);
        }
        return ret;
    }

    public static int[] nextLayer(int[] layer){
        int[] newLayer = new int[layer.length - 1];
        for(int i = 0; i < layer.length - 1; i++){   
            newLayer[i] = layer[i + 1] - layer[i];
        }
        return newLayer;
    }

    public static boolean layerEqual(int[] layer){
        if(layer == null){
            return false;
        }
        int value = layer[0];
        for(int i = 0; i < layer.length; i++){
            if(layer[i] != value){
                return true;
            }
        }
        return false;
    }
}
