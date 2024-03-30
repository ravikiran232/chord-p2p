import 'package:chord_flutter/painter.dart';
import 'package:flutter/material.dart';

void main() {
  runApp(const MaterialApp(
    debugShowCheckedModeBanner: false,
    color: Colors.black,
    home:ChordRing(initialNumNodes: 5,)));
}

