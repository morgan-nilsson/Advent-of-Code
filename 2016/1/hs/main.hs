getPuzzleInput :: String
getPuzzleInput = ""

parsePuzzleInput :: [(Char, Int)]
parsePuzzleInput = map parsePuzzleStringToTuple (splitPuzzleInput (getPuzzleInput))

splitPuzzleInput :: String -> [String]
splitPuzzleInput s = map (filter (/= ',')) (words s)

parsePuzzleStringToTuple :: String -> (Char, Int)
parsePuzzleStringToTuple (s:ss) = (s, read ss :: Int)

moveInPuzzle :: (Char, (Int, Int)) -> [(Char, Int)] -> [(Char, (Int, Int))]
moveInPuzzle _ [] = []
moveInPuzzle (face, (x, y)) (s:ss) =
    let (newFace, (dx, dy)) = translateFaceToAbs face s in
        (newFace, (x + dx, y + dy)) : moveInPuzzle (newFace, (x + dx, y + dy)) ss

translateFaceToAbs :: Char -> (Char, Int) -> (Char, (Int, Int))
translateFaceToAbs face (dir, mag)
    | (face == 'N' && dir == 'R') || (face == 'S' && dir == 'L') = ('E', (mag, 0))
    | (face == 'N' && dir == 'L') || (face == 'S' && dir == 'R') = ('W', (-mag, 0))
    | (face == 'E' && dir == 'L') || (face == 'W' && dir == 'R') = ('N', (0, mag))
    | (face == 'E' && dir == 'R') || (face == 'W' && dir == 'L') = ('S', (0, -mag))

distanceFromStart :: (Int, Int) -> Int
distanceFromStart (x, y) = abs x + abs y