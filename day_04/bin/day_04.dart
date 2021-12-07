import 'package:pipe/pipe.dart';
import 'dart:io';
import 'dart:core';

class Cell {
  int n;
  bool b;

  Cell({this.n = 0, this.b = false});
}



void main(List<String> arguments) {
   var lines = File("input.txt").readAsStringSync().split("\n\n");
   lines | print;
}
