using System.Collections.Generic;
using System.Text;

namespace Lib2048
{
    public class Lib
    {
        public readonly struct Delta(int x, int y)
        {
            public int X { get; init; } = x;
            public int Y { get; init; } = y;

            public override string ToString() => string.Format("X:{0}, Y:{1}", X.ToString("+0;-#"), Y.ToString("+0;-#"));
        }
        public readonly struct Coords(int x, int y)
        {
            public int X { get; init; } = x;
            public int Y { get; init; } = y;

            public override string ToString() => $"(X:{X}, Y:{Y})";

            public Coords Project(int steps, Delta delta)
            {
                return new Coords(steps * delta.X + X, steps * delta.Y + Y);
            }

            public readonly bool IsValid()
            {
                return 0 <= X && X <= 3 && 0 <= Y && Y <= 3;
            }
        }

        public readonly struct Data(Lib.Coords start_coordinate, Lib.Delta group_delta, Lib.Delta pos_delta)
        {
            public Coords StartCoord { get; } = start_coordinate;

            public Delta GroupDelta { get; } = group_delta;
            public Delta PosDelta { get; } = pos_delta;

            public readonly Coords NextGroup( int steps )
            {
                return Next(StartCoord, steps, GroupDelta);
            }
            public readonly Coords NextGroup( Coords coord, int steps )
            {
                return Next(coord, steps, GroupDelta);
            }
            public readonly Coords NextPos( int steps )
            {
                return Next(StartCoord, steps, PosDelta);
            }

            public readonly Coords NextPos( Coords coord, int steps )
            {
                return Next(coord, steps, PosDelta);
            }

            private static Coords Next(Coords coord, int steps, Delta delta)
            {
                return coord.Project(steps, delta);
            }

            public static int GetBoardCell(int[,] board, Coords coordinate)
            {
                return board[coordinate.X, coordinate.Y];
            }
            public static void PutBoardCell(ref int[,] board, Coords coordinate, int value)
            {
                board[coordinate.X, coordinate.Y] = value;
            }
        }

        public enum Directions : int
        {
            Left = 0,
            Up = 1,
            Right = 2,
            Down = 3
        }

        public static readonly Dictionary<Directions, Data> _directionsData = new()
        {
            {Directions.Left, // Group is line, Pos is column, start upper right corner
             new Data(start_coordinate: new Coords(3, 3),
                      group_delta: new Delta(x: +0, y: -1),
                      pos_delta: new Delta(x: -1, y: +0))},
            {Directions.Up, // Group is column, Pos is line, start lower left corner
             new Data(start_coordinate: new Coords(0, 0),
                      group_delta: new Delta(x: +1, y: +0),
                      pos_delta: new Delta(x: +0, y: +1))},
            {Directions.Right, // Group is line, Pos is column, start upper left corner
             new Data(start_coordinate: new Coords(0, 3),
                      group_delta: new Delta(x: +0, y: -1),
                      pos_delta: new Delta(x: +1, y: +0))},
            {Directions.Down, // Group is column, Pos in line, start upper left corner
             new Data(start_coordinate: new Coords(0, 3),
                      group_delta: new Delta(x: +1, y: +0),
                      pos_delta: new Delta(x: +0, y: -1))}
        };

        public static Dictionary<Directions, Data> DirectionsData
        {
            get { return _directionsData; }
        }

        static Lib()
        {
        }

        public static int[,] ParseBoardLines(string[] boardLines)
        {
            int[,] board = new int[4, 4];
            for (int y = 0; y <= 3; y++) {
                string[] tokens = boardLines[y]!.Split();
                for (int x = 0; x <= 3; x++) {
                    int value = int.Parse(tokens[x]);
                    board[x, 3 - y] = value;
                }
            }
            return board;
        }

        // ToString for the 4 x 4 board
        public static string BoardToString(int[,] board)
        {
            StringBuilder sb = new();
            for (int y = 3; y >= 0; y--) {
                for (int x = 0; x <= 3; x++) {
                    if (x > 0) {
                        _ = sb.Append(' ');
                    }
                    _ = sb.Append(board[x, y]);
                }
                _ = sb.AppendLine("");
            }
            return sb.ToString();
        }

        public static void Play(ref int[,] board, Directions direction)
        {
            Compress(ref board, direction);
            Merge(ref board, direction);
            Compress(ref board, direction);
        }

        private static void Compress(ref int[,] board, Directions direction)
        {
            Data dd = DirectionsData[direction];
            for (int group_index = 0; group_index <= 3; group_index++)
            {
                Coords group_coord = dd.NextGroup( dd.StartCoord, group_index );

                // Do four times, but do reverse order of group to start from last pos
                for (int rev_pos_index = 3; rev_pos_index >= 0; rev_pos_index--)
                {
                    Coords pos_coord = dd.NextPos( group_coord, rev_pos_index );
                    int value = Data.GetBoardCell(board, pos_coord);
                    if (value > 0)
                    {
                        // Move it in direction as long as there are empty cells
                        int found = 0;
                        int k = 1;
                        Coords next_in_direction = dd.NextPos( pos_coord, k );
                        while (next_in_direction.IsValid() && Data.GetBoardCell( board, next_in_direction ) == 0)
                        {
                            found = k;
                            k++;
                            next_in_direction = dd.NextPos( pos_coord, k );
                        }
                        if (found > 0)
                        {
                            Data.PutBoardCell(ref board, dd.NextPos(pos_coord, found), value);
                            Data.PutBoardCell(ref board, pos_coord, 0);
                        }
                    }
                }
            }
        }

        private static void Merge(ref int[,] board, Directions direction)
        {
            Data dd = DirectionsData[direction];
            for (int group_index = 0; group_index <= 3; group_index++)
            {
                Coords group_coord = dd.NextGroup( dd.StartCoord, group_index );

                // From reverse side, check if a cell (except the last reverse one)
                // is non zero and has the same number as its neighbor in the direction.
                // If so merge them
                for (int rev_pos_index = 2; rev_pos_index >= 0; rev_pos_index--)
                {
                    Coords pos_coord = dd.NextPos( group_coord, rev_pos_index );
                    int value = Data.GetBoardCell(board, pos_coord);
                    Coords next_coord = dd.NextPos( group_coord, rev_pos_index + 1 );
                    int next_value = Data.GetBoardCell(board, next_coord);
                    if (value > 0 && value == next_value)
                    {
                        // Merge them
                        value *= 2;
                        Data.PutBoardCell(ref board, next_coord, value);
                        Data.PutBoardCell(ref board, pos_coord, 0);
                    }
                }
            }
        }
    }
}
