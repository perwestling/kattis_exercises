using Microsoft.VisualStudio.TestTools.UnitTesting;
using Lib2048;
using System.Net.Http.Headers;
using System.Diagnostics;
using System.Data.Common;

namespace Lib2048.Tests
{
    [TestClass]
    public class Lib2048Tests
    {
        private readonly String NL = System.Environment.NewLine;

        private void CompareBoards(string[] expectedBoardLines, int[,] actualBoard)
        {
            var expectedBoard = Lib.ParseBoardLines(expectedBoardLines);
            CollectionAssert.AreEqual(expectedBoard,
                                      actualBoard,
                                      @"" + NL +
                                      "Expected: " + NL + Lib.BoardToString(expectedBoard) + NL +
                                      "Actual  : " + NL + Lib.BoardToString(actualBoard) + NL);
 
        }

        public Lib2048Tests()
        {
        }

        [TestMethod]
        public void TestDataGiveCorrectNextGroupAndPos()
        {
            Lib.Data data = new(new Lib.Coords(0, 0), new Lib.Delta(1, 0), new Lib.Delta(0, 1));
            Assert.AreEqual(new Lib.Coords(2, 0), data.NextGroup(2));
            Assert.AreEqual(new Lib.Coords(0, 3), data.NextPos(3));
            Assert.AreEqual(new Lib.Coords(5, 0), data.NextGroup(new Lib.Coords(4, 0), 1));
            Assert.AreEqual(new Lib.Coords(4, 3), data.NextPos(new Lib.Coords(4, 0), 3));
        }

        [TestMethod]
        public void TestGetAndPutBoardCell()
        {
            int[,] board = {{1, 0, 0, 0},
                            {0, 2, 0, 0},
                            {0, 0, 3, 0},
                            {0, 0, 0, 4}};

            Assert.AreEqual(1, Lib.Data.GetBoardCell(board, new Lib.Coords(0, 0)));
            Assert.AreEqual(0, Lib.Data.GetBoardCell(board, new Lib.Coords(0, 1)));
            Assert.AreEqual(2, Lib.Data.GetBoardCell(board, new Lib.Coords(1, 1)));

            Lib.Data.PutBoardCell(ref board, new Lib.Coords(0, 0), 5);
            Assert.AreEqual(5, Lib.Data.GetBoardCell(board, new Lib.Coords(0, 0)));
            Assert.AreEqual(0, Lib.Data.GetBoardCell(board, new Lib.Coords(0, 1)));
            Assert.AreEqual(2, Lib.Data.GetBoardCell(board, new Lib.Coords(1, 1)));
        }

        [TestMethod]
        public void TestNextGroupDirection()
        {
            int[,] board = {{30, 31, 32, 33},
                            {20, 21, 22, 23},
                            {10, 11, 12, 13},
                            {00, 01, 02, 03}};

            Lib.Data dl = Lib.DirectionsData[Lib.Directions.Left];
            Assert.AreEqual(new Lib.Coords(2, 1), dl.NextGroup(new Lib.Coords(2, 2), 1));

            Lib.Data du = Lib.DirectionsData[Lib.Directions.Up];
            Assert.AreEqual(new Lib.Coords(3, 2), du.NextGroup(new Lib.Coords(2, 2), 1));

            Lib.Data dr = Lib.DirectionsData[Lib.Directions.Right];
            Assert.AreEqual(new Lib.Coords(2, 1), dr.NextGroup(new Lib.Coords(2, 2), 1));

            Lib.Data dd = Lib.DirectionsData[Lib.Directions.Down];
            Assert.AreEqual(new Lib.Coords(3, 2), dd.NextGroup(new Lib.Coords(2, 2), 1));
        }

        [TestMethod]
        public void TestNextPosDirection()
        {
            Lib.Data dl = Lib.DirectionsData[Lib.Directions.Left];
            Assert.AreEqual(new Lib.Coords(1, 2), dl.NextPos(new Lib.Coords(2, 2), 1));

            Lib.Data du = Lib.DirectionsData[Lib.Directions.Up];
            Assert.AreEqual(new Lib.Coords(2, 3), du.NextPos(new Lib.Coords(2, 2), 1));

            Lib.Data dr = Lib.DirectionsData[Lib.Directions.Right];
            Assert.AreEqual(new Lib.Coords(3, 2), dr.NextPos(new Lib.Coords(2, 2), 1));

            Lib.Data dd = Lib.DirectionsData[Lib.Directions.Down];
            Assert.AreEqual(new Lib.Coords(2, 1), dd.NextPos(new Lib.Coords(2, 2), 1));
        }

        [TestMethod]
        public void TestLeftShouldMoveThroughZeros()
        {
            string[] boardLines = ["1 0 0 0",
                                   "0 2 0 0",
                                   "0 0 3 0",
                                   "0 0 0 4"];

            var board = Lib.ParseBoardLines(boardLines);
            Lib.Play(ref board, Lib.Directions.Left);

            CompareBoards(["1 0 0 0",
                           "2 0 0 0",
                           "3 0 0 0",
                           "4 0 0 0"], board);
        }

        [TestMethod]
        public void TestUpShouldMoveThroughZeros()
        {
            string[] boardLines = ["1 0 0 0",
                                   "0 2 0 0",
                                   "0 0 3 0",
                                   "0 0 0 4"];

            var board = Lib.ParseBoardLines(boardLines);
            Lib.Play(ref board, Lib.Directions.Up);

            CompareBoards(["1 2 3 4",
                           "0 0 0 0",
                           "0 0 0 0",
                           "0 0 0 0"], board);
        }

        [TestMethod]
        public void TestRightShouldMoveThroughZeros()
        {
            string[] boardLines = ["1 0 0 0",
                                   "0 2 0 0",
                                   "0 0 3 0",
                                   "0 0 0 4"];

            var board = Lib.ParseBoardLines(boardLines);
            Lib.Play(ref board, Lib.Directions.Right);

            CompareBoards(["0 0 0 1",
                           "0 0 0 2",
                           "0 0 0 3",
                           "0 0 0 4"], board);
        }

        [TestMethod]
        public void TestDownShouldMoveThroughZeros()
        {
            string[] boardLines = ["1 0 0 0",
                                   "0 2 0 0",
                                   "0 0 3 0",
                                   "0 0 0 4"];

             var board = Lib.ParseBoardLines(boardLines);
           Lib.Play(ref board, Lib.Directions.Down);

            CompareBoards(["0 0 0 0",
                           "0 0 0 0",
                           "0 0 0 0",
                           "1 2 3 4"], board);
        }
    }
}