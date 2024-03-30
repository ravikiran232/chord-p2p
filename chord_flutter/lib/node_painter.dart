import 'package:flutter/material.dart';

Widget chordRingPainter({
  required Function(int) onNodeTap,
  required BuildContext context,
  required double radius,
  required Map nodePositions,
}) {
  List nodes = nodePositions.keys.toList();
  nodes.sort();
  final double screenWidth = MediaQuery.of(context).size.width;
  final double screenHeight = MediaQuery.of(context).size.height;
  final center = Offset(screenWidth / 2, screenHeight / 2);
  return Stack(
    children: [
      Positioned(
        left: center.dx - 200,
        top: center.dy - 200,
        child: Container(
          width: 400,
          height: 400,
          decoration: BoxDecoration(
            border: Border.all(color: Colors.white, width: 2),
            shape: BoxShape.circle,
          ),
        ),
      ),
      for (int node in nodes)
        Positioned(
          left: nodePositions[node].dx - 30,
          top: nodePositions[node].dy - 30,
          width: 60,
          height: 60,
          child: GestureDetector(
            onTap: () => onNodeTap(node),
            child: CircleAvatar(
              radius: 30,
              backgroundColor: Colors.blue,
              child: Text(
                node.toString(),
                style: const TextStyle(color: Colors.white),
                textAlign: TextAlign.center,
              ),
            ),
          ),
        ),
    ],
  );
}
