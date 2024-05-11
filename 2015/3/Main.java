import java.util.HashSet;
import java.util.Set;
import java.awt.Point;

class Main {
	public static void main(String[] args){
		String input = "";
		Set<Point> mySet = new HashSet<Point>();
		int x = 0;
		int y = 0;
		mySet.add(new Point(x,y));
		for(int i = 0; i < input.length(); i++){
			switch(input.charAt(i)){
				case '^': y++;
				break;
				case 'v': y--;
				break;
				case '>': x++;
				break;
				case '<': x--;
				break;
			}
			mySet.add(new Point(x,y));
		}
		System.out.println(mySet.size());

	}
}
