using Lib2048;

public class C2048
{
    // Read from standard input:
    // 4 lines containing a 4 x 4 board of integers
    // 1 line containing the direction (0 = left, 1 = up etc)


    public static void Main(string[] args)
    {
        try
        {
            string[] lines = new string[4];
            for (int y = 0; y <= 3; y++) {
                lines[y] = Console.ReadLine() ?? "";
            }
            Lib.Directions direction = (Lib.Directions) int.Parse(Console.ReadLine()!);

            int[,] board = Lib.ParseBoardLines(lines);

            Lib.Play(ref board, direction);

            Console.Write(Lib.BoardToString(board));
        }
        catch (Exception e)
        {
            Console.WriteLine("Failed with exception: " + e);
        }
    }
}
