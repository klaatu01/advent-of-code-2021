import 'dart:io';
import 'dart:core';

class Cell {
  int n = 0;
  bool b = false;
  Cell(int number, bool marked){
    print(number);
    n = number;
    b = marked;
  }
}

class Board {
  int dims = 0;
  late List<List<Cell>> cells;

  Board(List<String> rows){
    dims = rows.length;
    cells = List.generate(dims, (i) => rows[i].trim().replaceAll("  ", " ").split(" ").map((n) => int.parse(n)).map((n) => Cell(n, false)).toList(), growable:false);
  }

  bool mark(int n){
    for (var i = 0; i < dims; i++) {
      for(var j = 0; j < dims; j++) {
        if (cells[i][j].n == n) {
          cells[i][j].b = true;
          return true;
        }
      }
    }
    return false;
  }

  bool checkForWin() {
    //check rows
    for (var i = 0; i < dims; i++){
      var mark = true;
      for (var j = 0; j < dims; j++){
        mark = cells[i][j].b && mark;
      }
      if (mark == true) {
        return mark;
      }
    }

    //check columns
    for (var i = 0; i < dims; i++){
      var mark = true;
      for (var j = 0; j < dims; j++){
        mark = cells[j][i].b && mark;
      }
      if (mark == true) {
        return mark;
      }
    }

    return false;
  }

  int sumUnchecked() {
    return cells
      .fold(0, (v, l) => v +
          l.fold(0, 
              (acc, c) => c.b ? acc : acc + c.n));
  }
}


void main(List<String> arguments) {
   var lines = File("input.txt").readAsStringSync().split("\n\n");
   List<int> numbers = lines.removeAt(0).split(",").map((n)=> int.parse(n)).toList();
   print(numbers);
   var p1 = part_01(numbers, lines);
   print(p1);
   var p2 = part_02(numbers, lines);
   print(p2);
}

int part_01(List<int> numbers,List<String> boardStrings) {
  print(boardStrings[0]);
  List<Board> boards = List.generate(boardStrings.length-1, (i) => Board(boardStrings[i].split("\n")), growable: false);
  for (final e in numbers) {
    for(final b in boards) {
      if(b.mark(e) && b.checkForWin()){
        return b.sumUnchecked() * e;
      }
    }
  }
  return 0; 
}

int part_02(List<int> numbers,List<String> boardStrings) {
  print(boardStrings[0]);
  List<Board> boards = List.generate(boardStrings.length-1, (i) => Board(boardStrings[i].split("\n")), growable: true);
  for (final e in numbers) {
    List<Board> toRemove = [];
    for(final b in boards) {
      if(b.mark(e) && b.checkForWin()){
        toRemove.add(b);
      }
    }
    for(final b in toRemove){
      boards.remove(b);
      if(boards.length == 0){
        return b.sumUnchecked() * e;
      }
    }
  }
  return 0;
}
