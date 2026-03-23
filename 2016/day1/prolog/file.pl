direction(north).
direction(east).
direction(south).
direction(west).

% turn_right(+Current -New)
turn_right(north, east).
turn_right(east, south).
turn_right(south, west).
turn_right(west, north).

% turn_left(+Current -New)
turn_left(north, west).
turn_left(west, south).
turn_left(south, east).
turn_left(east, north).

% move(+Dir, +Steps, +X, +Y, -NewX, -NewY)
move(north, Steps, X, Y, X, NewY) :-
    NewY is Y + Steps.
move(east, Steps, X, Y, NewX, Y) :-
    NewX is X + Steps.
move(south, Steps, X, Y, X, NewY) :-
    NewY is Y - Steps.
move(west, Steps, X, Y, NewX, Y) :-
    NewX is X - Steps.

% step(+Instr, +Dir, +X, +Y, -NewDir, -NewX, -NewY)
step(r(N), Dir, X, Y, NewDir, NewX, NewY) :-
    turn_right(Dir, NewDir),
    move(NewDir, N, X, Y, NewX, NewY).

step(l(N), Dir, X, Y, NewDir, NewX, NewY) :-
    turn_left(Dir, NewDir),
    move(NewDir, N, X, Y, NewX, NewY).

% walk(+Instrs, +Dir, +X, +Y, -NewX, -NewY)
walk([], _, X, Y, X, Y).

walk([Instr|Rest], Dir, X, Y, NewX, NewY) :-
    step(Instr, Dir, X, Y, NewDir, NewX1, NewY1),
    walk(Rest, NewDir, NewX1, NewY1, NewX, NewY).

% manhattan_distance_origin(+X, +Y, -Distance)
manhattan_distance_origin(X, Y, Distance) :-
    Distance is abs(X) + abs(Y).

solve(Instrs, Distance) :-
    walk(Instrs, north, 0, 0, NewX, NewY),
    manhattan_distance_origin(NewX, NewY, Distance).