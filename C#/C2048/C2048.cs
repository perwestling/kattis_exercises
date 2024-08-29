namespace C2048NS;
public class C2048
{
    public string Something => "Hej";

    public static void Main(string[] args) {
        int [,] board =  new int [4,4];

        for (int i = 0; i < 3; i++) {
            string[] tokens = Console.ReadLine().Split();
            for (int j = 0; j < 3; j++) {
                board[i, j] = int.Parse(tokens[j]);
            }
        }
        int action = int.Parse(Console.ReadLine());

        Lib2048.Lib _lib = new Lib2048.Lib();

        _lib.play(board, action);

        // Print board
        for (int i = 0; i < 3; i++) {
            string[] tokens = Console.ReadLine().Split();
            for (int j = 0; j < 3; j++) {
                if (j > 0) {
                    Console.Write(" ");
                }
                Console.Write(board[i, j]);
            }
            Console.WriteLn("");
        }
    }
}
