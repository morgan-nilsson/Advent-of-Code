import java.util.Map;
import java.util.HashMap;

public class PointMap {

    private Map<String, Boolean> pointMap;

    public PointMap() {
        this.pointMap = new HashMap<>();
    }    

    public void setPoint(int x, int y){
        String key = x + "," + y;
        pointMap.put(key, true);
    }
    public boolean getPoint(int x, int y){
        String key = x + "," +  y;
        return pointMap.getOrDefault(key, false);
    }
    public int getSize(){
        return pointMap.size();
    }
}
